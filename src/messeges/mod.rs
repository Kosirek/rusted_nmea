use crate::devices::NmeaBuff;

//******************************************************************************** Position section
/// Store geological position
#[allow(dead_code)]
pub struct NmeaPosition {
    lattitude: f32,
    lattitude_sphere: char,
    longitude: f32,
    longitude_sphere: char,
}

impl NmeaBuff for NmeaPosition {
    fn as_string(&self) -> String {
        format!("{},{},{},{}", self.lattitude, self.lattitude_sphere, self.longitude, self.longitude_sphere)
    }

    fn parse_payload(msg: String) -> Self {
        let elements:Vec<&str> = msg.split(',').collect();

        NmeaPosition {
            lattitude: elements[0].parse().unwrap_or(0.0),
            lattitude_sphere: elements[1].parse().unwrap_or('x'),
            longitude: elements[2].parse().unwrap_or(0.0),
            longitude_sphere: elements[3].parse().unwrap_or('x'),
        } 
    }
}

//******************************************************************************** Time section
/// Stores time for NMEA message
#[allow(dead_code)]
pub struct NmeaTime {
    hour: u8,
    minute: u8,
    secunde: u8,
}

impl NmeaBuff for NmeaTime {
    fn as_string(&self) -> String {
        format!("{:#02}{:#02}{:#02}", self.hour, self.minute, self.secunde)
    }

    fn parse_payload(msg: String) -> Self {
        NmeaTime {
            hour: msg[0..1].parse::<u8>().unwrap_or(0),
            minute: msg[2..3].parse::<u8>().unwrap_or(0),
            secunde: msg[4..5].parse::<u8>().unwrap_or(0),
        }

    }    
}

impl NmeaTime {
    pub fn new() -> NmeaTime {
        NmeaTime {
            hour: 0,
            minute: 0,
            secunde: 0,
        }
    }
}

//******************************************************************************** Messages payloads section
#[allow(dead_code)]
pub struct GgaPayload {
    time: NmeaTime,
    position: NmeaPosition,
}

impl NmeaBuff for GgaPayload {
    fn as_string(&self) -> String {
        format!("{},{}", self.time.as_string(), self.position.as_string())
    }

    fn parse_payload(msg: String) -> Self {
        let time = NmeaTime::parse_payload(msg.clone());
        let time_end = 1 + msg.find(',').unwrap();
        let position = NmeaPosition::parse_payload(msg[time_end..].to_string()); 

        GgaPayload {
            time: time,
            position: position,
        }
    }
}

#[allow(dead_code)]
pub struct GllPayload {
    position: NmeaPosition,
}

#[allow(dead_code)]
pub enum NmeaMessages {
    None,
    GGA(GgaPayload),
    GLL(GllPayload),
}

impl NmeaBuff for GllPayload {
    fn as_string(&self) -> String {
        format!("{}", self.position.as_string())
    }

    fn parse_payload(msg: String) -> Self {
        let position = NmeaPosition::parse_payload(msg); 

        GllPayload {
            position: position,
        }
    }
}
//******************************************************************************** Messages section
impl NmeaMessages {
    /// Returns message identifier as a String needed for correct NMEA message creation
    pub fn message_identifier(&self) -> String {
        use NmeaMessages::*;
        match self {
            None => "None".to_string(),
            GGA(_) => "GGA".to_string(),
            GLL(_) => "GLL".to_string(),
        }
    }

    /// Converts internal structure to part of NMEA message returend as a String 
    pub fn convert_to_string(&self) -> String {
        use NmeaMessages::*;
        match self {
            None => "None".to_string(),
            GGA(x) => x.as_string(),
            GLL(x) => x.as_string(),
        }

    }

    /// Function returns count of elements that message consist of
    pub fn elements_count(&self) -> u8 {
        use NmeaMessages::*;
        match self {
            None => 0,
            GGA(_) => 9,
            GLL(_) => 9,
        }
    }
}

impl NmeaBuff for NmeaMessages {
    fn as_string(&self) -> String {
        format!("{},{}*", self.message_identifier(), self.convert_to_string())
    }

    fn parse_payload(msg: String) -> Self {
        match &msg[0..2] {
            "GGA" => NmeaMessages::GGA(GgaPayload::parse_payload(msg[4..].to_string())),
            "GLL" => NmeaMessages::GLL(GllPayload::parse_payload(msg[4..].to_string())),
            _ => NmeaMessages::None,
        }
    }
}