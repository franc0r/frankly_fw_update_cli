
// Node ID ----------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NodeID {
    Broadcast,
    Specific(u8),
}

// Request Type -----------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub enum RequestType {
    ReqPing, //< Ping device | Response is bootloader version
    ReqResetDevice, //< Resets the device (hardware reset)
    ReqStartApp, //< Start application and exit bootloader

    /* Device information */
    ReqDevInfoBootloaderVersion, //< Reads the bootloader version
    ReqDevInfoBootloaderCRC, //< Calculates the CRC of the bootloader flash area
    ReqDevInfoVID, //< Reads the vendor id
    ReqDevInfoPID, //< Reads the product id
    ReqDevInfoPRD, //< Reads the production date
    ReqDevInfoUID, //< Reads the device unique ID

    /* Flash information */
    ReqFlashInfoStartAddr, //< Get the start address of the flash area
    ReqFlashInfoPageSize, //< Get the size in bytes of a page
    ReqFlashInfoNumPages, //< Get the number of pages (including bootloader area)

    /* App Information */
    ReqAppInfoPageIdx, //< Get the page idx of app area in flash
    ReqAppInfoCRCCalc, //< Get the calculate CRC over app flash area
    ReqAppInfoCRCStrd, //< Get the stored CRC value used for safe startup

    /* Flash Read commands */
    ReqFlashReadWord, //< Reads a word from the flash

    /* Page Buffer Commands */
    ReqPageBufferClear, //< Clears the page buffer (RAM)
    ReqPageBufferReadWord, //< Reads a word to the page buffer (RAM)
    ReqPageBufferWriteWord, //< Writes a word to the page buffer (RAM)
    ReqPageBufferCalcCRC, //< Calculates the CRC over the page buffer
    ReqPageBufferWriteToFlash, //< Write the page buffer to the desired flash page

    /* Flash Write Commands*/
    ReqFlashWriteErasePage, //< Erases an flash page
    ReqFlashWriteAppCRC, //< Writes the CRC of the app to the flash
}

impl RequestType {
    pub fn from_u16(value: u16) -> RequestType {
        match value {
            0x0001 => RequestType::ReqPing,
            0x0011 => RequestType::ReqResetDevice,
            0x0012 => RequestType::ReqStartApp,
            0x0101 => RequestType::ReqDevInfoBootloaderVersion,
            0x0102 => RequestType::ReqDevInfoBootloaderCRC,
            0x0103 => RequestType::ReqDevInfoVID,
            0x0104 => RequestType::ReqDevInfoPID,
            0x0105 => RequestType::ReqDevInfoPRD,
            0x0106 => RequestType::ReqDevInfoUID,
            0x0201 => RequestType::ReqFlashInfoStartAddr,
            0x0202 => RequestType::ReqFlashInfoPageSize,
            0x0203 => RequestType::ReqFlashInfoNumPages,
            0x0301 => RequestType::ReqAppInfoPageIdx,
            0x0302 => RequestType::ReqAppInfoCRCCalc,
            0x0303 => RequestType::ReqAppInfoCRCStrd,
            0x0401 => RequestType::ReqFlashReadWord,
            0x1001 => RequestType::ReqPageBufferClear,
            0x1002 => RequestType::ReqPageBufferReadWord,
            0x1003 => RequestType::ReqPageBufferWriteWord,
            0x1004 => RequestType::ReqPageBufferCalcCRC,
            0x1005 => RequestType::ReqPageBufferWriteToFlash,
            0x1101 => RequestType::ReqFlashWriteErasePage,
            0x1102 => RequestType::ReqFlashWriteAppCRC,
            _ => panic!("Unknown request type: {}", value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            RequestType::ReqPing => 0x0001,
            RequestType::ReqResetDevice => 0x0011,
            RequestType::ReqStartApp => 0x0012,
            RequestType::ReqDevInfoBootloaderVersion => 0x0101,
            RequestType::ReqDevInfoBootloaderCRC => 0x0102,
            RequestType::ReqDevInfoVID => 0x0103,
            RequestType::ReqDevInfoPID => 0x0104,
            RequestType::ReqDevInfoPRD => 0x0105,
            RequestType::ReqDevInfoUID => 0x0106,
            RequestType::ReqFlashInfoStartAddr => 0x0201,
            RequestType::ReqFlashInfoPageSize => 0x0202,
            RequestType::ReqFlashInfoNumPages => 0x0203,
            RequestType::ReqAppInfoPageIdx => 0x0301,
            RequestType::ReqAppInfoCRCCalc => 0x0302,
            RequestType::ReqAppInfoCRCStrd => 0x0303,
            RequestType::ReqFlashReadWord => 0x0401,
            RequestType::ReqPageBufferClear => 0x1001,
            RequestType::ReqPageBufferReadWord => 0x1002,
            RequestType::ReqPageBufferWriteWord => 0x1003,
            RequestType::ReqPageBufferCalcCRC => 0x1004,
            RequestType::ReqPageBufferWriteToFlash => 0x1005,
            RequestType::ReqFlashWriteErasePage => 0x1101,
            RequestType::ReqFlashWriteAppCRC => 0x1102,
        }
    }
}


// Response types ---------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ResponseType {   
    RespNone, // Unused / ignored
    RespAck, // Acknowledge

    /* Errors */
    RespErr, // General error
    RespUnknownReq, // Unknow request type
    RespErrNotSupported, // Error, command known but not supported
    RespErrCRCInvld, // Error, CRC check failed
    RespAckPageFull, // Acknowledge and info that page buffer is full
    RespErrPageFull, // Error, word not writable page buffer is full
    RespErrInvldArg, // Error, invalid argument (out of range, ...)
}

impl ResponseType {
    pub fn to_u8(&self) -> u8 {
        match self {
            ResponseType::RespNone => 0x00,
            ResponseType::RespAck => 0x01,
            ResponseType::RespErr => 0xFE,
            ResponseType::RespUnknownReq => 0xFD,
            ResponseType::RespErrNotSupported => 0xFC,
            ResponseType::RespErrCRCInvld => 0xFB,
            ResponseType::RespAckPageFull => 0xFA,
            ResponseType::RespErrPageFull => 0xF9,
            ResponseType::RespErrInvldArg => 0xF8,
        }
    }

    pub fn from_u8(value: u8) -> ResponseType {
        match value {
            0x00 => ResponseType::RespNone,
            0x01 => ResponseType::RespAck,
            0xFE => ResponseType::RespErr,
            0xFD => ResponseType::RespUnknownReq,
            0xFC => ResponseType::RespErrNotSupported,
            0xFB => ResponseType::RespErrCRCInvld,
            0xFA => ResponseType::RespAckPageFull,
            0xF9 => ResponseType::RespErrPageFull,
            0xF8 => ResponseType::RespErrInvldArg,
            _ => panic!("Unknown response type: {}", value),
        }
    }
}

// Message ----------------------------------------------------------------------------------------

pub type MsgData = [u8; 4];

#[derive(Debug)]
pub struct Msg {
    pub node_id: NodeID,
    pub request: RequestType,
    pub response: ResponseType,
    pub packet_id: u8,
    pub data: MsgData,
}

// Tests ------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_type_convert_to_u16() {
        assert_eq!(RequestType::ReqPing.to_u16(), 0x0001);
        assert_eq!(RequestType::ReqResetDevice.to_u16(), 0x0011);
        assert_eq!(RequestType::ReqStartApp.to_u16(), 0x0012);
        assert_eq!(RequestType::ReqDevInfoBootloaderVersion.to_u16(), 0x0101);
        assert_eq!(RequestType::ReqDevInfoBootloaderCRC.to_u16(), 0x0102);
        assert_eq!(RequestType::ReqDevInfoVID.to_u16(), 0x0103);
        assert_eq!(RequestType::ReqDevInfoPID.to_u16(), 0x0104);
        assert_eq!(RequestType::ReqDevInfoPRD.to_u16(), 0x0105);
        assert_eq!(RequestType::ReqDevInfoUID.to_u16(), 0x0106);
        assert_eq!(RequestType::ReqFlashInfoStartAddr.to_u16(), 0x0201);
        assert_eq!(RequestType::ReqFlashInfoPageSize.to_u16(), 0x0202);
        assert_eq!(RequestType::ReqFlashInfoNumPages.to_u16(), 0x0203);
        assert_eq!(RequestType::ReqAppInfoPageIdx.to_u16(), 0x0301);
        assert_eq!(RequestType::ReqAppInfoCRCCalc.to_u16(), 0x0302);
        assert_eq!(RequestType::ReqAppInfoCRCStrd.to_u16(), 0x0303);
        assert_eq!(RequestType::ReqFlashReadWord.to_u16(), 0x0401);
        assert_eq!(RequestType::ReqPageBufferClear.to_u16(), 0x1001);
        assert_eq!(RequestType::ReqPageBufferReadWord.to_u16(), 0x1002);
        assert_eq!(RequestType::ReqPageBufferWriteWord.to_u16(), 0x1003);
        assert_eq!(RequestType::ReqPageBufferCalcCRC.to_u16(), 0x1004);
        assert_eq!(RequestType::ReqPageBufferWriteToFlash.to_u16(), 0x1005);
        assert_eq!(RequestType::ReqFlashWriteErasePage.to_u16(), 0x1101);
        assert_eq!(RequestType::ReqFlashWriteAppCRC.to_u16(), 0x1102);
    }

    #[test]
    fn request_type_convert_from_u16() {
        assert_eq!(RequestType::from_u16(0x0001), RequestType::ReqPing);
        assert_eq!(RequestType::from_u16(0x0011), RequestType::ReqResetDevice);
        assert_eq!(RequestType::from_u16(0x0012), RequestType::ReqStartApp);
        assert_eq!(RequestType::from_u16(0x0101), RequestType::ReqDevInfoBootloaderVersion);
        assert_eq!(RequestType::from_u16(0x0102), RequestType::ReqDevInfoBootloaderCRC);
        assert_eq!(RequestType::from_u16(0x0103), RequestType::ReqDevInfoVID);
        assert_eq!(RequestType::from_u16(0x0104), RequestType::ReqDevInfoPID);
        assert_eq!(RequestType::from_u16(0x0105), RequestType::ReqDevInfoPRD);
        assert_eq!(RequestType::from_u16(0x0106), RequestType::ReqDevInfoUID);
        assert_eq!(RequestType::from_u16(0x0201), RequestType::ReqFlashInfoStartAddr);
        assert_eq!(RequestType::from_u16(0x0202), RequestType::ReqFlashInfoPageSize);
        assert_eq!(RequestType::from_u16(0x0203), RequestType::ReqFlashInfoNumPages);
        assert_eq!(RequestType::from_u16(0x0301), RequestType::ReqAppInfoPageIdx);
        assert_eq!(RequestType::from_u16(0x0302), RequestType::ReqAppInfoCRCCalc);
        assert_eq!(RequestType::from_u16(0x0303), RequestType::ReqAppInfoCRCStrd);
        assert_eq!(RequestType::from_u16(0x0401), RequestType::ReqFlashReadWord);
        assert_eq!(RequestType::from_u16(0x1001), RequestType::ReqPageBufferClear);
        assert_eq!(RequestType::from_u16(0x1002), RequestType::ReqPageBufferReadWord);
        assert_eq!(RequestType::from_u16(0x1003), RequestType::ReqPageBufferWriteWord);
        assert_eq!(RequestType::from_u16(0x1004), RequestType::ReqPageBufferCalcCRC);
        assert_eq!(RequestType::from_u16(0x1005), RequestType::ReqPageBufferWriteToFlash);
        assert_eq!(RequestType::from_u16(0x1101), RequestType::ReqFlashWriteErasePage);
        assert_eq!(RequestType::from_u16(0x1102), RequestType::ReqFlashWriteAppCRC);
    }

    #[test]
    fn response_convert_to_u8() {
        assert_eq!(ResponseType::RespNone.to_u8(), 0x00);
        assert_eq!(ResponseType::RespAck.to_u8(), 0x01);
        assert_eq!(ResponseType::RespErr.to_u8(), 0xFE);
        assert_eq!(ResponseType::RespUnknownReq.to_u8(), 0xFD);
        assert_eq!(ResponseType::RespErrNotSupported.to_u8(), 0xFC);
        assert_eq!(ResponseType::RespErrCRCInvld.to_u8(), 0xFB);
        assert_eq!(ResponseType::RespAckPageFull.to_u8(), 0xFA);
        assert_eq!(ResponseType::RespErrPageFull.to_u8(), 0xF9);
        assert_eq!(ResponseType::RespErrInvldArg.to_u8(), 0xF8);
    }

    #[test]
    fn response_convert_from_u8() {
        assert_eq!(ResponseType::from_u8(0x00), ResponseType::RespNone);
        assert_eq!(ResponseType::from_u8(0x01), ResponseType::RespAck);
        assert_eq!(ResponseType::from_u8(0xFE), ResponseType::RespErr);
        assert_eq!(ResponseType::from_u8(0xFD), ResponseType::RespUnknownReq);
        assert_eq!(ResponseType::from_u8(0xFC), ResponseType::RespErrNotSupported);
        assert_eq!(ResponseType::from_u8(0xFB), ResponseType::RespErrCRCInvld);
        assert_eq!(ResponseType::from_u8(0xFA), ResponseType::RespAckPageFull);
        assert_eq!(ResponseType::from_u8(0xF9), ResponseType::RespErrPageFull);
        assert_eq!(ResponseType::from_u8(0xF8), ResponseType::RespErrInvldArg);
    }
}