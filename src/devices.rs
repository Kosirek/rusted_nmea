pub trait NmeaBuff {
    fn as_string(&self) -> String;

    fn parse_payload(msg: String) -> Self;
}

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

struct NmeaMessage<T> {
    device_id: DeviceId,
    message_body: T,
}

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

    // pub fn empty(payload: T) -> NmeaMessage<T> {

    // }

    pub fn create_message(&self) -> String {
        let payload = self.message_body.as_string();
        let message = format!("${}{}*{:#02x}\r", self.device_id.id_string().unwrap(), payload, 12);
        message
    }

    // pub fn parse_message(msg: String) -> Self {
    //     let ret_data = 

    // }
}