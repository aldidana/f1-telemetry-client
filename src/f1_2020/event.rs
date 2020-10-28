use crate::f1_2020::header::PacketHeader;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};
use std::time::Duration;

use derivative::Derivative;

const EVENT_MIN_SIZE: usize = 35;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct FastestLap {
    /// Vehicle index of car achieveing fastest lap
    pub vehicle_index: u8,
    /// Lap time in seconds
    pub lap_time: Duration,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Retirement {
    /// Vehicle index
    pub vehicle_index: u8,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TeamMateInPits {
    /// Vehicle index
    pub vehicle_index: u8,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RaceWinner {
    /// Vehicle index
    pub vehicle_index: u8,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PenaltyType {
    DriveThrough,
    StopGo,
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisLapInvalidatedWithoutReason,
    ThisAndNextLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InfringementType {
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Penalty {
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    /// Vehicle index of the car the penalty is applied to
    pub vehicle_index: u8,
    /// Vehicle index of the other car involved
    pub other_vehicle_index: u8,
    /// Time gained, or time spent doing action in seconds
    pub time: Duration,
    /// Lap the penalty occured on
    pub lap_num: u8,
    /// Number of places gained by this
    pub places_gained: u8,
}

#[derive(Debug, PartialEq, Copy, Clone, Derivative)]
#[derivative(Eq)]
pub struct SpeedTrap {
    pub vehicle_index: u8,
    pub speed: f32,
}

#[derive(Debug, PartialEq, Copy, Clone, Derivative)]
#[derivative(Eq)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(Retirement),
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits(TeamMateInPits),
    ChequeredFlag,
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
}

#[derive(Debug, PartialEq, Copy, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketEventData {
    pub header: PacketHeader,
    pub event: Event,
}

pub async fn parse_event_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketEventData, Error> {
    ensure_event_size(size)?;

    let event = parse_event(cursor).await?;
    Ok(PacketEventData { header, event })
}

pub async fn parse_event(cursor: &mut Cursor<Vec<u8>>) -> Result<Event, Error> {
    let mut event_chars: Vec<char> = Vec::with_capacity(4);
    for _ in 0..4 {
        event_chars.push(cursor.byte_order().read_u8().await? as char);
    }
    let event_code = event_chars.into_iter().collect::<String>();

    match event_code.as_str() {
        "SSTA" => Ok(Event::SessionStarted),
        "SEND" => Ok(Event::SessionEnded),
        "FTLP" => {
            let vehicle_index = cursor.byte_order().read_u8().await?;
            let lap_time =
                Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?);

            Ok(Event::FastestLap(FastestLap {
                vehicle_index,
                lap_time,
            }))
        }
        "RTMT" => {
            let vehicle_index = cursor.byte_order().read_u8().await?;

            Ok(Event::Retirement(Retirement { vehicle_index }))
        }
        "DRSE" => Ok(Event::DRSEnabled),
        "DRSD" => Ok(Event::DRSDisabled),
        "TMPT" => {
            let vehicle_index = cursor.byte_order().read_u8().await?;

            Ok(Event::TeamMateInPits(TeamMateInPits { vehicle_index }))
        }
        "CHQF" => Ok(Event::ChequeredFlag),
        "RCWN" => {
            let vehicle_index = cursor.byte_order().read_u8().await?;

            Ok(Event::RaceWinner(RaceWinner { vehicle_index }))
        }
        "PENA" => {
            let penalty_type = parse_penalty_type(cursor.byte_order().read_u8().await?)?;
            let infringement_type = parse_infringement_type(cursor.byte_order().read_u8().await?)?;
            let vehicle_index = cursor.byte_order().read_u8().await?;
            let other_vehicle_index = cursor.byte_order().read_u8().await?;
            let time = Duration::from_secs(cursor.byte_order().read_u8().await? as u64);
            let lap_num = cursor.byte_order().read_u8().await?;
            let places_gained = cursor.byte_order().read_u8().await?;

            Ok(Event::Penalty(Penalty {
                vehicle_index,
                penalty_type,
                infringement_type,
                other_vehicle_index,
                time,
                lap_num,
                places_gained,
            }))
        }
        "SPTP" => {
            let vehicle_index = cursor.byte_order().read_u8().await?;
            let speed = cursor.byte_order().read_f32::<LittleEndian>().await?;

            Ok(Event::SpeedTrap(SpeedTrap {
                vehicle_index,
                speed,
            }))
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid event code")),
    }
}

fn parse_penalty_type(value: u8) -> Result<PenaltyType, Error> {
    match value {
        0 => Ok(PenaltyType::DriveThrough),
        1 => Ok(PenaltyType::StopGo),
        2 => Ok(PenaltyType::GridPenalty),
        3 => Ok(PenaltyType::PenaltyReminder),
        4 => Ok(PenaltyType::TimePenalty),
        5 => Ok(PenaltyType::Warning),
        6 => Ok(PenaltyType::Disqualified),
        7 => Ok(PenaltyType::RemovedFromFormationLap),
        8 => Ok(PenaltyType::ParkedTooLongTimer),
        9 => Ok(PenaltyType::TyreRegulations),
        10 => Ok(PenaltyType::ThisLapInvalidated),
        11 => Ok(PenaltyType::ThisAndNextLapInvalidated),
        12 => Ok(PenaltyType::ThisLapInvalidatedWithoutReason),
        13 => Ok(PenaltyType::ThisAndNextLapInvalidatedWithoutReason),
        14 => Ok(PenaltyType::ThisAndPreviousLapInvalidated),
        15 => Ok(PenaltyType::ThisAndPreviousLapInvalidatedWithoutReason),
        16 => Ok(PenaltyType::Retired),
        17 => Ok(PenaltyType::BlackFlagTimer),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid penalty type")),
    }
}
fn parse_infringement_type(value: u8) -> Result<InfringementType, Error> {
    match value {
        0 => Ok(InfringementType::BlockingBySlowDriving),
        1 => Ok(InfringementType::BlockingByWrongWayDriving),
        2 => Ok(InfringementType::ReversingOffTheStartLine),
        3 => Ok(InfringementType::BigCollision),
        4 => Ok(InfringementType::SmallCollision),
        5 => Ok(InfringementType::CollisionFailedToHandBackPositionSingle),
        6 => Ok(InfringementType::CollisionFailedToHandBackPositionMultiple),
        7 => Ok(InfringementType::CornerCuttingGainedTime),
        8 => Ok(InfringementType::CornerCuttingOvertakeSingle),
        9 => Ok(InfringementType::CornerCuttingOvertakeMultiple),
        10 => Ok(InfringementType::CrossedPitExitLane),
        11 => Ok(InfringementType::IgnoringBlueFlags),
        12 => Ok(InfringementType::IgnoringYellowFlags),
        13 => Ok(InfringementType::IgnoringDriveThrough),
        14 => Ok(InfringementType::TooManyDriveThroughs),
        15 => Ok(InfringementType::DriveThroughReminderServeWithinNLaps),
        16 => Ok(InfringementType::DriveThroughReminderServeThisLap),
        17 => Ok(InfringementType::PitLaneSpeeding),
        18 => Ok(InfringementType::ParkedForTooLong),
        19 => Ok(InfringementType::IgnoringTyreRegulations),
        20 => Ok(InfringementType::TooManyPenalties),
        21 => Ok(InfringementType::MultipleWarnings),
        22 => Ok(InfringementType::ApproachingDisqualification),
        23 => Ok(InfringementType::TyreRegulationsSelectSingle),
        24 => Ok(InfringementType::TyreRegulationsSelectMultiple),
        25 => Ok(InfringementType::LapInvalidatedCornerCutting),
        26 => Ok(InfringementType::LapInvalidatedRunningWide),
        27 => Ok(InfringementType::CornerCuttingRanWideGainedTimeMinor),
        28 => Ok(InfringementType::CornerCuttingRanWideGainedTimeSignificant),
        29 => Ok(InfringementType::CornerCuttingRanWideGainedTimeExtreme),
        30 => Ok(InfringementType::LapInvalidatedWallRiding),
        31 => Ok(InfringementType::LapInvalidatedFlashbackUsed),
        32 => Ok(InfringementType::LapInvalidatedResetToTrack),
        33 => Ok(InfringementType::BlockingThePitlane),
        34 => Ok(InfringementType::JumpStart),
        35 => Ok(InfringementType::SafetyCarToCarCollision),
        36 => Ok(InfringementType::SafetyCarIllegalOvertake),
        37 => Ok(InfringementType::SafetyCarExceedingAllowedPace),
        38 => Ok(InfringementType::VirtualSafetyCarExceedingAllowedPace),
        39 => Ok(InfringementType::FormationLapBelowAllowedSpeed),
        40 => Ok(InfringementType::RetiredMechanicalFailure),
        41 => Ok(InfringementType::RetiredTerminallyDamaged),
        42 => Ok(InfringementType::SafetyCarFallingTooFarBack),
        43 => Ok(InfringementType::BlackFlagTimer),
        44 => Ok(InfringementType::UnservedStopGoPenalty),
        45 => Ok(InfringementType::UnservedDriveThroughPenalty),
        46 => Ok(InfringementType::EngineComponentChange),
        47 => Ok(InfringementType::GearboxChange),
        48 => Ok(InfringementType::LeagueGridPenalty),
        49 => Ok(InfringementType::RetryPenalty),
        50 => Ok(InfringementType::IllegalTimeGain),
        51 => Ok(InfringementType::MandatoryPitstop),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid infringement type",
        )),
    }
}

fn ensure_event_size(size: usize) -> Result<(), Error> {
    if size == EVENT_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Event size is too small",
    ))
}
