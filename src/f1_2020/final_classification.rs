use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};
use derivative::Derivative;
use std::time::Duration;

use crate::f1_2020::car::{ActualTyreCompound, VisualTyreCompound, TOTAL_CARS};
use crate::f1_2020::car_status::{parse_actual_tyre_compound, parse_visual_tyre_compound};
use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::lap::{parse_result_status, ResultStatus};

const FINAL_CLASSIFICATION_MIN_SIZE: usize = 839;

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct FinalClassificationData {
    position: u8,
    num_laps: u8,
    grid_position: u8,
    points: u8,
    num_pit_stops: u8,
    result_status: ResultStatus,
    best_lap_time: Duration,   // seconds
    total_race_time: Duration, // seconds
    penalties_time: u8,
    num_penalties: u8,
    num_tyre_stints: u8,
    tyre_stints_actual: Vec<ActualTyreCompound>,
    tyre_stints_visual: Vec<VisualTyreCompound>,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketFinalClassificationData {
    header: PacketHeader,
    num_cars: u8,
    final_classification_data: Vec<FinalClassificationData>,
}

pub async fn parse_final_classification_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketFinalClassificationData, Error> {
    ensure_final_classification_size(size)?;

    let num_cars = cursor.byte_order().read_u8().await?;

    let mut final_classification_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let data = parse_final_classification(cursor).await?;
        final_classification_data.push(data);
    }

    Ok(PacketFinalClassificationData {
        header,
        num_cars,
        final_classification_data,
    })
}

pub async fn parse_final_classification(
    cursor: &mut Cursor<Vec<u8>>,
) -> Result<FinalClassificationData, Error> {
    let position = cursor.byte_order().read_u8().await?;
    let num_laps = cursor.byte_order().read_u8().await?;
    let grid_position = cursor.byte_order().read_u8().await?;
    let points = cursor.byte_order().read_u8().await?;
    let num_pit_stops = cursor.byte_order().read_u8().await?;
    let result_status = parse_result_status(cursor).await?;
    let best_lap_time =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?);
    let total_race_time =
        Duration::from_secs_f64(cursor.byte_order().read_f64::<LittleEndian>().await?);
    let penalties_time = cursor.byte_order().read_u8().await?;
    let num_penalties = cursor.byte_order().read_u8().await?;
    let num_tyre_stints = cursor.byte_order().read_u8().await?;

    let mut tyre_stints_actual = Vec::with_capacity(8);
    for _ in 0..8 {
        let tc = parse_actual_tyre_compound(cursor.byte_order().read_u8().await?)?;
        tyre_stints_actual.push(tc);
    }

    let mut tyre_stints_visual = Vec::with_capacity(8);
    for _ in 0..8 {
        let tc = parse_visual_tyre_compound(cursor.byte_order().read_u8().await?)?;
        tyre_stints_visual.push(tc);
    }

    Ok(FinalClassificationData {
        position,
        num_laps,
        grid_position,
        points,
        num_pit_stops,
        result_status,
        best_lap_time,
        total_race_time,
        penalties_time,
        num_penalties,
        num_tyre_stints,
        tyre_stints_actual,
        tyre_stints_visual,
    })
}

fn ensure_final_classification_size(size: usize) -> Result<(), Error> {
    if size == FINAL_CLASSIFICATION_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Final classification size is too small",
    ))
}
