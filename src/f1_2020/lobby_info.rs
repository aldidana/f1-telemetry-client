use async_std::io::{Cursor, Error, ErrorKind};

use crate::f1_2020::car::TOTAL_CARS;
use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::nationality::Nationality;
use crate::f1_2020::participants::{parse_nationality, parse_team};
use crate::f1_2020::team::Team;
use byteorder_async::ReaderToByteOrder;

const LOBBY_INFO_MIN_SIZE: usize = 1169;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ReadyStatus {
    NotReady,
    Ready,
    Spectating,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LobbyInfoData {
    ai_controlled: bool,
    team: Team,
    nationality: Nationality,
    name: String,
    ready_status: ReadyStatus,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PacketLobbyInfoData {
    header: PacketHeader,
    num_players: u8,
    lobby_info_data: Vec<LobbyInfoData>,
}

pub async fn parse_lobby_info_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketLobbyInfoData, Error> {
    ensure_lobby_info_size(size)?;

    let num_players = cursor.byte_order().read_u8().await?;

    let mut lobby_info_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let data = parse_lobby_info(cursor).await?;
        lobby_info_data.push(data);
    }

    Ok(PacketLobbyInfoData {
        header,
        num_players,
        lobby_info_data,
    })
}

pub async fn parse_lobby_info(cursor: &mut Cursor<Vec<u8>>) -> Result<LobbyInfoData, Error> {
    let ai_controlled = cursor.byte_order().read_u8().await? == 1;
    let team = parse_team(cursor.byte_order().read_u8().await?)?;
    let nationality = parse_nationality(cursor.byte_order().read_u8().await?)?;
    let mut name: Vec<char> = Vec::with_capacity(48);
    for _ in 0..48 {
        name.push(cursor.byte_order().read_u8().await? as char);
    }

    let ready_status = parse_ready_status(cursor.byte_order().read_u8().await?)?;

    Ok(LobbyInfoData {
        ai_controlled,
        team,
        nationality,
        name: name.iter().collect(),
        ready_status,
    })
}

fn parse_ready_status(value: u8) -> Result<ReadyStatus, Error> {
    match value {
        0 => Ok(ReadyStatus::NotReady),
        1 => Ok(ReadyStatus::Ready),
        2 => Ok(ReadyStatus::Spectating),
        _ => Err(Error::new(ErrorKind::InvalidData, "Ready status invalid")),
    }
}

fn ensure_lobby_info_size(size: usize) -> Result<(), Error> {
    if size == LOBBY_INFO_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Lobby info size is too small",
    ))
}
