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
struct NmeaMessage<T> {
    device_id: DeviceId,
    message_body: T,
}

#[allow(dead_code)]
impl<T> NmeaMessage<T>
where
    T: NmeaBuff,
{
    pub fn new(id: DeviceId, payload: T) -> NmeaMessage<T> {
        NmeaMessage {
            device_id: id,
            message_body: payload,
        }
    }

    pub fn empty(payload: T) -> NmeaMessage<T> {
        NmeaMessage {
            device_id: DeviceId::None,
            message_body: payload,
        }
    }

    fn checksum(msg: String) -> u8 {
        let sum: u32 = msg.into_bytes().iter().map(|x| *x as u32).sum();
        sum as u8
    }

    pub fn create_message(&self) -> String {
        let payload = self.message_body.as_string();
        let message_body = format!("${}{}*", self.device_id.id_string().unwrap(), payload);
        let message = format!("{}{:#02x}\r\n", message_body.clone(), NmeaMessage::<T>::checksum(message_body)); 
        message
    }

    // pub fn parse_message(msg: String) -> Self {
    //     let ret_data = 

    // }
}