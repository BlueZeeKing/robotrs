// Device Identification Registers

pub const NAVX_REG_WHOAMI: u8 = 0x00;
/// IMU_MODEL_XXX
pub const NAVX_REG_HW_REV: u8 = 0x01;
pub const NAVX_REG_FW_VER_MAJOR: u8 = 0x02;
pub const NAVX_REG_FW_VER_MINOR: u8 = 0x03;

// Status and Control Registers

/// Read-write
pub const NAVX_REG_UPDATE_RATE_HZ: u8 = 0x04;
/// Range:  4 - 50 [unsigned byte]
/// Read-only
/// Accelerometer Full-Scale Range:  in units of G [unsigned byte]
pub const NAVX_REG_ACCEL_FSR_G: u8 = 0x05;
/// Gyro Full-Scale Range (Degrees/Sec):  Range:  250, 500, 1000 or 2000 [unsigned short]
pub const NAVX_REG_GYRO_FSR_DPS_L: u8 = 0x06;
/// Lower 8-bits of Gyro Full-Scale Range
pub const NAVX_REG_GYRO_FSR_DPS_H: u8 = 0x07;
/// Upper 8-bits of Gyro Full-Scale Range
pub const NAVX_REG_OP_STATUS: u8 = 0x08;
/// NAVX_OP_STATUS_XXX
pub const NAVX_REG_CAL_STATUS: u8 = 0x09;
/// NAVX_CAL_STATUS_XXX
pub const NAVX_REG_SELFTEST_STATUS: u8 = 0x0A;
/// NAVX_SELFTEST_STATUS_XXX
pub const NAVX_REG_CAPABILITY_FLAGS_L: u8 = 0x0B;
pub const NAVX_REG_CAPABILITY_FLAGS_H: u8 = 0x0C;

// Processed Data Registers

pub const NAVX_REG_SENSOR_STATUS_L: u8 = 0x10; // NAVX_SENSOR_STATUS_XXX */
pub const NAVX_REG_SENSOR_STATUS_H: u8 = 0x11;
/// Timestamp:  [unsigned long]
pub const NAVX_REG_TIMESTAMP_L_L: u8 = 0x12;
pub const NAVX_REG_TIMESTAMP_L_H: u8 = 0x13;
pub const NAVX_REG_TIMESTAMP_H_L: u8 = 0x14;
pub const NAVX_REG_TIMESTAMP_H_H: u8 = 0x15;

// Yaw, Pitch, Roll:  Range: -180.00 to 180.00 [signed hundredths]
// Compass Heading:   Range: 0.00 to 360.00 [unsigned hundredths]
// Altitude in Meters:  In units of meters [16:16]

pub const NAVX_REG_YAW_L: u8 = 0x16;
/// Lower 8 bits of Yaw
pub const NAVX_REG_YAW_H: u8 = 0x17;
/// Upper 8 bits of Yaw
pub const NAVX_REG_ROLL_L: u8 = 0x18;
/// Lower 8 bits of Roll
pub const NAVX_REG_ROLL_H: u8 = 0x19;
/// Upper 8 bits of Roll
pub const NAVX_REG_PITCH_L: u8 = 0x1A;
/// Lower 8 bits of Pitch
pub const NAVX_REG_PITCH_H: u8 = 0x1B;
/// Upper 8 bits of Pitch
pub const NAVX_REG_HEADING_L: u8 = 0x1C;
/// Lower 8 bits of Heading
pub const NAVX_REG_HEADING_H: u8 = 0x1D;
/// Upper 8 bits of Heading
pub const NAVX_REG_FUSED_HEADING_L: u8 = 0x1E;
/// Upper 8 bits of Fused Heading
pub const NAVX_REG_FUSED_HEADING_H: u8 = 0x1F;
/// Upper 8 bits of Fused Heading
pub const NAVX_REG_ALTITUDE_I_L: u8 = 0x20;
pub const NAVX_REG_ALTITUDE_I_H: u8 = 0x21;
pub const NAVX_REG_ALTITUDE_D_L: u8 = 0x22;
pub const NAVX_REG_ALTITUDE_D_H: u8 = 0x23;

// World-frame Linear Acceleration: In units of +/- G * 1000 [signed thousandths]

pub const NAVX_REG_LINEAR_ACC_X_L: u8 = 0x24;
/// Lower 8 bits of Linear Acceleration X
pub const NAVX_REG_LINEAR_ACC_X_H: u8 = 0x25;
/// Upper 8 bits of Linear Acceleration X
pub const NAVX_REG_LINEAR_ACC_Y_L: u8 = 0x26;
/// Lower 8 bits of Linear Acceleration Y
pub const NAVX_REG_LINEAR_ACC_Y_H: u8 = 0x27;
/// Upper 8 bits of Linear Acceleration Y
pub const NAVX_REG_LINEAR_ACC_Z_L: u8 = 0x28;
/// Lower 8 bits of Linear Acceleration Z
pub const NAVX_REG_LINEAR_ACC_Z_H: u8 = 0x29;
/// Upper 8 bits of Linear Acceleration Z

// Quaternion:  Range -1 to 1 [signed short ratio]

pub const NAVX_REG_QUAT_W_L: u8 = 0x2A;
/// Lower 8 bits of Quaternion W
pub const NAVX_REG_QUAT_W_H: u8 = 0x2B;
/// Upper 8 bits of Quaternion W
pub const NAVX_REG_QUAT_X_L: u8 = 0x2C;
/// Lower 8 bits of Quaternion X
pub const NAVX_REG_QUAT_X_H: u8 = 0x2D;
/// Upper 8 bits of Quaternion X
pub const NAVX_REG_QUAT_Y_L: u8 = 0x2E;
/// Lower 8 bits of Quaternion Y
pub const NAVX_REG_QUAT_Y_H: u8 = 0x2F;
/// Upper 8 bits of Quaternion Y
pub const NAVX_REG_QUAT_Z_L: u8 = 0x30;
/// Lower 8 bits of Quaternion Z
pub const NAVX_REG_QUAT_Z_H: u8 = 0x31;
/// Upper 8 bits of Quaternion Z

// Raw Data Registers

// Sensor Die Temperature:  Range +/- 150, In units of Centigrade * 100 [signed hundredths float

pub const NAVX_REG_MPU_TEMP_C_L: u8 = 0x32;
/// Lower 8 bits of Temperature
pub const NAVX_REG_MPU_TEMP_C_H: u8 = 0x33;
/// Upper 8 bits of Temperature

// Raw, Calibrated Angular Rotation, in device units.  Value in DPS = units / GYRO_FSR_DPS [signed short]

pub const NAVX_REG_GYRO_X_L: u8 = 0x34;
pub const NAVX_REG_GYRO_X_H: u8 = 0x35;
pub const NAVX_REG_GYRO_Y_L: u8 = 0x36;
pub const NAVX_REG_GYRO_Y_H: u8 = 0x37;
pub const NAVX_REG_GYRO_Z_L: u8 = 0x38;
pub const NAVX_REG_GYRO_Z_H: u8 = 0x39;

// Raw, Calibrated, Acceleration Data, in device units.  Value in G = units / ACCEL_FSR_G [signed short]

pub const NAVX_REG_ACC_X_L: u8 = 0x3A;
pub const NAVX_REG_ACC_X_H: u8 = 0x3B;
pub const NAVX_REG_ACC_Y_L: u8 = 0x3C;
pub const NAVX_REG_ACC_Y_H: u8 = 0x3D;
pub const NAVX_REG_ACC_Z_L: u8 = 0x3E;
pub const NAVX_REG_ACC_Z_H: u8 = 0x3F;

// Raw, Calibrated, Un-tilt corrected Magnetometer Data, in device units.  1 unit = 0.15 uTesla [signed short]

pub const NAVX_REG_MAG_X_L: u8 = 0x40;
pub const NAVX_REG_MAG_X_H: u8 = 0x41;
pub const NAVX_REG_MAG_Y_L: u8 = 0x42;
pub const NAVX_REG_MAG_Y_H: u8 = 0x43;
pub const NAVX_REG_MAG_Z_L: u8 = 0x44;
pub const NAVX_REG_MAG_Z_H: u8 = 0x45;

// Calibrated Pressure in millibars Valid Range:  10.00 Max:  1200.00 [16:16 float]

pub const NAVX_REG_PRESSURE_IL: u8 = 0x46;
pub const NAVX_REG_PRESSURE_IH: u8 = 0x47;
pub const NAVX_REG_PRESSURE_DL: u8 = 0x48;
pub const NAVX_REG_PRESSURE_DH: u8 = 0x49;

// Pressure Sensor Die Temperature:  Range +/- 150.00C [signed hundredths]

pub const NAVX_REG_PRESSURE_TEMP_L: u8 = 0x4A;
pub const NAVX_REG_PRESSURE_TEMP_H: u8 = 0x4B;

// Calibration Registers

// Yaw Offset: Range -180.00 to 180.00 [signed hundredths]

pub const NAVX_REG_YAW_OFFSET_L: u8 = 0x4C;
/// Lower 8 bits of Yaw Offset
pub const NAVX_REG_YAW_OFFSET_H: u8 = 0x4D;
/// Upper 8 bits of Yaw Offset

// Quaternion Offset:  Range: -1 to 1 [signed short ratio]

pub const NAVX_REG_QUAT_OFFSET_W_L: u8 = 0x4E;
/// Lower 8 bits of Quaternion W
pub const NAVX_REG_QUAT_OFFSET_W_H: u8 = 0x4F;
/// Upper 8 bits of Quaternion W
pub const NAVX_REG_QUAT_OFFSET_X_L: u8 = 0x50;
/// Lower 8 bits of Quaternion X
pub const NAVX_REG_QUAT_OFFSET_X_H: u8 = 0x51;
/// Upper 8 bits of Quaternion X
pub const NAVX_REG_QUAT_OFFSET_Y_L: u8 = 0x52;
/// Lower 8 bits of Quaternion Y
pub const NAVX_REG_QUAT_OFFSET_Y_H: u8 = 0x53;
/// Upper 8 bits of Quaternion Y
pub const NAVX_REG_QUAT_OFFSET_Z_L: u8 = 0x54;
/// Lower 8 bits of Quaternion Z
pub const NAVX_REG_QUAT_OFFSET_Z_H: u8 = 0x55;
/// Upper 8 bits of Quaternion Z

// Integrated Data Registers

// Integration Control (Write-Only)
pub const NAVX_REG_INTEGRATION_CTL: u8 = 0x56;
pub const NAVX_REG_PAD_UNUSED: u8 = 0x57;

// Velocity:  Range -32768.9999 - 32767.9999 in units of Meters/Sec

pub const NAVX_REG_VEL_X_I_L: u8 = 0x58;
pub const NAVX_REG_VEL_X_I_H: u8 = 0x59;
pub const NAVX_REG_VEL_X_D_L: u8 = 0x5A;
pub const NAVX_REG_VEL_X_D_H: u8 = 0x5B;
pub const NAVX_REG_VEL_Y_I_L: u8 = 0x5C;
pub const NAVX_REG_VEL_Y_I_H: u8 = 0x5D;
pub const NAVX_REG_VEL_Y_D_L: u8 = 0x5E;
pub const NAVX_REG_VEL_Y_D_H: u8 = 0x5F;
pub const NAVX_REG_VEL_Z_I_L: u8 = 0x60;
pub const NAVX_REG_VEL_Z_I_H: u8 = 0x61;
pub const NAVX_REG_VEL_Z_D_L: u8 = 0x62;
pub const NAVX_REG_VEL_Z_D_H: u8 = 0x63;

// Displacement:  Range -32768.9999 - 32767.9999 in units of Meters

pub const NAVX_REG_DISP_X_I_L: u8 = 0x64;
pub const NAVX_REG_DISP_X_I_H: u8 = 0x65;
pub const NAVX_REG_DISP_X_D_L: u8 = 0x66;
pub const NAVX_REG_DISP_X_D_H: u8 = 0x67;
pub const NAVX_REG_DISP_Y_I_L: u8 = 0x68;
pub const NAVX_REG_DISP_Y_I_H: u8 = 0x69;
pub const NAVX_REG_DISP_Y_D_L: u8 = 0x6A;
pub const NAVX_REG_DISP_Y_D_H: u8 = 0x6B;
pub const NAVX_REG_DISP_Z_I_L: u8 = 0x6C;
pub const NAVX_REG_DISP_Z_I_H: u8 = 0x6D;
pub const NAVX_REG_DISP_Z_D_L: u8 = 0x6E;
pub const NAVX_REG_DISP_Z_D_H: u8 = 0x6F;

pub const NAVX_REG_LAST: u8 = NAVX_REG_DISP_Z_D_H;
