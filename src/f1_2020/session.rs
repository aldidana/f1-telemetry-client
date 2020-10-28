use crate::f1_2020::header::PacketHeader;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};
use derivative::Derivative;

const SESSION_MIN_SIZE: usize = 251;
const MARSHAL_ZONE_MAX: usize = 21;
const WEATHER_FORECAST_SAMPLE_MAX: usize = 20;

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum ZoneFlag {
    Unknown = -1,
    None = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Red = 4,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum SessionType {
    Unknown, // 0
    P1,
    P2,
    P3,
    ShortP,
    Q1,
    Q2,
    Q3,
    ShortQ,
    OSQ,
    R,
    R2,
    TimeTrial,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum Weather {
    Clear, // 0
    LightCloud,
    Overcast,
    LightRain,
    HeavyRain,
    Storm,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum Formula {
    F1Modern, // 0
    F1Classic,
    F2,
    F1Generic,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum SafetyCar {
    None, // 0,
    Full,
    Virtual,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum NetworkGame {
    Offline, // 0
    Online,
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum Track {
    Melbourne,
    PaulRicard,
    Shanghai,
    Sakhir,
    Catalunya,
    Monaco,
    Montreal,
    Silverstone,
    Hockenheim,
    Hungaroring,
    Spa,
    Monza,
    Singapore,
    Suzuka,
    AbuDhabi,
    Texas,
    Brazil,
    Austria,
    Sochi,
    Mexico,
    Baku,
    SakhirShort,
    SilverstoneShort,
    TexasShort,
    SuzukaShort,
    Hanoi,
    Zandvoort,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct MarshalZone {
    /// Fraction (0..1) of way through the lap the marshal zone starts
    pub zone_start: f32,
    /// -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
    pub zone_flag: ZoneFlag, // i8
}

#[derive(Debug, PartialOrd, Eq, PartialEq, Clone)]
pub struct WeatherForecastSample {
    /// 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
    /// 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
    /// 12 = Time Trial
    pub session_type: SessionType, // u8,
    /// Time in minutes the forecast is for
    pub time_offset: u8,
    /// Weather - 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
    pub weather: Weather, //u8,
    /// Track temp. in degrees celsius
    pub track_temperature: i8,
    /// Air temp. in degrees celsius
    pub air_temperature: i8,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketSessionData {
    /// Header
    pub header: PacketHeader,
    /// Weather - 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
    pub weather: Weather, //u8,
    /// Track temp. in degrees celsius
    pub track_temperature: i8,
    /// Air temp. in degrees celsius
    pub air_temperature: i8,
    /// Total number of laps in this race
    pub total_laps: u8,
    /// Track length in metres
    pub track_length: u16,
    /// 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
    /// 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
    /// 10 = R, 11 = R2, 12 = Time Trial
    pub session_type: SessionType, //u8,
    /// -1 for unknown, 0-21 for tracks, see appendix
    pub track_id: Track, //i8,
    /// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
    /// 3 = F1 Generic
    pub formula: Formula, // u8,
    /// Time left in session in seconds
    pub session_time_left: u16,
    /// Session duration in seconds
    pub session_duration: u16,
    /// Pit speed limit in kilometres per hour
    pub pit_speed_limit: u8,
    /// Whether the game is paused
    pub game_paused: u8,
    /// Whether the player is spectating
    pub is_spectating: u8,
    /// Index of the car being spectated
    pub spectator_car_index: u8,
    /// SLI Pro support, 0 = inactive, 1 = active
    pub sli_pro_native_support: u8,
    /// Number of marshal zones to follow
    pub num_marshal_zones: u8,
    /// List of marshal zones – max 21
    pub marshal_zone: Vec<MarshalZone>,
    /// 0 = no safety car, 1 = full safety car
    /// 2 = virtual safety car
    pub safety_car_status: SafetyCar, //u8,
    /// 0 = offline, 1 = online
    pub network_game: NetworkGame, // u8,
    /// Number of weather samples to follow
    pub num_weather_forecast_samples: u8,
    /// Array of weather forecast samples
    pub weather_forecast_sample: Vec<WeatherForecastSample>,
}

pub async fn parse_session(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketSessionData, Error> {
    ensure_session_size(size)?;

    let weather = parse_weather(cursor.byte_order().read_u8().await?)?;

    let track_temperature = cursor.byte_order().read_i8().await?;
    let air_temperature = cursor.byte_order().read_i8().await?;
    let total_laps = cursor.byte_order().read_u8().await?;
    let track_length = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let session_type = parse_session_type(cursor.byte_order().read_u8().await?)?;
    let track_id = parse_track(cursor.byte_order().read_i8().await?)?;
    let formula = parse_formula(cursor.byte_order().read_u8().await?)?;
    let session_time_left = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let session_duration = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let pit_speed_limit = cursor.byte_order().read_u8().await?;
    let game_paused = cursor.byte_order().read_u8().await?;
    let is_spectating = cursor.byte_order().read_u8().await?;
    let spectator_car_index = cursor.byte_order().read_u8().await?;
    let sli_pro_native_support = cursor.byte_order().read_u8().await?;

    let num_marshal_zones = cursor.byte_order().read_u8().await?;
    let mut marshal_zone = Vec::with_capacity(MARSHAL_ZONE_MAX);
    for _ in 0..num_marshal_zones {
        let zone_start = cursor.byte_order().read_f32::<LittleEndian>().await?;
        let zone_flag = parse_flag(cursor.byte_order().read_i8().await?)?;
        let zone = parse_marshal_zone(zone_start, zone_flag)?;
        marshal_zone.push(zone);
    }

    let safety_car_status = parse_safety_car(cursor.byte_order().read_u8().await?)?;
    let network_game = parse_network_game(cursor.byte_order().read_u8().await?)?;

    let num_weather_forecast_samples = cursor.byte_order().read_u8().await?;
    let mut weather_forecast_sample = Vec::with_capacity(WEATHER_FORECAST_SAMPLE_MAX);
    for _ in 0..WEATHER_FORECAST_SAMPLE_MAX {
        let weather_forecast = parse_weather_forecast_sample(cursor).await?;
        weather_forecast_sample.push(weather_forecast);
    }

    Ok(PacketSessionData {
        header,
        weather,
        track_temperature,
        air_temperature,
        total_laps,
        track_length,
        session_type,
        track_id,
        formula,
        session_time_left,
        session_duration,
        pit_speed_limit,
        game_paused,
        is_spectating,
        spectator_car_index,
        sli_pro_native_support,
        num_marshal_zones,
        marshal_zone,
        safety_car_status,
        network_game,
        num_weather_forecast_samples,
        weather_forecast_sample,
    })
}

fn ensure_session_size(size: usize) -> Result<(), Error> {
    if size == SESSION_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Session size is too small",
    ))
}

fn parse_weather(value: u8) -> Result<Weather, Error> {
    match value {
        0 => Ok(Weather::Clear),
        1 => Ok(Weather::LightCloud),
        2 => Ok(Weather::Overcast),
        3 => Ok(Weather::LightRain),
        4 => Ok(Weather::HeavyRain),
        5 => Ok(Weather::Storm),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid weather")),
    }
}

fn parse_session_type(value: u8) -> Result<SessionType, Error> {
    match value {
        0 => Ok(SessionType::Unknown),
        1 => Ok(SessionType::P1),
        2 => Ok(SessionType::P2),
        3 => Ok(SessionType::P3),
        4 => Ok(SessionType::ShortP),
        5 => Ok(SessionType::Q1),
        6 => Ok(SessionType::Q2),
        7 => Ok(SessionType::Q3),
        8 => Ok(SessionType::ShortQ),
        9 => Ok(SessionType::ShortQ),
        10 => Ok(SessionType::R),
        11 => Ok(SessionType::R2),
        12 => Ok(SessionType::TimeTrial),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid session type")),
    }
}

fn parse_track(value: i8) -> Result<Track, Error> {
    match value {
        -1 => Ok(Track::Unknown),
        0 => Ok(Track::Melbourne),
        1 => Ok(Track::PaulRicard),
        2 => Ok(Track::Shanghai),
        3 => Ok(Track::Sakhir),
        4 => Ok(Track::Catalunya),
        5 => Ok(Track::Monaco),
        6 => Ok(Track::Montreal),
        7 => Ok(Track::Silverstone),
        8 => Ok(Track::Hockenheim),
        9 => Ok(Track::Hungaroring),
        10 => Ok(Track::Spa),
        11 => Ok(Track::Monza),
        12 => Ok(Track::Singapore),
        13 => Ok(Track::Suzuka),
        14 => Ok(Track::AbuDhabi),
        15 => Ok(Track::Texas),
        16 => Ok(Track::Brazil),
        17 => Ok(Track::Austria),
        18 => Ok(Track::Sochi),
        19 => Ok(Track::Mexico),
        20 => Ok(Track::Baku),
        21 => Ok(Track::SakhirShort),
        22 => Ok(Track::SilverstoneShort),
        23 => Ok(Track::TexasShort),
        24 => Ok(Track::SuzukaShort),
        25 => Ok(Track::Hanoi),
        26 => Ok(Track::Zandvoort),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid track")),
    }
}

fn parse_formula(value: u8) -> Result<Formula, Error> {
    match value {
        0 => Ok(Formula::F1Modern),
        1 => Ok(Formula::F1Classic),
        2 => Ok(Formula::F2),
        3 => Ok(Formula::F1Generic),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid formula type")),
    }
}

pub fn parse_flag(value: i8) -> Result<ZoneFlag, Error> {
    match value {
        -1 => Ok(ZoneFlag::Unknown),
        0 => Ok(ZoneFlag::None),
        1 => Ok(ZoneFlag::Green),
        2 => Ok(ZoneFlag::Blue),
        3 => Ok(ZoneFlag::Yellow),
        4 => Ok(ZoneFlag::Red),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid zone flag")),
    }
}

fn parse_marshal_zone(zone_start: f32, zone_flag: ZoneFlag) -> Result<MarshalZone, Error> {
    Ok(MarshalZone {
        zone_start,
        zone_flag,
    })
}

fn parse_safety_car(value: u8) -> Result<SafetyCar, Error> {
    match value {
        0 => Ok(SafetyCar::None),
        1 => Ok(SafetyCar::Full),
        2 => Ok(SafetyCar::Virtual),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid safety car")),
    }
}

fn parse_network_game(value: u8) -> Result<NetworkGame, Error> {
    match value {
        0 => Ok(NetworkGame::Offline),
        1 => Ok(NetworkGame::Online),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid network game")),
    }
}

async fn parse_weather_forecast_sample(
    cursor: &mut Cursor<Vec<u8>>,
) -> Result<WeatherForecastSample, Error> {
    let session_type = parse_session_type(cursor.byte_order().read_u8().await?)?;
    let time_offset = cursor.byte_order().read_u8().await?;
    let weather = parse_weather(cursor.byte_order().read_u8().await?)?;
    let track_temperature = cursor.byte_order().read_i8().await?;
    let air_temperature = cursor.byte_order().read_i8().await?;

    Ok(WeatherForecastSample {
        session_type,
        time_offset,
        weather,
        track_temperature,
        air_temperature,
    })
}
