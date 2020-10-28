use crate::f1_2020::car::{
    ActualTyreCompound, AntiLockBrakes, CarStatusData, DRSStatus, ERSDeploymentMode, FuelMix,
    PacketCarStatusData, TractionControl, VisualTyreCompound, CAR_STATUS_MIN_SIZE, TOTAL_CARS,
};
use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::motion::Wheel;
use crate::f1_2020::session::parse_flag;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};

pub async fn parse_car_status_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarStatusData, Error> {
    ensure_car_status_size(size)?;

    let mut car_status_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let csd = parse_car_status(cursor).await?;
        car_status_data.push(csd);
    }

    Ok(PacketCarStatusData {
        header,
        car_status_data,
    })
}

async fn parse_car_status(cursor: &mut Cursor<Vec<u8>>) -> Result<CarStatusData, Error> {
    let traction_control = parse_traction_control(cursor.byte_order().read_u8().await?)?;
    let anti_lock_brakes = parse_anti_lock_brakes(cursor.byte_order().read_u8().await?)?;
    let fuel_mix = parse_fuel_mix(cursor.byte_order().read_u8().await?)?;
    let front_brake_bias = cursor.byte_order().read_u8().await?;
    let pit_limiter = cursor.byte_order().read_u8().await? == 1;
    let fuel_in_tank = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let fuel_capacity = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let fuel_remaining_laps = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let max_rpm = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let idle_rpm = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let max_gears = cursor.byte_order().read_u8().await?;
    let drs_allowed = parse_drs(cursor.byte_order().read_i8().await?)?;
    let drs_activation_distance = cursor.byte_order().read_u16::<LittleEndian>().await?;
    let tyres_wear = Wheel {
        rear_left: cursor.byte_order().read_u8().await?,
        rear_right: cursor.byte_order().read_u8().await?,
        front_left: cursor.byte_order().read_u8().await?,
        front_right: cursor.byte_order().read_u8().await?,
    };
    let actual_tyre_compound = parse_actual_tyre_compound(cursor.byte_order().read_u8().await?)?;
    let visual_tyre_compound = parse_visual_tyre_compound(cursor.byte_order().read_u8().await?)?;
    let tyres_age_laps = cursor.byte_order().read_u8().await?;
    let tyres_damage = Wheel {
        rear_left: cursor.byte_order().read_u8().await?,
        rear_right: cursor.byte_order().read_u8().await?,
        front_left: cursor.byte_order().read_u8().await?,
        front_right: cursor.byte_order().read_u8().await?,
    };
    let front_left_wing_damage = cursor.byte_order().read_u8().await?;
    let front_right_wing_damage = cursor.byte_order().read_u8().await?;
    let rear_wing_damage = cursor.byte_order().read_u8().await?;
    let drs_fault = cursor.byte_order().read_u8().await? == 1;
    let engine_damage = cursor.byte_order().read_u8().await?;
    let gear_box_damage = cursor.byte_order().read_u8().await?;
    let vehicle_fia_flags = parse_flag(cursor.byte_order().read_i8().await?)?;
    let ers_store_energy = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let ers_deploy_mode = parse_ers_deployment_mode(cursor.byte_order().read_u8().await?)?;
    let ers_harvested_this_lap_mguk = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let ers_harvested_this_lap_mguh = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let ers_deployed_this_lap = cursor.byte_order().read_f32::<LittleEndian>().await?;

    Ok(CarStatusData {
        traction_control,
        anti_lock_brakes,
        fuel_mix,
        front_brake_bias,
        pit_limiter,
        fuel_in_tank,
        fuel_capacity,
        fuel_remaining_laps,
        max_rpm,
        idle_rpm,
        max_gears,
        drs_allowed,
        drs_activation_distance,
        tyres_wear,
        actual_tyre_compound,
        visual_tyre_compound,
        tyres_age_laps,
        tyres_damage,
        front_left_wing_damage,
        front_right_wing_damage,
        rear_wing_damage,
        drs_fault,
        engine_damage,
        gear_box_damage,
        vehicle_fia_flags,
        ers_store_energy,
        ers_deploy_mode,
        ers_harvested_this_lap_mguk,
        ers_harvested_this_lap_mguh,
        ers_deployed_this_lap,
    })
}

fn ensure_car_status_size(size: usize) -> Result<(), Error> {
    if size == CAR_STATUS_MIN_SIZE {
        return Ok(());
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Car status size is too small",
    ))
}

fn parse_traction_control(value: u8) -> Result<TractionControl, Error> {
    match value {
        0 => Ok(TractionControl::Off),
        1 => Ok(TractionControl::Low),
        2 => Ok(TractionControl::High),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Traction control invalid",
        )),
    }
}

fn parse_fuel_mix(value: u8) -> Result<FuelMix, Error> {
    match value {
        0 => Ok(FuelMix::Lean),
        1 => Ok(FuelMix::Standard),
        2 => Ok(FuelMix::Rich),
        3 => Ok(FuelMix::Max),
        _ => Err(Error::new(ErrorKind::InvalidData, "Fuel mix invalid")),
    }
}

fn parse_drs(value: i8) -> Result<DRSStatus, Error> {
    match value {
        0 => Ok(DRSStatus::NotAllowed),
        1 => Ok(DRSStatus::Allowed),
        -1 => Ok(DRSStatus::Unknown),
        _ => Err(Error::new(ErrorKind::InvalidData, "DRS status invalid")),
    }
}

fn parse_ers_deployment_mode(value: u8) -> Result<ERSDeploymentMode, Error> {
    match value {
        0 => Ok(ERSDeploymentMode::None),
        1 => Ok(ERSDeploymentMode::Medium),
        2 => Ok(ERSDeploymentMode::Overtake),
        3 => Ok(ERSDeploymentMode::Hotlap),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "ERS Deployment mode invalid",
        )),
    }
}

pub fn parse_actual_tyre_compound(value: u8) -> Result<ActualTyreCompound, Error> {
    match value {
        16 => Ok(ActualTyreCompound::C5),
        17 => Ok(ActualTyreCompound::C4),
        18 => Ok(ActualTyreCompound::C3),
        19 => Ok(ActualTyreCompound::C2),
        20 => Ok(ActualTyreCompound::C1),
        7 => Ok(ActualTyreCompound::Inter),
        8 => Ok(ActualTyreCompound::Wet),
        9 => Ok(ActualTyreCompound::F1ClassicDry),
        10 => Ok(ActualTyreCompound::F1ClassicWet),
        11 => Ok(ActualTyreCompound::F2SuperSoft),
        12 => Ok(ActualTyreCompound::F2Soft),
        13 => Ok(ActualTyreCompound::F2Medium),
        14 => Ok(ActualTyreCompound::F2Hard),
        15 => Ok(ActualTyreCompound::F2Wet),
        0 | 255 => Ok(ActualTyreCompound::Unknown),
        _ => Err(Error::new(ErrorKind::InvalidData, "Tyre compound invalid")),
    }
}

pub fn parse_visual_tyre_compound(value: u8) -> Result<VisualTyreCompound, Error> {
    match value {
        16 => Ok(VisualTyreCompound::Soft),
        17 => Ok(VisualTyreCompound::Medium),
        18 => Ok(VisualTyreCompound::Hard),
        7 => Ok(VisualTyreCompound::Inter),
        8 => Ok(VisualTyreCompound::Wet),
        9 => Ok(VisualTyreCompound::F1ClassicDry),
        10 => Ok(VisualTyreCompound::F1ClassicWet),
        11 => Ok(VisualTyreCompound::F2SuperSoft),
        12 => Ok(VisualTyreCompound::F2Soft),
        13 => Ok(VisualTyreCompound::F2Medium),
        14 => Ok(VisualTyreCompound::F2Hard),
        15 => Ok(VisualTyreCompound::F2Wet),
        0 => Ok(VisualTyreCompound::Unknown),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Tyre compound visual invalid",
        )),
    }
}

pub fn parse_anti_lock_brakes(value: u8) -> Result<AntiLockBrakes, Error> {
    match value {
        0 => Ok(AntiLockBrakes::Off),
        1 => Ok(AntiLockBrakes::On),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Anti lock brakes invalid",
        )),
    }
}
