use crate::f1_2020::car::TOTAL_CARS;
use crate::f1_2020::header::PacketHeader;
use async_std::io::{Cursor, Error, ErrorKind};
use byteorder_async::{LittleEndian, ReaderToByteOrder};

use derivative::Derivative;

const MOTION_MIN_SIZE: usize = 1464;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CarMotionData {
    pub world_position_x: f32,
    pub world_position_y: f32,
    pub world_position_z: f32,
    pub world_velocity_x: f32,
    pub world_velocity_y: f32,
    pub world_velocity_z: f32,
    pub world_forward_dir_x: i16,
    pub world_forward_dir_y: i16,
    pub world_forward_dir_z: i16,
    pub world_right_dir_x: i16,
    pub world_right_dir_y: i16,
    pub world_right_dir_z: i16,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketMotionData {
    pub header: PacketHeader,
    pub motion_data: Vec<CarMotionData>,
    pub suspension_position: Wheel<f32>,
    pub suspension_velocity: Wheel<f32>,
    pub suspension_acceleration: Wheel<f32>,
    pub wheel_speed: Wheel<f32>,
    pub wheel_slip: Wheel<f32>,
    pub local_velocity_x: f32,
    pub local_velocity_y: f32,
    pub local_velocity_z: f32,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub angular_acceleration_x: f32,
    pub angular_acceleration_y: f32,
    pub angular_acceleration_z: f32,
    pub front_wheels_angle: f32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Wheel<T>
where
    T: Copy + Clone,
{
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

pub async fn parse_motion_data(
    cursor: &mut Cursor<Vec<u8>>,
    header: PacketHeader,
    size: usize,
) -> Result<PacketMotionData, Error> {
    ensure_motion_size(size)?;

    let mut car_motion_data = Vec::with_capacity(TOTAL_CARS);
    for _ in 0..TOTAL_CARS {
        let car_motion = parse_car_motion(cursor).await?;
        car_motion_data.push(car_motion);
    }

    let suspension_position = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };

    let suspension_velocity = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };

    let suspension_acceleration = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };

    let wheel_speed = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };

    let wheel_slip = Wheel {
        rear_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        rear_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_left: cursor.byte_order().read_f32::<LittleEndian>().await?,
        front_right: cursor.byte_order().read_f32::<LittleEndian>().await?,
    };

    let local_velocity_x = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let local_velocity_y = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let local_velocity_z = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_velocity_x = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_velocity_y = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_velocity_z = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_acceleration_x = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_acceleration_y = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let angular_acceleration_z = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let front_wheels_angle = cursor.byte_order().read_f32::<LittleEndian>().await?;

    Ok(PacketMotionData {
        header,
        motion_data: car_motion_data,
        suspension_position,
        suspension_velocity,
        suspension_acceleration,
        wheel_speed,
        wheel_slip,
        local_velocity_x,
        local_velocity_y,
        local_velocity_z,
        angular_velocity_x,
        angular_velocity_y,
        angular_velocity_z,
        angular_acceleration_x,
        angular_acceleration_y,
        angular_acceleration_z,
        front_wheels_angle,
    })
}

async fn parse_car_motion(cursor: &mut Cursor<Vec<u8>>) -> Result<CarMotionData, Error> {
    let world_position_x = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_position_y = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_position_z = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_velocity_x = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_velocity_y = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_velocity_z = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let world_forward_dir_x = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let world_forward_dir_y = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let world_forward_dir_z = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let world_right_dir_x = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let world_right_dir_y = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let world_right_dir_z = cursor.byte_order().read_i16::<LittleEndian>().await?;
    let g_force_lateral = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let g_force_longitudinal = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let g_force_vertical = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let yaw = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let pitch = cursor.byte_order().read_f32::<LittleEndian>().await?;
    let roll = cursor.byte_order().read_f32::<LittleEndian>().await?;

    Ok(CarMotionData {
        world_position_x,
        world_position_y,
        world_position_z,
        world_velocity_x,
        world_velocity_y,
        world_velocity_z,
        world_forward_dir_x,
        world_forward_dir_y,
        world_forward_dir_z,
        world_right_dir_x,
        world_right_dir_y,
        world_right_dir_z,
        g_force_lateral,
        g_force_longitudinal,
        g_force_vertical,
        yaw,
        pitch,
        roll,
    })
}

fn ensure_motion_size(size: usize) -> Result<(), Error> {
    if size < MOTION_MIN_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Motion size is too small",
        ));
    }

    return Ok(());
}
