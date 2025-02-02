pub trait NmeaBuff {
    /// Used for NMEA message serialization from bottom to top structures
    fn as_string(&self) -> String;

    /// Used for parsing (deserialization) NMEA message from top to bottom structures
    fn parse_payload(msg: String) -> Self;
}

#[allow(dead_code)]
enum DeviceId {
    None,
    GPS,
    EchoSounder,
    Radar,
}

impl DeviceId {
    pub fn id_string(&self) -> Result<String, String> {
        use crate::devices::DeviceId::*;
        match self {
            GPS => Ok("GP".to_string()),
            EchoSounder => Ok("ES".to_string()),
            Radar => Ok("RD".to_string()),
            None => Err("Invalid input".to_string()),
        }
    }
}

#[allow(dead_code)]
struct DeviceMessage<T> {
    device_id: DeviceId,
    message_body: T,
}
// TODO: rework functions to return Result
#[allow(dead_code)]
impl<T> DeviceMessage<T>
where
    T: NmeaBuff,
{
    /// Create a new NMEA message
    pub fn new(id: DeviceId, payload: T) -> DeviceMessage<T> {
        DeviceMessage {
            device_id: id,
            message_body: payload,
        }
    }

    /// Create an empty NMEA message
    pub fn empty(payload: T) -> DeviceMessage<T> {
        DeviceMessage {
            device_id: DeviceId::None,
            message_body: payload,
        }
    }

    /// Calculate the checksum of the NMEA message
    fn checksum(msg: String) -> u8 {
        let sum = msg[1..msg.len() - 1].to_string().into_bytes().iter().fold(0, |acc, x| acc ^ x);
        sum
    }

    pub fn create_message(&self) -> String {
        let payload = self.message_body.as_string();
        let message_body = format!("${}{}*", self.device_id.id_string().unwrap(), payload);
        let message = format!("{}{:02X}\r\n", message_body.clone(), DeviceMessage::<T>::checksum(message_body));
        message
    }

    // pub fn parse_message(msg: String) -> Self {
    //     let ret_data =

    // }
}

#[cfg(test)]
mod testy{
    use super::*;
    use crate::messeges::NmeaMessages;

    #[test]
    fn device_message() {
        let gga = NmeaMessages::new_gga();
        let nmea_gga = DeviceMessage::new(DeviceId::GPS, gga);
        let gll: NmeaMessages = NmeaMessages::new_gll();
        let nmea_gll = DeviceMessage::new(DeviceId::GPS, gll);
        let gsa = NmeaMessages::new_gsa();
        let nmea_gsa = DeviceMessage::new(DeviceId::GPS, gsa);
        let gsv = NmeaMessages::new_gsv();
        let nmea_gsv = DeviceMessage::new(DeviceId::GPS, gsv);

        assert_eq!(nmea_gga.create_message(), "$GPGGA,0,N,0,E,000000,0,0,0,0,M,0,M,0,0*6D\r\n".to_string());
        assert_eq!(nmea_gll.create_message(), "$GPGLL,0,N,0,E,000000,A*1A\r\n".to_string());
        assert_eq!(nmea_gsa.create_message(), "$GPGSA,A,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*2E\r\n".to_string());
        assert_eq!(nmea_gsv.create_message(), "$GPGSV,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*49\r\n".to_string());
    }
}