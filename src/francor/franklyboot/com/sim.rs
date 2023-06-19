use crate::francor::franklyboot::{
    com::{
        msg::{Msg, RequestType},
        ComInterface, ComMode,
    },
    utils::sim_api,
    Error,
};

// SIM Interface ----------------------------------------------------------------------------------

pub struct SIMInterface {
    mode: ComMode,
}

impl SIMInterface {
    ///
    /// Resets the network and adds the given nodes to the simulated network
    ///
    pub fn config_nodes(node_lst: Vec<u8>) -> Result<(), Error> {
        sim_api::reset();

        for node in node_lst {
            match sim_api::add_device(node) {
                Ok(_) => {}
                Err(e) => {
                    return Err(Error::Error(e.to_string()));
                }
            };
        }

        Ok(())
    }

    ///
    /// Pings the network to search for nodes and returns a list of found nodes
    ///
    pub fn ping_network() -> Result<Vec<u8>, Error> {
        // Send ping
        let ping_request = Msg::new_std_request(RequestType::Ping);
        sim_api::send_broadcast_msg(&ping_request.to_raw_data_array());

        // Receive until no new response
        let mut node_id_lst = Vec::new();
        loop {
            let response = sim_api::get_broadcast_response_msg();

            if response.is_none() {
                break;
            }

            let (node_id, response_msg_raw) = response.unwrap();
            let response_msg = Msg::from_raw_data_array(&response_msg_raw);

            if ping_request.is_response_ok(&response_msg).is_ok() {
                node_id_lst.push(node_id);
            }
        }

        Ok(node_id_lst)
    }

    ///
    /// Opens sim interface
    ///
    /// This function opens the simulation interface. Port name is ignored.
    ///
    /// # Arguments
    ///
    /// * `port_name` - Port name of the interface - Ignored
    pub fn open(_port_name: &str) -> Result<SIMInterface, Error> {
        Ok(SIMInterface {
            mode: ComMode::Broadcast,
        })
    }
}

impl ComInterface for SIMInterface {
    fn set_mode(&mut self, mode: ComMode) -> Result<(), Error> {
        self.mode = mode;
        Ok(())
    }

    fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), Error> {
        Ok(())
    }

    fn get_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(0)
    }

    fn send(&mut self, msg: &Msg) -> Result<(), Error> {
        match self.mode {
            ComMode::Specific(node_id) => {
                sim_api::send_node_msg(node_id, &msg.to_raw_data_array());
            }
            _ => {}
        }

        Ok(())
    }

    fn recv(&mut self) -> Result<Msg, Error> {
        match self.mode {
            ComMode::Specific(node_id) => match sim_api::get_node_response_msg(node_id) {
                Some(msg_raw) => {
                    let response = Msg::from_raw_data_array(&msg_raw);
                    return Ok(response);
                }
                None => {
                    return Err(Error::ComNoResponse);
                }
            },
            _ => {}
        }

        return Err(Error::Error("Mode not supported!".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_network() {
        let node_lst = vec![1, 20, 3, 52];
        SIMInterface::config_nodes(node_lst.clone()).unwrap();

        let node_lst_found = SIMInterface::ping_network().unwrap();

        assert_eq!(node_lst, node_lst_found);
    }
}
