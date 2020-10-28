use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub major_version: u8,
    pub minor_version: u8,
    pub packet_version: u8,
    pub packet_id: u8,
    pub session_uid: u64,
    pub session_time: Duration,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

const HEADER_MIN_SIZE: usize = 24;

pub async fn parse_headers(
    cursor: &mut Cursor<Vec<u8>>,
    size: usize,
) -> Result<PacketHeader, Error> {
    ensure_header_size(size)?;

    let packet_format = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let major_version = cursor.byte_order().read_u8().await?;
    let minor_version = cursor.byte_order().read_u8().await?;
    let packet_version = cursor.byte_order().read_u8().await?;
    let packet_id = cursor.byte_order().read_u8().await?;
    let session_uid = cursor.byte_order().read_u64::<LittleEndian>().await?;
    let session_time =
        Duration::from_secs_f32(cursor.byte_order().read_f32::<LittleEndian>().await?);
    let frame_identifier = cursor.byte_order().read_u32::<LittleEndian>().await?;
    let player_car_index = cursor.byte_order().read_u8().await?;
    let secondary_player_car_index = cursor.byte_order().read_u8().await?;

    Ok(PacketHeader {
        packet_format,
        major_version,
        minor_version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
        secondary_player_car_index,
    })
}

fn ensure_header_size(size: usize) -> Result<(), Error> {
    if size < HEADER_MIN_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Header size is too small",
        ));
    }

    return Ok(());
}

#[cfg(test)]
mod test {
    use crate::f1_2020::header::parse_headers;
    use async_std::io::Cursor;

    use byteorder_async::{LittleEndian, WriteBytesExt};

    #[async_std::test]
    async fn parse_header_error() {
        let mut buf = Vec::with_capacity(1);
        buf.write_u16::<LittleEndian>(0).unwrap();

        let mut cursor = Cursor::new(buf);
        let len = cursor.get_ref().len();

        let result = parse_headers(&mut cursor, len).await.map_err(|e| e.kind());
        assert_eq!(result, Err(async_std::io::ErrorKind::InvalidData));
    }

    #[async_std::test]
    async fn parse_header_success() {
        let mut buf = Vec::with_capacity(2048);
        buf.write_u16::<LittleEndian>(2020).unwrap();
        buf.write_u8(1).unwrap();
        buf.write_u8(2).unwrap();
        buf.write_u8(3).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u64::<LittleEndian>(u64::max_value()).unwrap();
        buf.write_f32::<LittleEndian>(1.0).unwrap();
        buf.write_u32::<LittleEndian>(u32::max_value()).unwrap();
        buf.write_u8(19).unwrap();
        buf.write_u8(255).unwrap();

        let mut cursor = Cursor::new(buf);
        let len = cursor.get_mut().len();
        let result = parse_headers(&mut cursor, len).await.unwrap();

        assert_eq!(2020, result.packet_format);
        assert_eq!(1, result.major_version);
        assert_eq!(2, result.minor_version);
        assert_eq!(3, result.packet_version);
        assert_eq!(0, result.packet_id);
        assert_eq!(u64::max_value(), result.session_uid);
        assert_eq!(1, result.session_time.as_secs());
        assert_eq!(1000, result.session_time.as_millis());
        assert_eq!(u32::max_value(), result.frame_identifier);
        assert_eq!(19, result.player_car_index);
        assert_eq!(255, result.secondary_player_car_index);
    }
}
