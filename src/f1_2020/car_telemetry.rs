use crate::f1_2020::car::{
    CarTelemetryData, MFDPanel, PacketCarTelemetryData, SurfaceType, CAR_TELEMETRY_MIN_SIZE,
    TOTAL_CARS,
};
use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::motion::Wheel;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};

pub async fn parse_car_telemetry_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarTelemetryData, Error> {
    ensure_car_telemetry_size(size)?;

    let mut car_telemetry_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let ctd = parse_car_telemetry(cursor).await?;
        car_telemetry_data.push(ctd);
    }

    let button_status = cursor.byte_order().read_u32::<LittleEndian>().await?;
    let mfd_panel_index = parse_mfd_panel(cursor.byte_order().read_u8().await?)?;
    let mfd_panel_index_secondary_player = parse_mfd_panel(cursor.byte_order().read_u8().await?)?;
    let suggested_gear = cursor.byte_order().read_i8().await?;

    Ok(PacketCarTelemetryData {
        header,
        car_telemetry_data,
        button_status,
        mfd_panel_index,
        mfd_panel_index_secondary_player,
        suggested_gear,
    })
}

async fn parse_car_telemetry(cursor: &mut Cursor<Vec<u8>>) -> Result<CarTelemetryData, Error> {
    let speed = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let throttle = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let steer = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let brake = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let clutch = cursor.byte_order().read_u8().await?;
    let gear = cursor.byte_order().read_i8().await?;
    let engine_rpm = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let drs = cursor.byte_order().read_u8().await? == 1;
    let rev_lights_percent = cursor.byte_order().read_u8().await?;
    let brakes_temperature = Wheel {
        rear_left: cursor.byte_order().read_u16::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_u16::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_u16::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_u16::<LittleEndian>().await?,
    };
    let tyres_surface_temperature = Wheel {
        rear_left: cursor.byte_order().read_u8().await?,
        rear_right: cursor.byte_order().read_u8().await?,
        front_left: cursor.byte_order().read_u8().await?,
        front_right: cursor.byte_order().read_u8().await?,
    };
    let tyres_inner_temperature = Wheel {
        rear_left: cursor.byte_order().read_u8().await?,
        rear_right: cursor.byte_order().read_u8().await?,
        front_left: cursor.byte_order().read_u8().await?,
        front_right: cursor.byte_order().read_u8().await?,
    };
    let engine_temperature = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let tyre_pressures = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };
    let surface_types = Wheel {
        rear_left: parse_surface_type(cursor.byte_order().read_u8().await?)?,
        rear_right: parse_surface_type(cursor.byte_order().read_u8().await?)?,
        front_left: parse_surface_type(cursor.byte_order().read_u8().await?)?,
        front_right: parse_surface_type(cursor.byte_order().read_u8().await?)?,
    };

    Ok(CarTelemetryData {
        speed,
        throttle,
        steer,
        brake,
        clutch,
        gear,
        engine_rpm,
        drs,
        rev_lights_percent,
        brakes_temperature,
        tyres_surface_temperature,
        tyres_inner_temperature,
        engine_temperature,
        tyre_pressures,
        surface_types,
    })
}

fn ensure_car_telemetry_size(size: usize) -> Result<(), Error> {
    if size == CAR_TELEMETRY_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Car telemetry size is too small",
    ))
}

fn parse_surface_type(value: u8) -> Result<SurfaceType, Error> {
    match value {
        0 => Ok(SurfaceType::Tarmac),
        1 => Ok(SurfaceType::RumbleStrip),
        2 => Ok(SurfaceType::Concrete),
        3 => Ok(SurfaceType::Rock),
        4 => Ok(SurfaceType::Gravel),
        5 => Ok(SurfaceType::Mud),
        6 => Ok(SurfaceType::Sand),
        7 => Ok(SurfaceType::Grass),
        8 => Ok(SurfaceType::Water),
        9 => Ok(SurfaceType::Cobblestone),
        10 => Ok(SurfaceType::Metal),
        11 => Ok(SurfaceType::Ridged),
        12 => Ok(SurfaceType::Unknown),
        _ => Err(Error::new(ErrorKind::InvalidData, "Surface type invalid")),
    }
}

fn parse_mfd_panel(value: u8) -> Result<MFDPanel, Error> {
    match value {
        0 => Ok(MFDPanel::CarSetup),
        1 => Ok(MFDPanel::Pits),
        2 => Ok(MFDPanel::Damage),
        3 => Ok(MFDPanel::Engine),
        4 => Ok(MFDPanel::Temperatures),
        255 => Ok(MFDPanel::Closed),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "MFD panel index invalid",
        )),
    }
}
