use clap::{Arg, ArgAction, Command};
use frankly_fw_update_cli::francor::franklyboot::{
    com::{
        can::CANInterface, serial::SerialInterface, sim::SIMInterface, ComConnParams, ComInterface,
        ComMode,
    },
    device::Device,
    firmware::hex_file::HexFile,
    Error,
};

const SIM_NODE_LST: [u8; 4] = [1, 3, 31, 8];

pub enum InterfaceType {
    Sim,
    Serial,
    CAN,
    Ethernet,
}

// Convert from string to interface type
impl InterfaceType {
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "sim" => Ok(InterfaceType::Sim),
            "serial" => Ok(InterfaceType::Serial),
            "can" => Ok(InterfaceType::CAN),
            "ethernet" => Ok(InterfaceType::Ethernet),
            _ => Err(Error::Error(format!("Unknown interface type {}", s))),
        }
    }
}

pub fn connect_device<I>(
    conn_params: &ComConnParams,
    node_id: Option<u8>,
) -> Result<Device<I>, Error>
where
    I: ComInterface,
{
    let mut interface = I::create()?;
    interface.open(conn_params)?;
    if node_id.is_some() {
        interface.set_mode(ComMode::Specific(node_id.unwrap()))?;
    }

    let mut device = Device::new(interface);
    device.init()?;

    Ok(device)
}

pub fn search_for_devices<I>(conn_params: &ComConnParams)
where
    I: ComInterface,
{
    if I::is_network() {
        let node_lst = {
            let mut interface = I::create().unwrap();
            interface.open(conn_params).unwrap();
            interface.scan_network().unwrap()
        };

        for node in node_lst {
            let device = connect_device::<I>(conn_params, Some(node)).unwrap();
            println!("Device found[{:3}]: {}", node, device);
        }
    } else {
        let device = connect_device::<I>(conn_params, None).unwrap();
        println!("Device found: {}", device);
    }
}

pub fn erase_device<I>(conn_params: &ComConnParams, node_id: u8)
where
    I: ComInterface,
{
    let node_id = {
        if I::is_network() {
            Some(node_id)
        } else {
            None
        }
    };

    let mut device = connect_device::<I>(conn_params, node_id).unwrap();
    println!("Device: {}", device);
    device.erase().unwrap();
}

pub fn flash_device<I>(conn_params: &ComConnParams, node_id: u8, hex_file_path: &str)
where
    I: ComInterface,
{
    let node_id = {
        if I::is_network() {
            Some(node_id)
        } else {
            None
        }
    };

    let mut device = connect_device::<I>(conn_params, node_id).unwrap();
    println!("Device: {}", device);

    let hex_file = HexFile::from_file(hex_file_path).unwrap();
    device.flash(&hex_file).unwrap();
}

fn create_sim_devices() {
    let node_lst = SIM_NODE_LST.to_vec();
    SIMInterface::config_nodes(node_lst).unwrap();
}

fn main() {
    create_sim_devices();

    let type_arg = Arg::new("type")
        .short('t')
        .long("type")
        .help("Interface type \"sim\", \"serial\", \"can\"")
        .required(true)
        .action(ArgAction::Set)
        .num_args(1);

    let interface_arg = Arg::new("interface")
        .short('i')
        .long("interface")
        .help("Interface name \"can0\", \"ttyACM0\", \"sim\"")
        .required(true)
        .action(ArgAction::Set)
        .num_args(1);

    let node_arg = Arg::new("node")
        .short('n')
        .long("node")
        .help("Node ID: 0, 1, ..")
        .value_parser(clap::value_parser!(u8).range(0..))
        .required(true)
        .action(ArgAction::Set)
        .num_args(1);

    let matches = Command::new("frankly-fw-update")
        .version("0.1.0")
        .author("Martin Bauernschmitt - FRANCOR e.V.")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("search")
                .short_flag('s')
                .long_flag("search")
                .about("Search for connected devices on specified network")
                .arg(type_arg.clone())
                .arg(interface_arg.clone()),
        )
        .subcommand(
            Command::new("erase")
                .short_flag('e')
                .long_flag("erase")
                .about("Erases the application from the device")
                .arg(type_arg.clone())
                .arg(interface_arg.clone())
                .arg(node_arg.clone()),
        )
        .subcommand(
            Command::new("flash")
                .short_flag('f')
                .long_flag("flash")
                .about("Flashes the application to the device")
                .arg(type_arg.clone())
                .arg(interface_arg.clone())
                .arg(node_arg.clone())
                .arg(
                    Arg::new("hex-file")
                        .long("hex-file")
                        .help("Path to hex file")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        .get_matches();

    println!("Frankly Firmware Update CLI (c) 2021 Martin Bauernschmitt - FRANCOR e.V.");

    match matches.subcommand() {
        Some(("search", search_matches)) => {
            let interface_type_str = search_matches.get_one::<String>("type").unwrap();
            let interface_type = InterfaceType::from_str(&interface_type_str).unwrap();
            let interface_name = search_matches.get_one::<String>("interface").unwrap();

            match interface_type {
                InterfaceType::Serial => search_for_devices::<SerialInterface>(
                    &ComConnParams::for_serial_conn(interface_name, 115200),
                ),
                InterfaceType::CAN => {
                    search_for_devices::<CANInterface>(&ComConnParams::for_can_conn(interface_name))
                }
                InterfaceType::Ethernet => {
                    println!("Ethernet not supported yet");
                }
                InterfaceType::Sim => {
                    search_for_devices::<SIMInterface>(&ComConnParams::for_sim_device())
                }
            }
        }
        Some(("erase", erase_matches)) => {
            let interface_type_str = erase_matches.get_one::<String>("type").unwrap();
            let interface_type = InterfaceType::from_str(interface_type_str).unwrap();
            let interface_name = erase_matches.get_one::<String>("interface").unwrap();
            let node_id = *erase_matches.get_one::<u8>("node").unwrap();

            match interface_type {
                InterfaceType::Serial => erase_device::<SerialInterface>(
                    &ComConnParams::for_serial_conn(interface_name, 115200),
                    node_id,
                ),
                InterfaceType::CAN => erase_device::<CANInterface>(
                    &ComConnParams::for_can_conn(interface_name),
                    node_id,
                ),
                InterfaceType::Ethernet => println!("Ethernet not supported yet"),
                InterfaceType::Sim => {
                    erase_device::<SIMInterface>(&ComConnParams::for_sim_device(), node_id)
                }
            }
        }
        Some(("flash", flash_matches)) => {
            let interface_type_str = flash_matches.get_one::<String>("type").unwrap();
            let interface_type = InterfaceType::from_str(interface_type_str).unwrap();
            let interface_name = flash_matches.get_one::<String>("interface").unwrap();
            let node_id = *flash_matches.get_one::<u8>("node").unwrap();
            let hex_file_path = flash_matches.get_one::<String>("hex-file").unwrap();

            match interface_type {
                InterfaceType::Serial => flash_device::<SerialInterface>(
                    &ComConnParams::for_serial_conn(interface_name, 115200),
                    node_id,
                    &hex_file_path,
                ),
                InterfaceType::CAN => flash_device::<CANInterface>(
                    &ComConnParams::for_can_conn(interface_name),
                    node_id,
                    &hex_file_path,
                ),
                InterfaceType::Ethernet => println!("Ethernet not supported yet"),
                InterfaceType::Sim => flash_device::<SIMInterface>(
                    &ComConnParams::for_sim_device(),
                    node_id,
                    &hex_file_path,
                ),
            }
        }
        _ => {
            println!("Unknown command");
        }
    }
}

// Tests ------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sim_net_ping() {
        let node_lst = vec![1, 3, 31, 8];
        SIMInterface::config_nodes(node_lst).unwrap();
    }
}
