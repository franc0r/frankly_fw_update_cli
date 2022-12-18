use crate::francor::franklyboot::com::{
    msg::{Msg, RequestType, ResponseType},
    ComError, ComInterface,
};

// Device Entry -----------------------------------------------------------------------------------

pub struct DeviceEntry {
    name: String,
    request_type: RequestType,
    value: Option<u32>,
}

impl DeviceEntry {
    pub fn new(name: &str, request_type: RequestType) -> Self {
        DeviceEntry {
            name: name.to_string(),
            request_type: request_type,
            value: None,
        }
    }

    pub fn read_from_device<T: ComInterface>(
        &mut self,
        interface: &mut T,
    ) -> Result<bool, ComError> {
        // Send request to device
        let request = Msg::new_std_request(self.request_type);
        interface.send(&request)?;

        // Wait for response
        let response = interface.recv()?;
        match response {
            Some(msg) => {
                // Check if response is valid
                let request_valid = msg.get_request() == request.get_request();
                let response_valid = msg.get_response() == ResponseType::RespAck;
                let msg_valid = request_valid && response_valid;

                if msg_valid {
                    self.value = Some(msg.get_data().to_word());
                    return Ok(true);
                } else {
                    self.value = None;
                    return Err(ComError::MsgError(format!(
                        "Error Reading \"{:?}\"\nDevice response is invalid! \
                         TX: Request {:?}\n\tRX: RequestType {:?} ResponseType {:?}",
                        self.name,
                        request.get_request(),
                        msg.get_request(),
                        msg.get_response()
                    )));
                }
            }
            None => {
                return Ok(false);
            }
        }
    }

    pub fn get_value(&self) -> Option<u32> {
        self.value
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_request_type(&self) -> RequestType {
        self.request_type
    }
}

/*
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

pub struct DeviceInfo {
    bootloader_version: Version,
    bootloader_crc: u32,
    vendor_id: u32,
    product_id: u32,
    production_date: u32,
    unique_id: u32,
}

pub struct FlashInfo {
    start_address: u32,
    page_size: u32,
    num_pages: u32,
}
*/

// Tests ------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::francor::franklyboot::com::{msg::MsgData, ComSimulator};

    #[test]
    fn device_entry_new() {
        let entry = DeviceEntry::new(
            "Bootloader Version",
            RequestType::ReqDevInfoBootloaderVersion,
        );

        assert_eq!(entry.name, "Bootloader Version");
        assert_eq!(entry.request_type, RequestType::ReqDevInfoBootloaderVersion);
        assert_eq!(entry.value, None);
    }

    #[test]
    fn device_entry_read() {
        let mut entry = DeviceEntry::new(
            "Bootloader Version",
            RequestType::ReqDevInfoBootloaderVersion,
        );

        let mut com = ComSimulator::new();
        com.add_response(Msg::new(
            RequestType::ReqDevInfoBootloaderVersion,
            ResponseType::RespAck,
            0,
            &MsgData::from_word(0x01020304),
        ));

        let result = entry.read_from_device(&mut com);
        assert_eq!(result, Ok(true));
        assert_eq!(entry.value, Some(0x01020304));
    }

    #[test]
    fn device_entry_read_send_error() {
        let mut entry = DeviceEntry::new(
            "Bootloader Version",
            RequestType::ReqDevInfoBootloaderVersion,
        );

        let mut com = ComSimulator::new();
        com.add_response(Msg::new(
            RequestType::ReqDevInfoBootloaderVersion,
            ResponseType::RespAck,
            0,
            &MsgData::from_word(0x01020304),
        ));
        com.set_send_error(ComError::Error("Send error".to_string()));

        let result = entry.read_from_device(&mut com);
        assert_eq!(result, Err(ComError::Error("Send error".to_string())));
        assert_eq!(entry.value, None);
    }

    #[test]
    fn device_entry_read_recv_error() {
        let mut entry = DeviceEntry::new(
            "Bootloader Version",
            RequestType::ReqDevInfoBootloaderVersion,
        );

        let mut com = ComSimulator::new();
        com.add_response(Msg::new(
            RequestType::ReqDevInfoBootloaderVersion,
            ResponseType::RespAck,
            0,
            &MsgData::from_word(0x01020304),
        ));
        com.set_recv_error(ComError::Error("Recv error".to_string()));

        let result = entry.read_from_device(&mut com);
        assert_eq!(result, Err(ComError::Error("Recv error".to_string())));
        assert_eq!(entry.value, None);
    }

    #[test]
    fn device_entry_read_recv_timeout() {
        let mut entry = DeviceEntry::new(
            "Bootloader Version",
            RequestType::ReqDevInfoBootloaderVersion,
        );

        let mut com = ComSimulator::new();
        com.add_response(Msg::new(
            RequestType::ReqDevInfoBootloaderVersion,
            ResponseType::RespAck,
            0,
            &MsgData::from_word(0x01020304),
        ));
        com.set_recv_timeout_error();

        let result = entry.read_from_device(&mut com);
        assert_eq!(result, Ok(false));
        assert_eq!(entry.value, None);
    }
}
