use crate::messeges;
use thiserror::Error;
use tracing::instrument;

#[derive(Error, Debug)]
pub enum NmeaError {
    #[error("Invalid NMEA message")]
    InvalidNmeaMessage,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Invalid device identifier")]
    InvalidDeviceId,
    #[error("Invalid payload")]
    InvalidPayload,
    #[error("Invalid message buffer")]
    IvalidMessageBuffer(#[from] messeges::NmeaMessageError)
}

pub trait NmeaBuff {
    /// Used for NMEA message serialization from bottom to top structures
    fn as_string(&self) -> Result<String, NmeaError>;

    /// Used for parsing (deserialization) NMEA message from top to bottom structures
    fn parse_payload(msg: String) -> Self;
}

#[allow(dead_code)]
#[derive(Debug)]
enum DeviceId {
    None,
    GPS,
    EchoSounder,
    Radar,
}

impl DeviceId {
    #[instrument]
    pub fn id_string(&self) -> Result<String, NmeaError> {
        use crate::devices::DeviceId::*;
        match self {
            GPS => Ok("GP".to_string()),
            EchoSounder => Ok("ES".to_string()),
            Radar => Ok("RD".to_string()),
            None => Err(NmeaError::InvalidDeviceId),
        }
    }

    #[instrument]
    pub fn id_from_message(msg: &str) -> Result<DeviceId, NmeaError> {
        use crate::devices::DeviceId::*;
        match msg {
            "GP" => Ok(GPS),
            "ES" => Ok(EchoSounder),
            "RD" => Ok(Radar),
            _ => Err(NmeaError::InvalidDeviceId),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct DeviceMessage<T> {
    device_id: DeviceId,
    message_body: T,
}
// TODO: rework functions to return Result
#[allow(dead_code)]
impl<T> DeviceMessage<T>
where
    T: NmeaBuff + std::fmt::Debug,
{
    /// Create a new NMEA message
    #[instrument]
    pub fn new(id: DeviceId, payload: T) -> DeviceMessage<T> {
        DeviceMessage {
            device_id: id,
            message_body: payload,
        }
    }

    /// Create an empty NMEA message
    #[instrument]
    pub fn empty(payload: T) -> DeviceMessage<T> {
        DeviceMessage {
            device_id: DeviceId::None,
            message_body: payload,
        }
    }

    /// Calculate the checksum of the NMEA message
    /// The checksum is calculated by XORing all the bytes between the $ and the * (not including the delimiters themselves)
    #[instrument]
    fn checksum(msg: String) -> u8 {
        let sum = msg[1..msg.len() - 1]
            .to_string()
            .into_bytes()
            .iter()
            .fold(0, |acc, x| acc ^ x);
        sum
    }

    /// Creating string message
    #[instrument]
    pub fn create_message(&self) -> Result<String, NmeaError> {
        let payload = self.message_body.as_string()?;
        let message_body = format!("${}{}*", self.device_id.id_string()?, payload);
        let message = format!(
            "{}{:02X}\r\n",
            message_body.clone(),
            DeviceMessage::<T>::checksum(message_body)
        );
        Ok(message)
    }

    // pub fn parse_message(msg: String) -> Self {
    //     let ret_data =

    // }
}

#[cfg(test)]
mod message {
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

        assert_eq!(
            nmea_gga.create_message().unwrap(),
            "$GPGGA,0,N,0,E,000000,0,0,0,0,M,0,M,0,0*6D\r\n".to_string()
        );
        assert_eq!(
            nmea_gll.create_message().unwrap(),
            "$GPGLL,0,N,0,E,000000,A*1A\r\n".to_string()
        );
        assert_eq!(
            nmea_gsa.create_message().unwrap(),
            "$GPGSA,A,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*2E\r\n".to_string()
        );
        assert_eq!(
            nmea_gsv.create_message().unwrap(),
            "$GPGSV,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*49\r\n".to_string()
        );
    }
}
