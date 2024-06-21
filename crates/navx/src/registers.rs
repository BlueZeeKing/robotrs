// Device Identification Registers

pub const NAVX_REG_WHOAMI: usize = 0x00;
/// IMU_MODEL_XXX
pub const NAVX_REG_HW_REV: usize = 0x01;
pub const NAVX_REG_FW_VER_MAJOR: usize = 0x02;
pub const NAVX_REG_FW_VER_MINOR: usize = 0x03;

// Status and Control Registers

/// Read-write
pub const NAVX_REG_UPDATE_RATE_HZ: usize = 0x04;
/// Range:  4 - 50 [unsigned byte]
/// Read-only
/// Accelerometer Full-Scale Range:  in units of G [unsigned byte]
pub const NAVX_REG_ACCEL_FSR_G: usize = 0x05;
/// Gyro Full-Scale Range (Degrees/Sec):  Range:  250, 500, 1000 or 2000 [unsigned short]
pub const NAVX_REG_GYRO_FSR_DPS_L: usize = 0x06;
/// Lower 8-bits of Gyro Full-Scale Range
pub const NAVX_REG_OP_STATUS: usize = 0x08;
/// NAVX_OP_STATUS_XXX
pub const NAVX_REG_CAL_STATUS: usize = 0x09;
/// NAVX_CAL_STATUS_XXX
pub const NAVX_REG_SELFTEST_STATUS: usize = 0x0A;
/// NAVX_SELFTEST_STATUS_XXX
pub const NAVX_REG_CAPABILITY_FLAGS_L: usize = 0x0B;

// Processed Data Registers

pub const NAVX_REG_SENSOR_STATUS_L: usize = 0x10; // NAVX_SENSOR_STATUS_XXX */
pub const NAVX_REG_SENSOR_STATUS_H: usize = 0x11;

// Yaw, Pitch, Roll:  Range: -180.00 to 180.00 [signed hundredths]
// Compass Heading:   Range: 0.00 to 360.00 [unsigned hundredths]
// Altitude in Meters:  In units of meters [16:16]

pub const NAVX_REG_YAW_L: usize = 0x16;
/// Upper 8 bits of Yaw
pub const NAVX_REG_ROLL_L: usize = 0x18;
/// Upper 8 bits of Roll
pub const NAVX_REG_PITCH_L: usize = 0x1A;
/// Upper 8 bits of Pitch
pub const NAVX_REG_HEADING_L: usize = 0x1C;
/// Upper 8 bits of Heading
pub const NAVX_REG_FUSED_HEADING_L: usize = 0x1E;
/// Upper 8 bits of Fused Heading
pub const NAVX_REG_ALTITUDE_D_L: usize = 0x22;

// World-frame Linear Acceleration: In units of +/- G * 1000 [signed thousandths]

pub const NAVX_REG_LINEAR_ACC_X_L: usize = 0x24;
/// Lower 8 bits of Linear Acceleration X
pub const NAVX_REG_LINEAR_ACC_Y_L: usize = 0x26;
/// Lower 8 bits of Linear Acceleration Y
pub const NAVX_REG_LINEAR_ACC_Z_L: usize = 0x28;
/// Lower 8 bits of Linear Acceleration Z

// Quaternion:  Range -1 to 1 [signed short ratio]

pub const NAVX_REG_QUAT_W_L: usize = 0x2A;
/// Lower 8 bits of Quaternion W
pub const NAVX_REG_QUAT_X_L: usize = 0x2C;
/// Lower 8 bits of Quaternion X
pub const NAVX_REG_QUAT_Y_L: usize = 0x2E;
/// Lower 8 bits of Quaternion Y
pub const NAVX_REG_QUAT_Z_L: usize = 0x30;
/// Lower 8 bits of Quaternion Z

// Raw Data Registers

// Sensor Die Temperature:  Range +/- 150, In units of Centigrade * 100 [signed hundredths float

pub const NAVX_REG_MPU_TEMP_C_L: usize = 0x32;
/// Lower 8 bits of Temperature

// Raw, Calibrated Angular Rotation, in device units.  Value in DPS = units / GYRO_FSR_DPS [signed short]

pub const NAVX_REG_GYRO_X_L: usize = 0x34;
pub const NAVX_REG_GYRO_Y_L: usize = 0x36;
pub const NAVX_REG_GYRO_Z_L: usize = 0x38;

// Raw, Calibrated, Acceleration Data, in device units.  Value in G = units / ACCEL_FSR_G [signed short]

pub const NAVX_REG_ACC_X_L: usize = 0x3A;
pub const NAVX_REG_ACC_Y_L: usize = 0x3C;
pub const NAVX_REG_ACC_Z_L: usize = 0x3E;

pub const NAVX_REG_MAG_X_L: usize = 0x40;
pub const NAVX_REG_MAG_Y_L: usize = 0x42;
pub const NAVX_REG_MAG_Z_L: usize = 0x44;

pub const NAVX_REG_PRESSURE_DL: usize = 0x48;

pub const NAVX_REG_QUAT_OFFSET_Z_H: usize = 0x55;

pub const NAVX_REG_VEL_X_I_L: usize = 0x58;
pub const NAVX_REG_VEL_Y_I_L: usize = 0x5C;
pub const NAVX_REG_VEL_Z_I_L: usize = 0x60;

pub const NAVX_REG_DISP_X_I_L: usize = 0x64;
pub const NAVX_REG_DISP_Y_I_L: usize = 0x68;
pub const NAVX_REG_DISP_Z_I_L: usize = 0x6C;
pub const NAVX_REG_DISP_Z_D_H: usize = 0x6F;

pub const NAVX_REG_LAST: usize = NAVX_REG_DISP_Z_D_H;
