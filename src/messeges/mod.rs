use crate::devices::NmeaBuff;

// TODO: consider other solution: create enum with all possible fields in NMEA messages and use vector of enums
// for each specific message type, this should allow automatic message creation and parsing

#[allow(dead_code)]
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

impl NmeaFields
{
    fn unwrap(&self) -> String {
        use NmeaFields::*;
        match self {
            Lattitude(x) => format!("{}", x),
            LattitudeSphere(x) => format!("{}", x),
            Longitude(x) => format!("{}", x),
            LongitudeSphere(x) => format!("{}", x),
            PositionTime(x) => format!("{}", x.as_string()),
            QualityIndicator(x) => format!("{}", x),
            SatellitesCount(x) => format!("{}", x),
            HorizontalDilution(x) => format!("{}", x),
            AntenaAltitude(x) => format!("{}", x),
            AltitudeUnits(x) => format!("{}", x),
            GeoidalSeparation(x) => format!("{}", x),
            GeoidalSeparationUnits(x) => format!("{}", x),
            AgeOfDifferentialData(x) => format!("{}", x),
            DifferentialReferenceStationId(x) => format!("{}", x),
            Status(x) => format!("{}", x),
            Mode(x) => format!("{}", x),
            CurrentMode(x) => format!("{}", x),
            SateliteNumber(x) => format!("{}", x),
            PositionDilutionOfPrecision(x) => format!("{}", x),
            HorizontalDilutionOfPrecision(x) => format!("{}", x),
            VerticalDilutionOfPrecision(x) => format!("{}", x),
            TotalGsvMessages(x) => format!("{}", x),
            MessageNumber(x) => format!("{}", x),
            SatellitesInView(x) => format!("{}", x),
            SatelliteElevation(x) => format!("{}", x),
            SatelliteAzimuth(x) => format!("{}", x),
            SignalToNoiseRatio(x) => format!("{}", x),
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
pub enum NmeaMessages {
    None,
    GGA(Vec<NmeaFields>),
    GLL(Vec<NmeaFields>),
    GSA(Vec<NmeaFields>),
    GSV(Vec<NmeaFields>),
}

//******************************************************************************** Messages section
impl NmeaMessages {

    pub fn new_gga() -> NmeaMessages {
        NmeaMessages::GGA(
            vec![NmeaFields::Lattitude(0.0),
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
                NmeaFields::DifferentialReferenceStationId(0),])
    }

    pub fn new_gll() -> NmeaMessages {
        NmeaMessages::GLL(
            vec![NmeaFields::Lattitude(0.0),
                NmeaFields::LattitudeSphere('N'),
                NmeaFields::Longitude(0.0),
                NmeaFields::LongitudeSphere('E'),
                NmeaFields::PositionTime(NmeaTime::new()),
                NmeaFields::Status('A'),])
    }

    pub fn new_gsa() -> NmeaMessages {
        NmeaMessages::GSA(
            vec![NmeaFields::Mode('A'),
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
                NmeaFields::VerticalDilutionOfPrecision(0.0)])
    }

    pub fn new_gsv() -> NmeaMessages {
        NmeaMessages::GSV(
            vec![NmeaFields::TotalGsvMessages(0),
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
                NmeaFields::SignalToNoiseRatio(0)])
    }

    /// Returns message identifier as a String needed for correct NMEA message creation
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
    pub fn convert_to_string(&self) -> Result<String, u32> {
        use NmeaMessages::*;
        match self {
            None => Err(1),
            GGA(x) |
            GLL(x) |
            GSA(x) |
            GSV(x) =>
            {
                if self.elements_count() as usize == x.len() {
                    let mut result = self.message_identifier();
                    x.iter().for_each(|field| result.push_str(format!(",{}", field.unwrap()).as_str()));
                    Ok(result)
                }
                else {
                    Err(1)
                }
            }
        }
    }

    /// Returns reference to payload of the message
    pub fn get_pyload(&self) -> Result<&Vec<NmeaFields>, u32> {
        use NmeaMessages::*;
        match self {
            None => Err(1),
            GGA(x) |
            GLL(x) |
            GSA(x) |
            GSV(x) =>
            {
                Ok(x)
            }
        }
    }

    /// Function returns count of elements that message consist of
    pub fn elements_count(&self) -> u8 {
        use NmeaMessages::*;
        match self {
            None => 0,
            GGA(_) => 14,
            GLL(_) => 6,
            GSA(_) => 17,
            GSV(_) => 19,
        }
    }
}

impl NmeaBuff for NmeaMessages {
    fn as_string(&self) -> String {
//        format!("{},{}*", self.message_identifier(), self.convert_to_string())
        self.convert_to_string().unwrap_or("Error".to_string())
    }

    fn parse_payload(msg: String) -> Self {
        match &msg[0..2] {
            // "GGA" => NmeaMessages::GGA(GgaPayload::parse_payload(msg[4..].to_string())),
            // "GLL" => NmeaMessages::GLL(GllPayload::parse_payload(msg[4..].to_string())),
            _ => NmeaMessages::None,
        }
    }
}

#[cfg(test)]
mod testy{
    use super::*;

    #[test]
    fn test_to_string() {
        let gga = NmeaMessages::new_gga();
        let gll = NmeaMessages::new_gll();
        let gsa = NmeaMessages::new_gsa();
        let gsv = NmeaMessages::new_gsv();

        assert_eq!(gga.convert_to_string().unwrap(), "GGA,0,N,0,E,000000,0,0,0,0,M,0,M,0,0".to_string());
        assert_eq!(gll.convert_to_string().unwrap(), "GLL,0,N,0,E,000000,A".to_string());
        assert_eq!(gsa.convert_to_string().unwrap(), "GSA,A,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".to_string());
        assert_eq!(gsv.convert_to_string().unwrap(), "GSV,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".to_string());
    }

    #[test]
    fn test_element_count() {
        let gga = NmeaMessages::new_gga();
        let gll = NmeaMessages::new_gll();
        let gsa = NmeaMessages::new_gsa();
        let gsv = NmeaMessages::new_gsv();

        assert_eq!(gga.elements_count(), 14);
        assert_eq!(gga.get_pyload().unwrap().len(), 14);
        assert_eq!(gll.elements_count(), 6);
        assert_eq!(gll.get_pyload().unwrap().len(), 6);
        assert_eq!(gsa.elements_count(), 17);
        assert_eq!(gsa.get_pyload().unwrap().len(), 17);
        assert_eq!(gsv.elements_count(), 19);
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
}