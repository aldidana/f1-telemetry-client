use crate::f1_2020::car::TOTAL_CARS;
use crate::f1_2020::header::PacketHeader;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};
use derivative::Derivative;
use std::time::Duration;

const LAP_DATA_MIN_SIZE: usize = 1190;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PitStatus {
    None,
    Pitting,
    PitArea,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DriverStatus {
    Garage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
}

#[derive(Debug, PartialEq, Clone, Copy, Derivative)]
#[derivative(Eq)]
pub struct LapData {
    /// Last lap time in seconds
    pub last_lap_time: Duration,
    /// Current time around the lap in seconds
    pub current_lap_time: Duration,
    /// Sector 1 time in milliseconds
    pub sector_1_time: Duration,
    /// Sector 2 time in milliseconds
    pub sector_2_time: Duration,
    /// Best lap time of the session in seconds
    pub best_lap_time: Duration,
    pub best_lap_num: u8,
    /// Sector 1 time of best lap in the session in milliseconds
    pub best_lap_sector_1_time: Duration,
    /// Sector 2 time of best lap in the session in milliseconds
    pub best_lap_sector_2_time: Duration,
    /// Sector 3 time of best lap in the session in milliseconds
    pub best_lap_sector_3_time: Duration,
    /// Best overall sector 1 time of the session in milliseconds
    pub best_overall_sector_1_time: Duration,
    pub best_overall_sector_1_lap_num: u8,
    /// Best overall sector 2 time of the session in milliseconds
    pub best_overall_sector_2_time: Duration,
    pub best_overall_sector_2_lap_num: u8,
    /// Best overall sector 3 time of the session in milliseconds
    pub best_overall_sector_3_time: Duration,
    pub best_overall_sector_3_lap_num: u8,
    // #[derivative(Eq="ignore")]
    pub lap_distance: f32,
    // #[derivative(Eq="ignore")]
    pub total_distance: f32,
    // #[derivative(Eq="ignore")]
    /// Delta in seconds for safety car
    pub safety_car_delta: Duration,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub sector: u8,
    pub current_lap_invalid: bool,
    pub penalties: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PacketLapData {
    pub header: PacketHeader,
    pub lap_data: Vec<LapData>,
}

pub async fn parse_lap_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketLapData, Error> {
    ensure_lap_data_size(size)?;

    let mut laps = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let lap = parse_lap(cursor).await?;
        laps.push(lap);
    }

    Ok(PacketLapData {
        header,
        lap_data: laps,
    })
}

async fn parse_lap(cursor: &mut Cursor<Vec<u8>>) -> Result<LapData, Error> {
    let last_lap_time =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?); // in seconds
    let current_lap_time =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?); // in seconds
    let sector_1_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let sector_2_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_lap_time =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?); // in seconds
    let best_lap_num = cursor.byte_order().read_u8().await?;
    let best_lap_sector_1_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_lap_sector_2_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_lap_sector_3_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_overall_sector_1_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_overall_sector_1_lap_num = cursor.byte_order().read_u8().await?;
    let best_overall_sector_2_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_overall_sector_2_lap_num = cursor.byte_order().read_u8().await?;
    let best_overall_sector_3_time =
        Duration::from_millis(cursor.byte_order().read_u16::<LittleEndian>().await? as u64); // in ms
    let best_overall_sector_3_lap_num = cursor.byte_order().read_u8().await?;
    let lap_distance = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let total_distance = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let safety_car_delta =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?); // in seconds
    let car_position = cursor.byte_order().read_u8().await?;
    let current_lap_num = cursor.byte_order().read_u8().await?;
    let pit_status = parse_pit_status(cursor).await?;
    let sector = cursor.byte_order().read_u8().await?;
    let current_lap_invalid = cursor.byte_order().read_u8().await? == 1;
    let penalties = cursor.byte_order().read_u8().await?;
    let grid_position = cursor.byte_order().read_u8().await?;
    let driver_status = parse_driver_status(cursor).await?;
    let result_status = parse_result_status(cursor).await?;

    Ok(LapData {
        last_lap_time,
        current_lap_time,
        sector_1_time,
        sector_2_time,
        best_lap_time,
        best_lap_num,
        best_lap_sector_1_time,
        best_lap_sector_2_time,
        best_lap_sector_3_time,
        best_overall_sector_1_time,
        best_overall_sector_1_lap_num,
        best_overall_sector_2_time,
        best_overall_sector_2_lap_num,
        best_overall_sector_3_time,
        best_overall_sector_3_lap_num,
        lap_distance,
        total_distance,
        safety_car_delta,
        car_position,
        current_lap_num,
        pit_status,
        sector,
        current_lap_invalid,
        penalties,
        grid_position,
        driver_status,
        result_status,
    })
}

pub async fn parse_pit_status(cursor: &mut Cursor<Vec<u8>>) -> Result<PitStatus, Error> {
    match cursor.byte_order().read_u8().await? {
        0 => Ok(PitStatus::None),
        1 => Ok(PitStatus::Pitting),
        2 => Ok(PitStatus::PitArea),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid pit status")),
    }
}

pub async fn parse_driver_status(cursor: &mut Cursor<Vec<u8>>) -> Result<DriverStatus, Error> {
    match cursor.byte_order().read_u8().await? {
        0 => Ok(DriverStatus::Garage),
        1 => Ok(DriverStatus::FlyingLap),
        2 => Ok(DriverStatus::InLap),
        3 => Ok(DriverStatus::OutLap),
        4 => Ok(DriverStatus::OnTrack),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid driver status")),
    }
}

pub async fn parse_result_status(cursor: &mut Cursor<Vec<u8>>) -> Result<ResultStatus, Error> {
    match cursor.byte_order().read_u8().await? {
        0 => Ok(ResultStatus::Invalid),
        1 => Ok(ResultStatus::Inactive),
        2 => Ok(ResultStatus::Active),
        3 => Ok(ResultStatus::Finished),
        4 => Ok(ResultStatus::Disqualified),
        5 => Ok(ResultStatus::NotClassified),
        6 => Ok(ResultStatus::Retired),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid result status")),
    }
}

fn ensure_lap_data_size(size: usize) -> Result<(), Error> {
    if size < LAP_DATA_MIN_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Lap data size is too small",
        ));
    }

    return Ok(());
}
