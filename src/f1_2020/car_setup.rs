use crate::f1_2020::car::{
    CarSetupData, PacketCarSetupData, TyrePressure, CAR_SETUP_MIN_SIZE, TOTAL_CARS,
};
use crate::f1_2020::header::PacketHeader;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};

pub async fn parse_car_setup_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarSetupData, Error> {
    ensure_car_setup_size(size)?;

    let mut car_setup_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let csd = parse_car_setup(cursor).await?;
        car_setup_data.push(csd);
    }

    Ok(PacketCarSetupData {
        header,
        car_setup_data,
    })
}

async fn parse_car_setup(cursor: &mut Cursor<Vec<u8>>) -> Result<CarSetupData, Error> {
    let front_wing = cursor.byte_order().read_u8().await?;
    let rear_wing = cursor.byte_order().read_u8().await?;
    let on_throttle = cursor.byte_order().read_u8().await?;
    let off_throttle = cursor.byte_order().read_u8().await?;
    let front_camber = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let rear_camber = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let front_toe = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let rear_toe = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let front_suspension = cursor.byte_order().read_u8().await?;
    let rear_suspension = cursor.byte_order().read_u8().await?;
    let front_anti_roll_bar = cursor.byte_order().read_u8().await?;
    let rear_anti_roll_bar = cursor.byte_order().read_u8().await?;
    let front_suspension_height = cursor.byte_order().read_u8().await?;
    let rear_suspension_height = cursor.byte_order().read_u8().await?;
    let brake_pressure = cursor.byte_order().read_u8().await?;
    let brake_bias = cursor.byte_order().read_u8().await?;
    let rear_tyre_pressure = TyrePressure {
        left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };
    let front_tyre_pressure = TyrePressure {
        left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };
    let ballast = cursor.byte_order().read_u8().await?;
    let fuel_load = cursor.byte_order().read_f32::<LittleEndian>().await?;

    Ok(CarSetupData {
        front_wing,
        rear_wing,
        on_throttle,
        off_throttle,
        front_camber,
        rear_camber,
        front_toe,
        rear_toe,
        front_suspension,
        rear_suspension,
        front_anti_roll_bar,
        rear_anti_roll_bar,
        front_suspension_height,
        rear_suspension_height,
        brake_pressure,
        brake_bias,
        rear_tyre_pressure,
        front_tyre_pressure,
        ballast,
        fuel_load,
    })
}

fn ensure_car_setup_size(size: usize) -> Result<(), Error> {
    if size == CAR_SETUP_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Car setup size is too small",
    ))
}
