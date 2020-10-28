use crate::f1_2020::car::{PacketCarSetupData, PacketCarStatusData, PacketCarTelemetryData};
use crate::f1_2020::car_setup::parse_car_setup_data;
use crate::f1_2020::car_status::parse_car_status_data;
use crate::f1_2020::car_telemetry::parse_car_telemetry_data;
use crate::f1_2020::event::{parse_event_data, PacketEventData};
use crate::f1_2020::final_classification::{
    parse_final_classification_data, PacketFinalClassificationData,
};
use crate::f1_2020::header::parse_headers;
use crate::f1_2020::lap::{parse_lap_data, PacketLapData};
use crate::f1_2020::lobby_info::{parse_lobby_info_data, PacketLobbyInfoData};
use crate::f1_2020::motion::{parse_motion_data, PacketMotionData};
use crate::f1_2020::participants::{parse_participants_data, PacketParticipantsData};
use crate::f1_2020::session::{parse_session, PacketSessionData};
use async_std::io::{Cursor, Error, ErrorKind};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PacketID {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Packet2020 {
    Motion(PacketMotionData),
    Session(PacketSessionData),
    Lap(PacketLapData),
    Event(PacketEventData),
    Participants(PacketParticipantsData),
    CarSetups(PacketCarSetupData),
    CarTelemetry(PacketCarTelemetryData),
    CarStatus(PacketCarStatusData),
    FinalClassification(PacketFinalClassificationData),
    LobbyInfo(PacketLobbyInfoData),
}

pub async fn parse_f12020(cursor: &mut Cursor<Vec<u8>>, size: usize) -> Result<Packet2020, Error> {
    let header = parse_headers(cursor, size).await?;
    match packet_type(header.packet_id)? {
        PacketID::Motion => {
            let motion = parse_motion_data(cursor, header, size).await?;
            Ok(Packet2020::Motion(motion))
        }
        PacketID::Session => {
            let session = parse_session(cursor, header, size).await?;
            Ok(Packet2020::Session(session))
        }
        PacketID::LapData => {
            let lap = parse_lap_data(cursor, header, size).await?;
            Ok(Packet2020::Lap(lap))
        }
        PacketID::Event => {
            let event = parse_event_data(cursor, header, size).await?;
            Ok(Packet2020::Event(event))
        }
        PacketID::Participants => {
            let participants = parse_participants_data(cursor, header, size).await?;
            Ok(Packet2020::Participants(participants))
        }
        PacketID::CarSetups => {
            let car_setups = parse_car_setup_data(cursor, header, size).await?;
            Ok(Packet2020::CarSetups(car_setups))
        }
        PacketID::CarTelemetry => {
            let car_telemetry = parse_car_telemetry_data(cursor, header, size).await?;
            Ok(Packet2020::CarTelemetry(car_telemetry))
        }
        PacketID::CarStatus => {
            let car_status = parse_car_status_data(cursor, header, size).await?;
            Ok(Packet2020::CarStatus(car_status))
        }
        PacketID::FinalClassification => {
            let final_classification =
                parse_final_classification_data(cursor, header, size).await?;
            Ok(Packet2020::FinalClassification(final_classification))
        }
        PacketID::LobbyInfo => {
            let lobby_info = parse_lobby_info_data(cursor, header, size).await?;
            Ok(Packet2020::LobbyInfo(lobby_info))
        }
    }
}

pub fn packet_type(packet_id: u8) -> Result<PacketID, Error> {
    match packet_id {
        0 => Ok(PacketID::Motion),
        1 => Ok(PacketID::Session),
        2 => Ok(PacketID::LapData),
        3 => Ok(PacketID::Event),
        4 => Ok(PacketID::Participants),
        5 => Ok(PacketID::CarSetups),
        6 => Ok(PacketID::CarTelemetry),
        7 => Ok(PacketID::CarStatus),
        8 => Ok(PacketID::FinalClassification),
        9 => Ok(PacketID::LobbyInfo),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to parse packet_type",
        )),
    }
}
