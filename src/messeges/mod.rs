use crate::devices::{NmeaBuff, NmeaError};
use thiserror::Error;
use tracing::instrument;

// TODO: consider other solution: create enum with all possible fields in NMEA messages and use vector of enums
// for each specific message type, this should allow automatic message creation and parsing

#[derive(Error, Debug)]
pub enum NmeaMessageError {
    #[error("Invalid NMEA field")]
    InvalidField,

    #[error("Invalid NMEA payload")]
    InvalidPayload,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NmeaFields {
    Lattitude(f32),
    LattitudeSphere(char),
    Longitude(f32),
    LongitudeSphere(char),
    PositionTime(NmeaTime),
    QualityIndicator(u8),
    SatellitesCount(u8),
    HorizontalDilution(f32),
    AntenaAltitude(f32),
    AltitudeUnits(char),
    GeoidalSeparation(f32),
    GeoidalSeparationUnits(char),
    AgeOfDifferentialData(f32),
    DifferentialReferenceStationId(u16),
    Status(char),
    Mode(char),
    CurrentMode(u8),
    SateliteNumber(u8),
    PositionDilutionOfPrecision(f32),
    HorizontalDilutionOfPrecision(f32),
    VerticalDilutionOfPrecision(f32),
    TotalGsvMessages(u8),
    MessageNumber(u8),
    SatellitesInView(u8),
    SatelliteElevation(u8),
    SatelliteAzimuth(u16),
    SignalToNoiseRatio(u8),
}

impl NmeaFields {
    #[instrument]
    fn unwrap(&self) -> Option<String> {
        use NmeaFields::*;
        match self {
            Lattitude(x) => Some(format!("{}", x)),
            LattitudeSphere(x) => Some(format!("{}", x)),
            Longitude(x) => Some(format!("{}", x)),
            LongitudeSphere(x) => Some(format!("{}", x)),
            PositionTime(x) => match x.as_string() {
                Ok(string) => Some(string),
                Err(_) => None,
            },
            QualityIndicator(x) => Some(format!("{}", x)),
            SatellitesCount(x) => Some(format!("{}", x)),
            HorizontalDilution(x) => Some(format!("{}", x)),
            AntenaAltitude(x) => Some(format!("{}", x)),
            AltitudeUnits(x) => Some(format!("{}", x)),
            GeoidalSeparation(x) => Some(format!("{}", x)),
            GeoidalSeparationUnits(x) => Some(format!("{}", x)),
            AgeOfDifferentialData(x) => Some(format!("{}", x)),
            DifferentialReferenceStationId(x) => Some(format!("{}", x)),
            Status(x) => Some(format!("{}", x)),
            Mode(x) => Some(format!("{}", x)),
            CurrentMode(x) => Some(format!("{}", x)),
            SateliteNumber(x) => Some(format!("{}", x)),
            PositionDilutionOfPrecision(x) => Some(format!("{}", x)),
            HorizontalDilutionOfPrecision(x) => Some(format!("{}", x)),
            VerticalDilutionOfPrecision(x) => Some(format!("{}", x)),
            TotalGsvMessages(x) => Some(format!("{}", x)),
            MessageNumber(x) => Some(format!("{}", x)),
            SatellitesInView(x) => Some(format!("{}", x)),
            SatelliteElevation(x) => Some(format!("{}", x)),
            SatelliteAzimuth(x) => Some(format!("{}", x)),
            SignalToNoiseRatio(x) => Some(format!("{}", x)),
        }
    }

    #[instrument]
    fn assign_value(&mut self, value: String) {
        use NmeaFields::*;
        match self {
            Lattitude(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            LattitudeSphere(x) => *x = value.parse::<char>().unwrap_or('N'),
            Longitude(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            LongitudeSphere(x) => *x = value.parse::<char>().unwrap_or('E'),
            PositionTime(x) => *x = NmeaTime::parse_payload(value),
            QualityIndicator(x) => *x = value.parse::<u8>().unwrap_or(0),
            SatellitesCount(x) => *x = value.parse::<u8>().unwrap_or(0),
            HorizontalDilution(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            AntenaAltitude(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            AltitudeUnits(x) => *x = value.parse::<char>().unwrap_or('M'),
            GeoidalSeparation(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            GeoidalSeparationUnits(x) => *x = value.parse::<char>().unwrap_or('M'),
            AgeOfDifferentialData(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            DifferentialReferenceStationId(x) => *x = value.parse::<u16>().unwrap_or(0),
            Status(x) => *x = value.parse::<char>().unwrap_or('A'),
            Mode(x) => *x = value.parse::<char>().unwrap_or('A'),
            CurrentMode(x) => *x = value.parse::<u8>().unwrap_or(1),
            SateliteNumber(x) => *x = value.parse::<u8>().unwrap_or(0),
            PositionDilutionOfPrecision(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            HorizontalDilutionOfPrecision(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            VerticalDilutionOfPrecision(x) => *x = value.parse::<f32>().unwrap_or(0.0),
            TotalGsvMessages(x) => *x = value.parse::<u8>().unwrap_or(0),
            MessageNumber(x) => *x = value.parse::<u8>().unwrap_or(0),
            SatellitesInView(x) => *x = value.parse::<u8>().unwrap_or(0),
            SatelliteElevation(x) => *x = value.parse::<u8>().unwrap_or(0),
            SatelliteAzimuth(x) => *x = value.parse::<u16>().unwrap_or(0),
            SignalToNoiseRatio(x) => *x = value.parse::<u8>().unwrap_or(0),
        }
    }
}

//******************************************************************************** Time section
/// Stores time for NMEA message
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct NmeaTime {
    hour: u8,
    minute: u8,
    secunde: u8,
}

impl NmeaBuff for NmeaTime {
    #[instrument]
    fn as_string(&self) -> Result<String, NmeaError> {
        Ok(format!(
            "{:#02}{:#02}{:#02}",
            self.hour, self.minute, self.secunde
        ))
    }

    #[instrument]
    fn parse_payload(msg: String) -> Self {
        NmeaTime {
            hour: msg[0..1].parse::<u8>().unwrap_or(0),
            minute: msg[2..3].parse::<u8>().unwrap_or(0),
            secunde: msg[4..5].parse::<u8>().unwrap_or(0),
        }
    }
}

impl NmeaTime {
    #[instrument]
    pub fn new() -> NmeaTime {
        NmeaTime {
            hour: 0,
            minute: 0,
            secunde: 0,
        }
    }

    #[instrument]
    pub fn set_time(&mut self, hour: u8, minute: u8, secunde: u8) {
        self.hour = hour;
        self.minute = minute;
        self.secunde = secunde;
    }

    #[instrument]
    pub fn get_time(&self) -> (u8, u8, u8) {
        (self.hour, self.minute, self.secunde)
    }
}
//******************************************************************************** Messages payloads section
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NmeaMessages {
    None,
    GGA(Vec<NmeaFields>),
    GLL(Vec<NmeaFields>),
    GSA(Vec<NmeaFields>),
    GSV(Vec<NmeaFields>),
}

//******************************************************************************** Messages section
impl NmeaMessages {
    /// Creates new GGA message with default values
    #[instrument]
    pub fn new_gga() -> NmeaMessages {
        NmeaMessages::GGA(vec![
            NmeaFields::Lattitude(0.0),
            NmeaFields::LattitudeSphere('N'),
            NmeaFields::Longitude(0.0),
            NmeaFields::LongitudeSphere('E'),
            NmeaFields::PositionTime(NmeaTime::new()),
            NmeaFields::QualityIndicator(0),
            NmeaFields::SatellitesCount(0),
            NmeaFields::HorizontalDilution(0.0),
            NmeaFields::AntenaAltitude(0.0),
            NmeaFields::AltitudeUnits('M'),
            NmeaFields::GeoidalSeparation(0.0),
            NmeaFields::GeoidalSeparationUnits('M'),
            NmeaFields::AgeOfDifferentialData(0.0),
            NmeaFields::DifferentialReferenceStationId(0),
        ])
    }

    /// Creates new GLL message with default values
    #[instrument]
    pub fn new_gll() -> NmeaMessages {
        NmeaMessages::GLL(vec![
            NmeaFields::Lattitude(0.0),
            NmeaFields::LattitudeSphere('N'),
            NmeaFields::Longitude(0.0),
            NmeaFields::LongitudeSphere('E'),
            NmeaFields::PositionTime(NmeaTime::new()),
            NmeaFields::Status('A'),
        ])
    }

    /// Creates new GSA message with default values
    #[instrument]
    pub fn new_gsa() -> NmeaMessages {
        NmeaMessages::GSA(vec![
            NmeaFields::Mode('A'),
            NmeaFields::CurrentMode(1),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::PositionDilutionOfPrecision(0.0),
            NmeaFields::HorizontalDilutionOfPrecision(0.0),
            NmeaFields::VerticalDilutionOfPrecision(0.0),
        ])
    }

    /// Creates new GSV message with default values
    #[instrument]
    pub fn new_gsv() -> NmeaMessages {
        NmeaMessages::GSV(vec![
            NmeaFields::TotalGsvMessages(0),
            NmeaFields::MessageNumber(0),
            NmeaFields::SatellitesInView(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SatelliteElevation(0),
            NmeaFields::SatelliteAzimuth(0),
            NmeaFields::SignalToNoiseRatio(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SatelliteElevation(0),
            NmeaFields::SatelliteAzimuth(0),
            NmeaFields::SignalToNoiseRatio(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SatelliteElevation(0),
            NmeaFields::SatelliteAzimuth(0),
            NmeaFields::SignalToNoiseRatio(0),
            NmeaFields::SateliteNumber(0),
            NmeaFields::SatelliteElevation(0),
            NmeaFields::SatelliteAzimuth(0),
            NmeaFields::SignalToNoiseRatio(0),
        ])
    }

    /// Returns message identifier as a String needed for correct NMEA message creation
    #[instrument]
    pub fn message_identifier(&self) -> String {
        use NmeaMessages::*;
        match self {
            None => "None".to_string(),
            GGA(_) => "GGA".to_string(),
            GLL(_) => "GLL".to_string(),
            GSA(_) => "GSA".to_string(),
            GSV(_) => "GSV".to_string(),
        }
    }

    /// Converts internal structure to part of NMEA message returend as a String
    #[instrument]
    pub fn convert_to_string(&self) -> Result<String, NmeaMessageError> {
        use NmeaMessages::*;
        match self {
            None => Err(NmeaMessageError::InvalidField),
            GGA(x) | GLL(x) | GSA(x) | GSV(x) => {
                if self.elements_count()? as usize == x.len() {
                    let mut result = self.message_identifier();
                    x.iter().for_each(|field| {
                        result.push_str(format!(",{}", field.unwrap().unwrap()).as_str())
                    });
                    Ok(result)
                } else {
                    Err(NmeaMessageError::InvalidPayload)
                }
            }
        }
    }

    /// Returns reference to payload of the message
    #[instrument]
    pub fn get_pyload(&mut self) -> Result<&mut Vec<NmeaFields>, NmeaMessageError> {
        use NmeaMessages::*;
        match self {
            None => Err(NmeaMessageError::InvalidField),
            GGA(x) | GLL(x) | GSA(x) | GSV(x) => Ok(x),
        }
    }

    /// Function returns count of elements that message consist of
    #[instrument]
    pub fn elements_count(&self) -> Result<u8, NmeaMessageError> {
        use NmeaMessages::*;
        match self {
            None => Err(NmeaMessageError::InvalidField),
            GGA(_) => Ok(14),
            GLL(_) => Ok(6),
            GSA(_) => Ok(17),
            GSV(_) => Ok(19),
        }
    }
}

impl NmeaBuff for NmeaMessages {
    #[instrument]
    fn as_string(&self) -> Result<String, NmeaError> {
        self.convert_to_string().map_err(|error| NmeaError::IvalidMessageBuffer(error))
    }

    #[instrument]
    fn parse_payload(msg: String) -> Self {
        let mut parsed_msg = match &msg[0..3] {
            "GGA" => NmeaMessages::new_gga(),
            "GLL" => NmeaMessages::new_gll(),
            "GSA" => NmeaMessages::new_gsa(),
            "GSV" => NmeaMessages::new_gsv(),
            _ => NmeaMessages::None,
        };
        let fields: &Vec<&str> = &msg[4..msg.len()].split(",").collect();

        if fields.len() != parsed_msg.elements_count().unwrap() as usize {
            return NmeaMessages::None;
        }

        parsed_msg
            .get_pyload()
            .unwrap()
            .iter_mut()
            .zip(fields.iter())
            .for_each(|(field, value)| {
                field.assign_value((*value).to_string());
            });

        parsed_msg
    }
}

#[cfg(test)]
mod payload_tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let gga = NmeaMessages::new_gga();
        let gll = NmeaMessages::new_gll();
        let gsa = NmeaMessages::new_gsa();
        let gsv = NmeaMessages::new_gsv();

        assert_eq!(
            gga.convert_to_string().unwrap(),
            "GGA,0,N,0,E,000000,0,0,0,0,M,0,M,0,0".to_string()
        );
        assert_eq!(
            gll.convert_to_string().unwrap(),
            "GLL,0,N,0,E,000000,A".to_string()
        );
        assert_eq!(
            gsa.convert_to_string().unwrap(),
            "GSA,A,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".to_string()
        );
        assert_eq!(
            gsv.convert_to_string().unwrap(),
            "GSV,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".to_string()
        );
    }

    #[test]
    fn test_element_count() {
        let mut gga = NmeaMessages::new_gga();
        let mut gll = NmeaMessages::new_gll();
        let mut gsa = NmeaMessages::new_gsa();
        let mut gsv = NmeaMessages::new_gsv();

        assert_eq!(gga.elements_count().unwrap(), 14);
        assert_eq!(gga.get_pyload().unwrap().len(), 14);
        assert_eq!(gll.elements_count().unwrap(), 6);
        assert_eq!(gll.get_pyload().unwrap().len(), 6);
        assert_eq!(gsa.elements_count().unwrap(), 17);
        assert_eq!(gsa.get_pyload().unwrap().len(), 17);
        assert_eq!(gsv.elements_count().unwrap(), 19);
        assert_eq!(gsv.get_pyload().unwrap().len(), 19);
    }

    #[test]
    fn test_message_identifier() {
        let gga = NmeaMessages::new_gga();
        let gll = NmeaMessages::new_gll();
        let gsa = NmeaMessages::new_gsa();
        let gsv = NmeaMessages::new_gsv();

        assert_eq!(gga.message_identifier(), "GGA".to_string());
        assert_eq!(gll.message_identifier(), "GLL".to_string());
        assert_eq!(gsa.message_identifier(), "GSA".to_string());
        assert_eq!(gsv.message_identifier(), "GSV".to_string());
    }

    #[test]
    fn test_parse_payload() {
        let gga = NmeaMessages::new_gga();
        let gll = NmeaMessages::new_gll();
        let gsa = NmeaMessages::new_gsa();
        let gsv = NmeaMessages::new_gsv();

        let gga_str = gga.as_string().unwrap();
        let gll_str = gll.as_string().unwrap();
        let gsa_str = gsa.as_string().unwrap();
        let gsv_str = gsv.as_string().unwrap();

        let gga_parsed = NmeaMessages::parse_payload(gga_str);
        let gll_parsed = NmeaMessages::parse_payload(gll_str);
        let gsa_parsed = NmeaMessages::parse_payload(gsa_str);
        let gsv_parsed = NmeaMessages::parse_payload(gsv_str);

        assert_eq!(gga, gga_parsed);
        assert_eq!(gll, gll_parsed);
        assert_eq!(gsa, gsa_parsed);
        assert_eq!(gsv, gsv_parsed);
    }
}
