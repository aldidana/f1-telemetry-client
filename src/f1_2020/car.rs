use crate::f1_2020::header::PacketHeader;
use crate::f1_2020::motion::Wheel;
use crate::f1_2020::session::ZoneFlag;

pub const TOTAL_CARS: usize = 22;
pub const CAR_SETUP_MIN_SIZE: usize = 1102;
pub const CAR_STATUS_MIN_SIZE: usize = 1344;
pub const CAR_TELEMETRY_MIN_SIZE: usize = 1307;

use derivative::Derivative;

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct TyrePressure {
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct CarSetupData {
    pub front_wing: u8,
    pub rear_wing: u8,
    pub on_throttle: u8,
    pub off_throttle: u8,
    pub front_camber: f32,
    pub rear_camber: f32,
    pub front_toe: f32,
    pub rear_toe: f32,
    pub front_suspension: u8,
    pub rear_suspension: u8,
    pub front_anti_roll_bar: u8,
    pub rear_anti_roll_bar: u8,
    pub front_suspension_height: u8,
    pub rear_suspension_height: u8,
    pub brake_pressure: u8,
    pub brake_bias: u8,
    pub rear_tyre_pressure: TyrePressure,
    pub front_tyre_pressure: TyrePressure,
    pub ballast: u8,
    pub fuel_load: f32,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    pub car_setup_data: Vec<CarSetupData>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SurfaceType {
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MFDPanel {
    CarSetup = 0,
    Pits = 1,
    Damage = 2,
    Engine = 3,
    Temperatures = 4,
    Closed = 255,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub brakes_temperature: Wheel<u16>,
    pub tyres_surface_temperature: Wheel<u8>,
    pub tyres_inner_temperature: Wheel<u8>,
    pub engine_temperature: u16,
    pub tyre_pressures: Wheel<f32>,
    pub surface_types: Wheel<SurfaceType>,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader,
    pub car_telemetry_data: Vec<CarTelemetryData>,
    pub button_status: u32,
    pub mfd_panel_index: MFDPanel,
    pub mfd_panel_index_secondary_player: MFDPanel,
    pub suggested_gear: i8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TractionControl {
    Off,
    Low,
    High,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AntiLockBrakes {
    Off,
    On,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FuelMix {
    Lean = 0,
    Standard = 1,
    Rich = 2,
    Max = 3,
}

impl FuelMix {
    pub fn to_string(&self) -> &'static str {
        match self {
            FuelMix::Lean => "Lean",
            FuelMix::Standard => "Standard",
            FuelMix::Rich => "Rich",
            FuelMix::Max => "Max",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DRSStatus {
    NotAllowed = 0,
    Allowed = 1,
    Unknown = 2,
}

impl DRSStatus {
    pub fn to_string(&self) -> &'static str {
        match self {
            DRSStatus::NotAllowed => "Not Allowed",
            DRSStatus::Allowed => "Allowed",
            DRSStatus::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ERSDeploymentMode {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Overtake = 4,
    Hotlap = 5,
}

impl ERSDeploymentMode {
    pub fn to_string(&self) -> &'static str {
        match self {
            ERSDeploymentMode::None => "None",
            ERSDeploymentMode::Low => "Low",
            ERSDeploymentMode::Medium => "Medium",
            ERSDeploymentMode::High => "High",
            ERSDeploymentMode::Overtake => "Overtake",
            ERSDeploymentMode::Hotlap => "Hotlap",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActualTyreCompound {
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
    Inter = 7,
    Wet = 8,
    F1ClassicDry = 9,
    F1ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VisualTyreCompound {
    Soft = 16,
    Medium = 17,
    Hard = 18,
    Inter = 7,
    Wet = 8,
    F1ClassicDry = 9,
    F1ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F2Wet = 15,
    Unknown = 0,
}

impl VisualTyreCompound {
    pub fn to_string(&self) -> &'static str {
        match self {
            VisualTyreCompound::Soft => "Soft",
            VisualTyreCompound::Medium => "Medium",
            VisualTyreCompound::Hard => "Hard",
            VisualTyreCompound::Inter => "Inter",
            VisualTyreCompound::Wet => "Wet",
            VisualTyreCompound::F1ClassicDry => "Dry",
            VisualTyreCompound::F1ClassicWet => "Wet",
            VisualTyreCompound::F2SuperSoft => "Soft",
            VisualTyreCompound::F2Soft => "Soft",
            VisualTyreCompound::F2Medium => "Medium",
            VisualTyreCompound::F2Hard => "Hard",
            VisualTyreCompound::F2Wet => "Wet",
            VisualTyreCompound::Unknown => "[N/A]",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct CarStatusData {
    pub traction_control: TractionControl,
    pub anti_lock_brakes: AntiLockBrakes,
    pub fuel_mix: FuelMix,
    pub front_brake_bias: u8,
    pub pit_limiter: bool,
    pub fuel_in_tank: f32,
    pub fuel_capacity: f32,
    pub fuel_remaining_laps: f32,
    pub max_rpm: u16,
    pub idle_rpm: u16,
    pub max_gears: u8,
    pub drs_allowed: DRSStatus,
    pub drs_activation_distance: u16,
    pub tyres_wear: Wheel<u8>,
    pub actual_tyre_compound: ActualTyreCompound,
    pub visual_tyre_compound: VisualTyreCompound,
    pub tyres_age_laps: u8,
    pub tyres_damage: Wheel<u8>,
    pub front_left_wing_damage: u8,
    pub front_right_wing_damage: u8,
    pub rear_wing_damage: u8,
    pub drs_fault: bool,
    pub engine_damage: u8,
    pub gear_box_damage: u8,
    pub vehicle_fia_flags: ZoneFlag,
    pub ers_store_energy: f32,
    pub ers_deploy_mode: ERSDeploymentMode,
    pub ers_harvested_this_lap_mguk: f32,
    pub ers_harvested_this_lap_mguh: f32,
    pub ers_deployed_this_lap: f32,
}

#[derive(Debug, PartialEq, Clone, Derivative)]
#[derivative(Eq)]
pub struct PacketCarStatusData {
    pub header: PacketHeader,
    pub car_status_data: Vec<CarStatusData>,
}
