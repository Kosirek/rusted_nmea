use crate::devices::NmeaBuff;


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

#[allow(dead_code)]
pub struct GgaPayload {
    time: NmeaTime,
    position: NmeaPosition,
}

#[allow(dead_code)]
pub struct GllPayload {
    position: NmeaPosition,
}

#[allow(dead_code)]
pub enum NmeaMessages {
    GGA(GgaPayload),
    GLL(GllPayload),
}

impl NmeaMessages {
    /* Returns message identifier */
    pub fn message_identifier(&self) -> String {
        use NmeaMessages::*;
        match self {
            GGA(_) => "GGA".to_string(),
            GLL(_) => "GLL".to_string(),
        }
    }

    /* Function returns count of elements that message consist of */
    pub fn elements_count(&self) -> u8 {
        use NmeaMessages::*;
        match self {
            GGA(_) => 9,
            GLL(_) => 9,
        }
    }
}

impl NmeaBuff for NmeaMessages {
    fn as_string(&self) -> String {
        format!("{},", self.message_identifier())
    }

    fn parse_payload(msg: String) -> Self {
        match &msg[0..2] {
            "GGA" => NmeaMessages::GGA(GgaPayload::parse_payload(msg[3..])),
            "GLL" => NmeaMessages::GLL(GllPayload::parse_payload(msg[3..])),
            _ => {}
        }
    }
}