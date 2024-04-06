use std::str;

pub const NAVX_CAL_STATUS_IMU_CAL_STATE_MASK: u8 = 0x03;
pub const NAVX_CAL_STATUS_IMU_CAL_INPROGRESS: u8 = 0x00;
pub const NAVX_CAL_STATUS_IMU_CAL_ACCUMULATE: u8 = 0x01;
pub const NAVX_CAL_STATUS_IMU_CAL_COMPLETE: u8 = 0x02;

pub const NAVX_CAL_STATUS_MAG_CAL_COMPLETE: u8 = 0x04;
pub const NAVX_CAL_STATUS_BARO_CAL_COMPLETE: u8 = 0x08;

/* NAVX_SELFTEST_STATUS */

pub const NAVX_SELFTEST_STATUS_COMPLETE: u8 = 0x80;

pub const NAVX_SELFTEST_RESULT_GYRO_PASSED: u8 = 0x01;
pub const NAVX_SELFTEST_RESULT_ACCEL_PASSED: u8 = 0x02;
pub const NAVX_SELFTEST_RESULT_MAG_PASSED: u8 = 0x04;
pub const NAVX_SELFTEST_RESULT_BARO_PASSED: u8 = 0x08;

/* NAVX_OP_STATUS */

pub const NAVX_OP_STATUS_INITIALIZING: u8 = 0x00;
pub const NAVX_OP_STATUS_SELFTEST_IN_PROGRESS: u8 = 0x01;
pub const NAVX_OP_STATUS_ERROR: u8 = 0x02;
pub const NAVX_OP_STATUS_IMU_AUTOCAL_IN_PROGRESS: u8 = 0x03;
pub const NAVX_OP_STATUS_NORMAL: u8 = 0x04;

/* NAVX_SENSOR_STATUS */
pub const NAVX_SENSOR_STATUS_MOVING: u8 = 0x01;
pub const NAVX_SENSOR_STATUS_YAW_STABLE: u8 = 0x02;
pub const NAVX_SENSOR_STATUS_MAG_DISTURBANCE: u8 = 0x04;
pub const NAVX_SENSOR_STATUS_ALTITUDE_VALID: u8 = 0x08;
pub const NAVX_SENSOR_STATUS_SEALEVEL_PRESS_SET: u8 = 0x10;
pub const NAVX_SENSOR_STATUS_FUSED_HEADING_VALID: u8 = 0x20;

/* NAVX_REG_CAPABILITY_FLAGS (Aligned w/NAV6 Flags, see IMUProtocol.h) */

pub const NAVX_CAPABILITY_FLAG_OMNIMOUNT: i16 = 0x0004;
pub const NAVX_CAPABILITY_FLAG_OMNIMOUNT_CONFIG_MASK: i16 = 0x0038;
pub const NAVX_CAPABILITY_FLAG_VEL_AND_DISP: i16 = 0x0040;
pub const NAVX_CAPABILITY_FLAG_YAW_RESET: i16 = 0x0080;
pub const NAVX_CAPABILITY_FLAG_AHRSPOS_TS: i16 = 0x0100;
pub const NAVX_CAPABILITY_FLAG_AHRSPOS_TS_RAW: i16 = 0x0800;

/* NAVX_OMNIMOUNT_CONFIG */

pub const OMNIMOUNT_DEFAULT: u8 = 0; /* Same as Y_Z_UP */
pub const OMNIMOUNT_YAW_X_UP: u8 = 1;
pub const OMNIMOUNT_YAW_X_DOWN: u8 = 2;
pub const OMNIMOUNT_YAW_Y_UP: u8 = 3;
pub const OMNIMOUNT_YAW_Y_DOWN: u8 = 4;
pub const OMNIMOUNT_YAW_Z_UP: u8 = 5;
pub const OMNIMOUNT_YAW_Z_DOWN: u8 = 6;

/* NAVX_INTEGRATION_CTL */

pub const NAVX_INTEGRATION_CTL_RESET_VEL_X: u8 = 0x01;
pub const NAVX_INTEGRATION_CTL_RESET_VEL_Y: u8 = 0x02;
pub const NAVX_INTEGRATION_CTL_RESET_VEL_Z: u8 = 0x04;
pub const NAVX_INTEGRATION_CTL_RESET_DISP_X: u8 = 0x08;
pub const NAVX_INTEGRATION_CTL_RESET_DISP_Y: u8 = 0x10;
pub const NAVX_INTEGRATION_CTL_RESET_DISP_Z: u8 = 0x20;
pub const NAVX_INTEGRATION_CTL_RESET_YAW: u8 = 0x80;

pub mod tuning_var_id {
    pub const UNSPECIFIED: u8 = 0;
    pub const MOTION_THRESHOLD: u8 = 1; /* In G */
    pub const YAW_STABLE_THRESHOLD: u8 = 2; /* In Degrees */
    pub const MAG_DISTURBANCE_THRESHOLD: u8 = 3; /* Ratio */
    pub const SEA_LEVEL_PRESSURE: u8 = 4; /* Millibars */
}

pub mod data_type {
    pub const TUNING_VARIABLE: u8 = 0;
    pub const MAG_CALIBRATION: u8 = 1;
    pub const BOARD_IDENTITY: u8 = 2;
}

pub mod data_action {
    pub const DATA_GET: u8 = 0;
    pub const DATA_SET: u8 = 1;
}

pub const BINARY_PACKET_INDICATOR_CHAR: char = '#';

/* AHRS Protocol encodes certain data in binary format, unlike the IMU  */
/* protocol, which encodes all data in ASCII characters.  Thus, the     */
/* packet start and message termination sequences may occur within the  */
/* message content itself.  To support the binary format, the binary    */
/* message has this format:                                             */
/*                                                                      */
/* [start][binary indicator][len][msgid]<MESSAGE>[checksum][terminator] */
/*                                                                      */
/* (The binary indicator and len are not present in the ASCII protocol) */
/*                                                                      */
/* The [len] does not include the length of the start and binary        */
/* indicator characters, but does include all other message items,      */
/* including the checksum and terminator sequence.                      */

pub const MSGID_AHRS_UPDATE: u8 = 'a' as u8;
pub const AHRS_UPDATE_YAW_VALUE_INDEX: i32 = 4; /* Degrees.  Signed Hundredths */
pub const AHRS_UPDATE_PITCH_VALUE_INDEX: i32 = 6; /* Degrees.  Signed Hundredeths */
pub const AHRS_UPDATE_ROLL_VALUE_INDEX: i32 = 8; /* Degrees.  Signed Hundredths */
pub const AHRS_UPDATE_HEADING_VALUE_INDEX: i32 = 10; /* Degrees.  Unsigned Hundredths */
pub const AHRS_UPDATE_ALTITUDE_VALUE_INDEX: i32 = 12; /* Meters.   Signed 16:16 */
pub const AHRS_UPDATE_FUSED_HEADING_VALUE_INDEX: i32 = 16; /* Degrees.  Unsigned Hundredths */
pub const AHRS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX: i32 = 18; /* Inst. G.  Signed Thousandths */
pub const AHRS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX: i32 = 20; /* Inst. G.  Signed Thousandths */
pub const AHRS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX: i32 = 22; /* Inst. G.  Signed Thousandths */
pub const AHRS_UPDATE_CAL_MAG_X_VALUE_INDEX: i32 = 24; /* Int16 (Device Units) */
pub const AHRS_UPDATE_CAL_MAG_Y_VALUE_INDEX: i32 = 26; /* Int16 (Device Units) */
pub const AHRS_UPDATE_CAL_MAG_Z_VALUE_INDEX: i32 = 28; /* Int16 (Device Units) */
pub const AHRS_UPDATE_CAL_MAG_NORM_RATIO_VALUE_INDEX: i32 = 30; /* Ratio.  Unsigned Hundredths */
pub const AHRS_UPDATE_CAL_MAG_SCALAR_VALUE_INDEX: i32 = 32; /* Coefficient. Signed 16:16 */
pub const AHRS_UPDATE_MPU_TEMP_VAUE_INDEX: i32 = 36; /* Centigrade.  Signed Hundredths */
pub const AHRS_UPDATE_RAW_MAG_X_VALUE_INDEX: i32 = 38; /* INT16 (Device Units */
pub const AHRS_UPDATE_RAW_MAG_Y_VALUE_INDEX: i32 = 40; /* INT16 (Device Units */
pub const AHRS_UPDATE_RAW_MAG_Z_VALUE_INDEX: i32 = 42; /* INT16 (Device Units */
pub const AHRS_UPDATE_QUAT_W_VALUE_INDEX: i32 = 44; /* INT16 */
pub const AHRS_UPDATE_QUAT_X_VALUE_INDEX: i32 = 46; /* INT16 */
pub const AHRS_UPDATE_QUAT_Y_VALUE_INDEX: i32 = 48; /* INT16 */
pub const AHRS_UPDATE_QUAT_Z_VALUE_INDEX: i32 = 50; /* INT16 */
pub const AHRS_UPDATE_BARO_PRESSURE_VALUE_INDEX: i32 = 52; /* millibar.  Signed 16:16 */
pub const AHRS_UPDATE_BARO_TEMP_VAUE_INDEX: i32 = 56; /* Centigrade.  Signed  Hundredths */
pub const AHRS_UPDATE_OPSTATUS_VALUE_INDEX: i32 = 58; /* NAVX_OP_STATUS_XXX */
pub const AHRS_UPDATE_SENSOR_STATUS_VALUE_INDEX: i32 = 59; /* NAVX_SENSOR_STATUS_XXX */
pub const AHRS_UPDATE_CAL_STATUS_VALUE_INDEX: i32 = 60; /* NAVX_CAL_STATUS_XXX */
pub const AHRS_UPDATE_SELFTEST_STATUS_VALUE_INDEX: i32 = 61; /* NAVX_SELFTEST_STATUS_XXX */
pub const AHRS_UPDATE_MESSAGE_CHECKSUM_INDEX: i32 = 62;
pub const AHRS_UPDATE_MESSAGE_TERMINATOR_INDEX: i32 = 64;
pub const AHRS_UPDATE_MESSAGE_LENGTH: i32 = 66;

// AHRSAndPositioning Update Packet (similar to AHRS, but removes magnetometer and adds velocity/displacement) */
pub const MSGID_AHRSPOS_UPDATE: u8 = 'p' as u8;
pub const AHRSPOS_UPDATE_YAW_VALUE_INDEX: i32 = 4; /* Degrees.  Signed Hundredths */
pub const AHRSPOS_UPDATE_PITCH_VALUE_INDEX: i32 = 6; /* Degrees.  Signed Hundredeths */
pub const AHRSPOS_UPDATE_ROLL_VALUE_INDEX: i32 = 8; /* Degrees.  Signed Hundredths */
pub const AHRSPOS_UPDATE_HEADING_VALUE_INDEX: i32 = 10; /* Degrees.  Unsigned Hundredths */
pub const AHRSPOS_UPDATE_ALTITUDE_VALUE_INDEX: i32 = 12; /* Meters.   Signed 16:16 */
pub const AHRSPOS_UPDATE_FUSED_HEADING_VALUE_INDEX: i32 = 16; /* Degrees.  Unsigned Hundredths */
pub const AHRSPOS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX: i32 = 18; /* Inst. G.  Signed Thousandths */
pub const AHRSPOS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX: i32 = 20; /* Inst. G.  Signed Thousandths */
pub const AHRSPOS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX: i32 = 22; /* Inst. G.  Signed Thousandths */
pub const AHRSPOS_UPDATE_VEL_X_VALUE_INDEX: i32 = 24; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_UPDATE_VEL_Y_VALUE_INDEX: i32 = 28; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_UPDATE_VEL_Z_VALUE_INDEX: i32 = 32; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_UPDATE_DISP_X_VALUE_INDEX: i32 = 36; /* Signed 16:16, in meters */
pub const AHRSPOS_UPDATE_DISP_Y_VALUE_INDEX: i32 = 40; /* Signed 16:16, in meters */
pub const AHRSPOS_UPDATE_DISP_Z_VALUE_INDEX: i32 = 44; /* Signed 16:16, in meters */
pub const AHRSPOS_UPDATE_QUAT_W_VALUE_INDEX: i32 = 48; /* INT16 */
pub const AHRSPOS_UPDATE_QUAT_X_VALUE_INDEX: i32 = 50; /* INT16 */
pub const AHRSPOS_UPDATE_QUAT_Y_VALUE_INDEX: i32 = 52; /* INT16 */
pub const AHRSPOS_UPDATE_QUAT_Z_VALUE_INDEX: i32 = 54; /* INT16 */
pub const AHRSPOS_UPDATE_MPU_TEMP_VAUE_INDEX: i32 = 56; /* Centigrade.  Signed Hundredths */
pub const AHRSPOS_UPDATE_OPSTATUS_VALUE_INDEX: i32 = 58; /* NAVX_OP_STATUS_XXX */
pub const AHRSPOS_UPDATE_SENSOR_STATUS_VALUE_INDEX: i32 = 59; /* NAVX_SENSOR_STATUS_XXX */
pub const AHRSPOS_UPDATE_CAL_STATUS_VALUE_INDEX: i32 = 60; /* NAVX_CAL_STATUS_XXX */
pub const AHRSPOS_UPDATE_SELFTEST_STATUS_VALUE_INDEX: i32 = 61; /* NAVX_SELFTEST_STATUS_XXX */
pub const AHRSPOS_UPDATE_MESSAGE_CHECKSUM_INDEX: i32 = 62;
pub const AHRSPOS_UPDATE_MESSAGE_TERMINATOR_INDEX: i32 = 64;
pub const AHRSPOS_UPDATE_MESSAGE_LENGTH: i32 = 66;

// AHRSAndPositioningWithTimestamp Update Packet (similar to AHRSPos, but adds sample timestamp)

pub const MSGID_AHRSPOS_TS_UPDATE: u8 = 't' as u8;
pub const AHRSPOS_TS_UPDATE_YAW_VALUE_INDEX: i32 = 4; /* Degrees.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_PITCH_VALUE_INDEX: i32 = 8; /* Degrees.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_ROLL_VALUE_INDEX: i32 = 12; /* Degrees.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_HEADING_VALUE_INDEX: i32 = 16; /* Degrees.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_ALTITUDE_VALUE_INDEX: i32 = 20; /* Meters.   Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_FUSED_HEADING_VALUE_INDEX: i32 = 24; /* Degrees.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX: i32 = 28; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX: i32 = 32; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX: i32 = 36; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_VEL_X_VALUE_INDEX: i32 = 40; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_UPDATE_VEL_Y_VALUE_INDEX: i32 = 44; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_UPDATE_VEL_Z_VALUE_INDEX: i32 = 48; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_UPDATE_DISP_X_VALUE_INDEX: i32 = 52; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_UPDATE_DISP_Y_VALUE_INDEX: i32 = 56; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_UPDATE_DISP_Z_VALUE_INDEX: i32 = 60; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_UPDATE_QUAT_W_VALUE_INDEX: i32 = 64; /* Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_QUAT_X_VALUE_INDEX: i32 = 68; /* Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_QUAT_Y_VALUE_INDEX: i32 = 72; /* Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_QUAT_Z_VALUE_INDEX: i32 = 76; /* Signed 16:16 */
pub const AHRSPOS_TS_UPDATE_MPU_TEMP_VAUE_INDEX: i32 = 80; /* Centigrade.  Signed Hundredths */
pub const AHRSPOS_TS_UPDATE_OPSTATUS_VALUE_INDEX: i32 = 82; /* NAVX_OP_STATUS_XXX */
pub const AHRSPOS_TS_UPDATE_SENSOR_STATUS_VALUE_INDEX: i32 = 83; /* NAVX_SENSOR_STATUS_XXX */
pub const AHRSPOS_TS_UPDATE_CAL_STATUS_VALUE_INDEX: i32 = 84; /* NAVX_CAL_STATUS_XXX */
pub const AHRSPOS_TS_UPDATE_SELFTEST_STATUS_VALUE_INDEX: i32 = 85; /* NAVX_SELFTEST_STATUS_XXX */
pub const AHRSPOS_TS_UPDATE_TIMESTAMP_INDEX: i32 = 86; /* UINT32, Timestamp (milliseconds) */
pub const AHRSPOS_TS_UPDATE_MESSAGE_CHECKSUM_INDEX: i32 = 90;
pub const AHRSPOS_TS_UPDATE_MESSAGE_TERMINATOR_INDEX: i32 = 92;
pub const AHRSPOS_TS_UPDATE_MESSAGE_LENGTH: i32 = 94;

// AHRSAndPositioningWithTimestampAndRaw Update Packet (similar to AHRSPosTS, but adds raw data)

pub const MSGID_AHRSPOS_TS_RAW_UPDATE: u8 = 'u' as u8;
pub const AHRSPOS_TS_RAW_UPDATE_YAW_VALUE_INDEX: i32 = 4; /* Signed 16:16.  Signed Hundredths */
pub const AHRSPOS_TS_RAW_UPDATE_ROLL_VALUE_INDEX: i32 = 8; /* Signed 16:16.  Signed Hundredths */
pub const AHRSPOS_TS_RAW_UPDATE_PITCH_VALUE_INDEX: i32 = 12; /* Signed 16:16.  Signed Hundredeths */
pub const AHRSPOS_TS_RAW_UPDATE_HEADING_VALUE_INDEX: i32 = 16; /* Signed 16:16.  Unsigned Hundredths */
pub const AHRSPOS_TS_RAW_UPDATE_ALTITUDE_VALUE_INDEX: i32 = 20; /* Meters.   Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_FUSED_HEADING_VALUE_INDEX: i32 = 24; /* Degrees.  Unsigned Hundredths */
pub const AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX: i32 = 28; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX: i32 = 32; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX: i32 = 36; /* Inst. G.  Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_VEL_X_VALUE_INDEX: i32 = 40; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_RAW_UPDATE_VEL_Y_VALUE_INDEX: i32 = 44; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_RAW_UPDATE_VEL_Z_VALUE_INDEX: i32 = 48; /* Signed 16:16, in meters/sec */
pub const AHRSPOS_TS_RAW_UPDATE_DISP_X_VALUE_INDEX: i32 = 52; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_RAW_UPDATE_DISP_Y_VALUE_INDEX: i32 = 56; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_RAW_UPDATE_DISP_Z_VALUE_INDEX: i32 = 60; /* Signed 16:16, in meters */
pub const AHRSPOS_TS_RAW_UPDATE_QUAT_W_VALUE_INDEX: i32 = 64; /* Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_QUAT_X_VALUE_INDEX: i32 = 68; /* Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_QUAT_Y_VALUE_INDEX: i32 = 72; /* Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_QUAT_Z_VALUE_INDEX: i32 = 76; /* Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_MPU_TEMP_VAUE_INDEX: i32 = 80; /* Centigrade.  Signed Hundredths */
pub const AHRSPOS_TS_RAW_UPDATE_OPSTATUS_VALUE_INDEX: i32 = 82; /* NAVX_OP_STATUS_XXX */
pub const AHRSPOS_TS_RAW_UPDATE_SENSOR_STATUS_VALUE_INDEX: i32 = 83; /* NAVX_SENSOR_STATUS_XXX */
pub const AHRSPOS_TS_RAW_UPDATE_CAL_STATUS_VALUE_INDEX: i32 = 84; /* NAVX_CAL_STATUS_XXX */
pub const AHRSPOS_TS_RAW_UPDATE_SELFTEST_STATUS_VALUE_INDEX: i32 = 85; /* NAVX_SELFTEST_STATUS_XXX */
pub const AHRSPOS_TS_RAW_UPDATE_TIMESTAMP_INDEX: i32 = 86; /* UINT32 Timestamp, in milliseconds */
pub const AHRSPOS_TS_RAW_UPDATE_GYRO_X_VALUE_INDEX: i32 = 90; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_GYRO_Y_VALUE_INDEX: i32 = 92; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_GYRO_Z_VALUE_INDEX: i32 = 94; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_ACC_X_VALUE_INDEX: i32 = 96; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_ACC_Y_VALUE_INDEX: i32 = 98; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_ACC_Z_VALUE_INDEX: i32 = 100; /* INT16, in device units (scaled by FSR) */
pub const AHRSPOS_TS_RAW_UPDATE_LAST_SAMPLE_DELTA_SEC_INDEX: i32 = 104; /* Signed 16:16 */
pub const AHRSPOS_TS_RAW_UPDATE_GYRO_BIAS_UPDATED_BOOL_INDEX: i32 = 108; /* INT16, 0 if false, otherwise true */
pub const AHRSPOS_TS_RAW_UPDATE_MESSAGE_CHECKSUM_INDEX: i32 = 110;
pub const AHRSPOS_TS_RAW_UPDATE_MESSAGE_TERMINATOR_INDEX: i32 = 112;
pub const AHRSPOS_TS_RAW_UPDATE_MESSAGE_LENGTH: i32 = 114;

// Data Get Request:  Tuning Variable, Mag Cal, Board Identity (Response message depends upon request type)
pub const MSGID_DATA_REQUEST: u8 = 'D' as u8;
pub const DATA_REQUEST_DATATYPE_VALUE_INDEX: i32 = 4;
pub const DATA_REQUEST_VARIABLEID_VALUE_INDEX: i32 = 5;
pub const DATA_REQUEST_CHECKSUM_INDEX: i32 = 6;
pub const DATA_REQUEST_TERMINATOR_INDEX: i32 = 8;
pub const DATA_REQUEST_MESSAGE_LENGTH: i32 = 10;

// Data Set Response Packet
pub const MSGID_DATA_SET_RESPONSE: u8 = 'v' as u8;
pub const DATA_SET_RESPONSE_DATATYPE_VALUE_INDEX: i32 = 4;
pub const DATA_SET_RESPONSE_VARID_VALUE_INDEX: i32 = 5;
pub const DATA_SET_RESPONSE_STATUS_VALUE_INDEX: i32 = 6;
pub const DATA_SET_RESPONSE_MESSAGE_CHECKSUM_INDEX: i32 = 7;
pub const DATA_SET_RESPONSE_MESSAGE_TERMINATOR_INDEX: i32 = 9;
pub const DATA_SET_RESPONSE_MESSAGE_LENGTH: i32 = 11;

/* Integration Control Command Packet */
pub const MSGID_INTEGRATION_CONTROL_CMD: u8 = 'I' as u8;
pub const INTEGRATION_CONTROL_CMD_ACTION_INDEX: i32 = 4;
pub const INTEGRATION_CONTROL_CMD_PARAMETER_INDEX: i32 = 5;
pub const INTEGRATION_CONTROL_CMD_MESSAGE_CHECKSUM_INDEX: i32 = 9;
pub const INTEGRATION_CONTROL_CMD_MESSAGE_TERMINATOR_INDEX: i32 = 11;
pub const INTEGRATION_CONTROL_CMD_MESSAGE_LENGTH: i32 = 13;

/* Integration Control Response Packet */
pub const MSGID_INTEGRATION_CONTROL_RESP: u8 = 'j' as u8;
pub const INTEGRATION_CONTROL_RESP_ACTION_INDEX: i32 = 4;
pub const INTEGRATION_CONTROL_RESP_PARAMETER_INDEX: i32 = 5;
pub const INTEGRATION_CONTROL_RESP_MESSAGE_CHECKSUM_INDEX: i32 = 9;
pub const INTEGRATION_CONTROL_RESP_MESSAGE_TERMINATOR_INDEX: i32 = 11;
pub const INTEGRATION_CONTROL_RESP_MESSAGE_LENGTH: i32 = 13;

// Magnetometer Calibration Packet - e.g., !m[x_bias][y_bias][z_bias][m1,1 ... m3,3][cr][lf]
pub const MSGID_MAG_CAL_CMD: u8 = 'M' as u8;
pub const MAG_CAL_DATA_ACTION_VALUE_INDEX: i32 = 4;
pub const MAG_X_BIAS_VALUE_INDEX: i32 = 5; /* signed short */
pub const MAG_Y_BIAS_VALUE_INDEX: i32 = 7;
pub const MAG_Z_BIAS_VALUE_INDEX: i32 = 9;
pub const MAG_XFORM_1_1_VALUE_INDEX: i32 = 11; /* signed 16:16 */
pub const MAG_XFORM_1_2_VALUE_INDEX: i32 = 15;
pub const MAG_XFORM_1_3_VALUE_INDEX: i32 = 19;
pub const MAG_XFORM_2_1_VALUE_INDEX: i32 = 23;
pub const MAG_XFORM_2_2_VALUE_INDEX: i32 = 25;
pub const MAG_XFORM_2_3_VALUE_INDEX: i32 = 31;
pub const MAG_XFORM_3_1_VALUE_INDEX: i32 = 35;
pub const MAG_XFORM_3_2_VALUE_INDEX: i32 = 39;
pub const MAG_XFORM_3_3_VALUE_INDEX: i32 = 43;
pub const MAG_CAL_EARTH_MAG_FIELD_NORM_VALUE_INDEX: i32 = 47;
pub const MAG_CAL_CMD_MESSAGE_CHECKSUM_INDEX: i32 = 51;
pub const MAG_CAL_CMD_MESSAGE_TERMINATOR_INDEX: i32 = 53;
pub const MAG_CAL_CMD_MESSAGE_LENGTH: i32 = 55;

// Tuning Variable Packet
pub const MSGID_FUSION_TUNING_CMD: u8 = 'T' as u8;
pub const FUSION_TUNING_DATA_ACTION_VALUE_INDEX: i32 = 4;
pub const FUSION_TUNING_CMD_VAR_ID_VALUE_INDEX: i32 = 5;
pub const FUSION_TUNING_CMD_VAR_VALUE_INDEX: i32 = 6;
pub const FUSION_TUNING_CMD_MESSAGE_CHECKSUM_INDEX: i32 = 10;
pub const FUSION_TUNING_CMD_MESSAGE_TERMINATOR_INDEX: i32 = 12;
pub const FUSION_TUNING_CMD_MESSAGE_LENGTH: i32 = 14;

// Board Identity Response Packet- e.g., !c[type][hw_rev][fw_major][fw_minor][unique_id[12]]
pub const MSGID_BOARD_IDENTITY_RESPONSE: u8 = 'i' as u8;
pub const BOARD_IDENTITY_BOARDTYPE_VALUE_INDEX: i32 = 4;
pub const BOARD_IDENTITY_HWREV_VALUE_INDEX: i32 = 5;
pub const BOARD_IDENTITY_FW_VER_MAJOR: i32 = 6;
pub const BOARD_IDENTITY_FW_VER_MINOR: i32 = 7;
pub const BOARD_IDENTITY_FW_VER_REVISION_VALUE_INDEX: i32 = 8;
pub const BOARD_IDENTITY_UNIQUE_ID_0: i32 = 10;
pub const BOARD_IDENTITY_UNIQUE_ID_1: i32 = 11;
pub const BOARD_IDENTITY_UNIQUE_ID_2: i32 = 12;
pub const BOARD_IDENTITY_UNIQUE_ID_3: i32 = 13;
pub const BOARD_IDENTITY_UNIQUE_ID_4: i32 = 14;
pub const BOARD_IDENTITY_UNIQUE_ID_5: i32 = 15;
pub const BOARD_IDENTITY_UNIQUE_ID_6: i32 = 16;
pub const BOARD_IDENTITY_UNIQUE_ID_7: i32 = 17;
pub const BOARD_IDENTITY_UNIQUE_ID_8: i32 = 18;
pub const BOARD_IDENTITY_UNIQUE_ID_9: i32 = 19;
pub const BOARD_IDENTITY_UNIQUE_ID_10: i32 = 20;
pub const BOARD_IDENTITY_UNIQUE_ID_11: i32 = 21;
pub const BOARD_IDENTITY_RESPONSE_CHECKSUM_INDEX: i32 = 22;
pub const BOARD_IDENTITY_RESPONSE_TERMINATOR_INDEX: i32 = 24;
pub const BOARD_IDENTITY_RESPONSE_MESSAGE_LENGTH: i32 = 26;

pub const MAX_BINARY_MESSAGE_LENGTH: i32 = AHRSPOS_TS_RAW_UPDATE_MESSAGE_LENGTH;

#[derive(Debug, Clone, Default)]
pub struct AHRSUpdateBase {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
    pub altitude: f32,
    pub fused_heading: f32,
    pub linear_accel_x: f32,
    pub linear_accel_y: f32,
    pub linear_accel_z: f32,
    pub mpu_temp: f32,
    pub quat_w: f32,
    pub quat_x: f32,
    pub quat_y: f32,
    pub quat_z: f32,
    pub barometric_pressure: f32,
    // pub baro_temp: f32,
    pub op_status: u8,
    pub sensor_status: u8,
    pub cal_status: u8,
    pub selftest_status: u8,
}

#[derive(Debug, Clone, Default)]
pub struct AHRSUpdate {
    pub base: AHRSUpdateBase,
    pub cal_mag_x: i16,
    pub cal_mag_y: i16,
    pub cal_mag_z: i16,
    pub mag_field_norm_ratio: f32,
    pub mag_field_norm_scalar: f32,
    pub raw_mag_x: i16,
    pub raw_mag_y: i16,
    pub raw_mag_z: i16,
}

#[derive(Debug, Clone, Default)]
pub struct AHRSPosUpdate {
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub disp_x: f32,
    pub disp_y: f32,
    pub disp_z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct AHRSPosTSUpdate {
    pub pos: AHRSPosUpdate,
    pub timestamp: u32,
}

#[derive(Debug, Clone, Default)]
pub struct AHRSPosTSRawUpdate {
    pub pos: AHRSPosTSUpdate,
    pub raw_gyro_x: i16,         // Device units, scaled by FSR
    pub raw_gyro_y: i16,         // Device units, scaled by FSR
    pub raw_gyro_z: i16,         // Device units, scaled by FSR
    pub accel_x: i16,            // Device units, scaled by FSR
    pub accel_y: i16,            // Device units, scaled by FSR
    pub accel_z: i16,            // Device units, scaled by FSR
    pub delta_t_sec: f32,        // last sample period, as a portion of a second
    pub gyro_bias_updated: bool, // true if gyro bias was updated (e.g., when still) during last sample
}

#[derive(Debug, Clone, Default)]
pub struct DataSetResponse {
    pub data_type: u8,
    pub var_id: u8, /* If type = TUNING_VARIABLE */
    pub status: u8,
}

#[derive(Debug, Clone, Default)]
pub struct IntegrationControl {
    pub action: u8,
    pub parameter: u32,
}

#[derive(Debug, Clone, Default)]
pub struct MagCalData {
    pub action: u8,
    pub mag_bias: [i16; 3],       /* 3 Values */
    pub mag_xform: [[f32; 3]; 3], /* 3 x 3 Values */
    pub earth_mag_field_norm: f32,
}

#[derive(Debug, Clone, Default)]
pub struct TuningVar {
    pub action: u8,
    pub var_id: u8, /* If type = TUNING_VARIABLE */
    pub value: f32,
}

#[derive(Debug, Clone, Default)]
pub struct BoardID {
    pub ty: u8,
    pub hw_rev: u8,
    pub fw_ver_major: u8,
    pub fw_ver_minor: u8,
    pub fw_revision: u16,
    pub unique_id: [u8; 12],
}

/* protocol data is encoded little endian, convert to Java's big endian format */
pub fn decode_u16(buffer: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap())
}

pub fn encode_u16(val: u16, buffer: &mut [u8], offset: usize) {
    buffer[offset..offset + 2].copy_from_slice(&val.to_le_bytes());
}

pub fn decode_u32(buffer: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap())
}

pub fn encode_u32(val: u32, buffer: &mut [u8], offset: usize) {
    buffer[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
}

pub fn decode_i16(buffer: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap())
}

pub fn encode_i16(val: i16, buffer: &mut [u8], offset: usize) {
    buffer[offset..offset + 2].copy_from_slice(&val.to_le_bytes());
}

/// -327.68 to +327.68
pub fn decode_protocol_signed_hundredths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_i16(buffer, offset) as f32 / 100.0
}

pub fn encode_protocol_signed_hundredths_float(val: f32, buffer: &mut [u8], offset: usize) {
    encode_i16((val * 100.0) as i16, buffer, offset);
}

pub fn encode_signed_hundredths_float(input: f32) -> i16 {
    (input * 100.0) as i16
}

pub fn encode_unsigned_hundredths_float(input: f32) -> u16 {
    (input * 100.0) as u16
}

pub fn encode_ratio_float(input_ratio: f32) -> f32 {
    input_ratio * 32768.0
}

pub fn encode_signed_thousandths_float(input: f32) -> f32 {
    input * 1000.0
}

/// 0 to 655.35
pub fn decode_protocol_unsigned_hundredths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u16(buffer, offset) as f32 / 100.0
}

pub fn encode_protocol_unsigned_hundredths_float(input: f32, buffer: &mut [u8], offset: usize) {
    encode_u16((input * 100.0) as u16, buffer, offset);
}

/// -32.768 to +32.768
pub fn decode_protocol_signed_thousandths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u16(buffer, offset) as f32 / 1000.0
}

pub fn encode_protocol_signed_thousandths_float(input: f32, buffer: &mut [u8], offset: usize) {
    encode_i16((input * 1000.0) as i16, buffer, offset);
}

/// In units of -1 to 1, multiplied by 16384
pub fn decode_protocol_ratio(buffer: &[u8], offset: usize) -> f32 {
    decode_u16(buffer, offset) as f32 / 32768.0
}

pub fn encode_protocol_ratio(ratio: f32, buffer: &mut [u8], offset: usize) {
    encode_i16((ratio * 32768.0) as i16, buffer, offset);
}

/// <int16>.<uint16> (-32768.9999 to 32767.9999)
pub fn decode_protocol1616_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u32(buffer, offset) as f32 / 65536.0
}
pub fn encode_protocol1616_float(val: f32, buffer: &mut [u8], offset: usize) {
    encode_u32((val * 65536.0) as u32, buffer, offset);
}

pub const PACKET_START_CHAR: u8 = '!' as u8;
pub const PROTOCOL_FLOAT_LENGTH: i32 = 7;
pub const CHECKSUM_LENGTH: i32 = 2;
pub const TERMINATOR_LENGTH: i32 = 2;

// Yaw/Pitch/Roll (YPR) Update Packet - e.g., !y[yaw][pitch][roll][compass_heading]
pub const MSGID_YPR_UPDATE: u8 = 'y' as u8;
pub const YPR_UPDATE_YAW_VALUE_INDEX: i32 = 2;
pub const YPR_UPDATE_PITCH_VALUE_INDEX: i32 = 9;
pub const YPR_UPDATE_ROLL_VALUE_INDEX: i32 = 16;
pub const YPR_UPDATE_COMPASS_VALUE_INDEX: i32 = 23;
pub const YPR_UPDATE_CHECKSUM_INDEX: i32 = 30;
pub const YPR_UPDATE_TERMINATOR_INDEX: i32 = 32;
pub const YPR_UPDATE_MESSAGE_LENGTH: i32 = 34;

// Quaternion Data Update Packet - e.g., !r[q1][q2][q3][q4][accelx][accely][accelz][magx][magy][magz]
pub const MSGID_QUATERNION_UPDATE: u8 = 'q' as u8;
pub const QUATERNION_UPDATE_MESSAGE_LENGTH: i32 = 53;
pub const QUATERNION_UPDATE_QUAT1_VALUE_INDEX: i32 = 2;
pub const QUATERNION_UPDATE_QUAT2_VALUE_INDEX: i32 = 6;
pub const QUATERNION_UPDATE_QUAT3_VALUE_INDEX: i32 = 10;
pub const QUATERNION_UPDATE_QUAT4_VALUE_INDEX: i32 = 14;
pub const QUATERNION_UPDATE_ACCEL_X_VALUE_INDEX: i32 = 18;
pub const QUATERNION_UPDATE_ACCEL_Y_VALUE_INDEX: i32 = 22;
pub const QUATERNION_UPDATE_ACCEL_Z_VALUE_INDEX: i32 = 26;
pub const QUATERNION_UPDATE_MAG_X_VALUE_INDEX: i32 = 30;
pub const QUATERNION_UPDATE_MAG_Y_VALUE_INDEX: i32 = 34;
pub const QUATERNION_UPDATE_MAG_Z_VALUE_INDEX: i32 = 38;
pub const QUATERNION_UPDATE_TEMP_VALUE_INDEX: i32 = 42;
pub const QUATERNION_UPDATE_CHECKSUM_INDEX: i32 = 49;
pub const QUATERNION_UPDATE_TERMINATOR_INDEX: i32 = 51;

// Gyro/Raw Data Update packet - e.g., !g[gx][gy][gz][accelx][accely][accelz][magx][magy][magz][temp_c][cr][lf]

pub const MSGID_GYRO_UPDATE: u8 = 'g' as u8;
pub const GYRO_UPDATE_GYRO_X_VALUE_INDEX: i32 = 2;
pub const GYRO_UPDATE_GYRO_Y_VALUE_INDEX: i32 = 6;
pub const GYRO_UPDATE_GYRO_Z_VALUE_INDEX: i32 = 10;
pub const GYRO_UPDATE_ACCEL_X_VALUE_INDEX: i32 = 14;
pub const GYRO_UPDATE_ACCEL_Y_VALUE_INDEX: i32 = 18;
pub const GYRO_UPDATE_ACCEL_Z_VALUE_INDEX: i32 = 22;
pub const GYRO_UPDATE_MAG_X_VALUE_INDEX: i32 = 26;
pub const GYRO_UPDATE_MAG_Y_VALUE_INDEX: i32 = 30;
pub const GYRO_UPDATE_MAG_Z_VALUE_INDEX: i32 = 34;
pub const GYRO_UPDATE_TEMP_VALUE_INDEX: i32 = 38;
pub const GYRO_UPDATE_CHECKSUM_INDEX: i32 = 42;
pub const GYRO_UPDATE_TERMINATOR_INDEX: i32 = 44;
pub const GYRO_UPDATE_MESSAGE_LENGTH: i32 = 46;

// EnableStream Command Packet - e.g., !S[stream type][checksum][cr][lf]
pub const MSGID_STREAM_CMD: u8 = 'S' as u8;
pub const STREAM_CMD_STREAM_TYPE_YPR: i32 = MSGID_YPR_UPDATE as i32;
pub const STREAM_CMD_STREAM_TYPE_QUATERNION: i32 = MSGID_QUATERNION_UPDATE as i32;
pub const STREAM_CMD_STREAM_TYPE_GYRO: i32 = MSGID_GYRO_UPDATE as i32;
pub const STREAM_CMD_STREAM_TYPE_INDEX: i32 = 2;
pub const STREAM_CMD_UPDATE_RATE_HZ_INDEX: i32 = 3;
pub const STREAM_CMD_CHECKSUM_INDEX: i32 = 5;
pub const STREAM_CMD_TERMINATOR_INDEX: i32 = 7;
pub const STREAM_CMD_MESSAGE_LENGTH: i32 = 9;

// EnableStream Response Packet - e.g., !s[stream type][gyro full scale range][accel full scale range][update rate hz][yaw_offset_degrees][flags][checksum][cr][lf]
pub const MSG_ID_STREAM_RESPONSE: u8 = 's' as u8;
pub const STREAM_RESPONSE_MESSAGE_LENGTH: i32 = 46;
pub const STREAM_RESPONSE_STREAM_TYPE_INDEX: i32 = 2;
pub const STREAM_RESPONSE_GYRO_FULL_SCALE_DPS_RANGE: i32 = 3;
pub const STREAM_RESPONSE_ACCEL_FULL_SCALE_G_RANGE: i32 = 7;
pub const STREAM_RESPONSE_UPDATE_RATE_HZ: i32 = 11;
pub const STREAM_RESPONSE_YAW_OFFSET_DEGREES: i32 = 15;
pub const STREAM_RESPONSE_QUAT1_OFFSET: i32 = 22;
pub const STREAM_RESPONSE_QUAT2_OFFSET: i32 = 26;
pub const STREAM_RESPONSE_QUAT3_OFFSET: i32 = 30;
pub const STREAM_RESPONSE_QUAT4_OFFSET: i32 = 34;
pub const STREAM_RESPONSE_FLAGS: i32 = 38;
pub const STREAM_RESPONSE_CHECKSUM_INDEX: i32 = 42;
pub const STREAM_RESPONSE_TERMINATOR_INDEX: i32 = 44;

pub const STREAM_MSG_TERMINATION_CHAR: u8 = '\n' as u8;

pub const NAV6_FLAG_MASK_CALIBRATION_STATE: i16 = 0x03;

pub const NAV6_CALIBRATION_STATE_WAIT: i16 = 0x00;
pub const NAV6_CALIBRATION_STATE_ACCUMULATE: i16 = 0x01;
pub const NAV6_CALIBRATION_STATE_COMPLETE: i16 = 0x02;

pub const IMU_PROTOCOL_MAX_MESSAGE_LENGTH: i32 = QUATERNION_UPDATE_MESSAGE_LENGTH;

#[derive(Debug, Clone, Default)]
pub struct YPRUpdate {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
}

#[derive(Debug, Clone, Default)]
pub struct StreamCommand {
    pub stream_type: u8,
}

#[derive(Debug, Clone, Default)]
pub struct StreamResponse {
    pub stream_type: u8,
    pub gyro_fsr_dps: u16,
    pub accel_fsr_g: u16,
    pub update_rate_hz: u16,
    pub yaw_offset_degrees: f32,
    pub q1_offset: u16,
    pub q2_offset: u16,
    pub q3_offset: u16,
    pub q4_offset: u16,
    pub flags: u16,
}

#[derive(Debug, Clone, Default)]
pub struct QuaternionUpdate {
    pub q1: u16,
    pub q2: u16,
    pub q3: u16,
    pub q4: u16,
    pub accel_x: u16,
    pub accel_y: u16,
    pub accel_z: u16,
    pub mag_x: u16,
    pub mag_y: u16,
    pub mag_z: u16,
    pub temp_c: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GyroUpdate {
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub mag_x: i16,
    pub mag_y: i16,
    pub mag_z: i16,
    pub temp_c: f32,
}

pub fn encode_stream_command(
    protocol_buffer: &mut [u8],
    stream_type: u8,
    update_rate_hz: u8,
) -> i32 {
    // Header
    protocol_buffer[0] = PACKET_START_CHAR;
    protocol_buffer[1] = MSGID_STREAM_CMD;

    // Data
    protocol_buffer[STREAM_CMD_STREAM_TYPE_INDEX as usize] = stream_type;
    byte_to_hex(
        update_rate_hz,
        protocol_buffer,
        STREAM_CMD_UPDATE_RATE_HZ_INDEX as usize,
    );

    // Footer
    encode_termination(
        protocol_buffer,
        STREAM_CMD_MESSAGE_LENGTH as usize,
        STREAM_CMD_MESSAGE_LENGTH as usize - 4,
    );

    STREAM_CMD_MESSAGE_LENGTH
}

pub fn decode_stream_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    r: &mut StreamResponse,
) -> i32 {
    if length < STREAM_RESPONSE_MESSAGE_LENGTH as usize {
        return 0;
    }

    if buffer[offset + 0] == PACKET_START_CHAR && buffer[offset + 1] == MSG_ID_STREAM_RESPONSE {
        if !verify_checksum(buffer, offset, STREAM_RESPONSE_CHECKSUM_INDEX as usize) {
            return 0;
        }

        r.stream_type = buffer[offset + 2];
        r.gyro_fsr_dps = decode_u16_hex(
            buffer,
            offset + STREAM_RESPONSE_GYRO_FULL_SCALE_DPS_RANGE as usize,
        );
        r.accel_fsr_g = decode_u16_hex(
            buffer,
            offset + STREAM_RESPONSE_ACCEL_FULL_SCALE_G_RANGE as usize,
        );
        r.update_rate_hz = decode_u16_hex(buffer, offset + STREAM_RESPONSE_UPDATE_RATE_HZ as usize);
        r.yaw_offset_degrees =
            decode_protocol_float(buffer, offset + STREAM_RESPONSE_YAW_OFFSET_DEGREES as usize);
        r.q1_offset = decode_u16_hex(buffer, offset + STREAM_RESPONSE_QUAT1_OFFSET as usize);
        r.q2_offset = decode_u16_hex(buffer, offset + STREAM_RESPONSE_QUAT2_OFFSET as usize);
        r.q3_offset = decode_u16_hex(buffer, offset + STREAM_RESPONSE_QUAT3_OFFSET as usize);
        r.q4_offset = decode_u16_hex(buffer, offset + STREAM_RESPONSE_QUAT4_OFFSET as usize);
        r.flags = decode_u16_hex(buffer, offset + STREAM_RESPONSE_FLAGS as usize);

        return STREAM_RESPONSE_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_stream_command(
    buffer: &[u8],
    offset: usize,
    length: usize,
    c: &mut StreamCommand,
) -> i32 {
    if length < STREAM_CMD_MESSAGE_LENGTH as usize {
        return 0;
    }

    if buffer[offset + 0] == '!' as u8 && buffer[offset + 1] == MSGID_STREAM_CMD {
        if !verify_checksum(buffer, offset, STREAM_CMD_CHECKSUM_INDEX as usize) {
            return 0;
        }

        c.stream_type = buffer[offset + STREAM_CMD_STREAM_TYPE_INDEX as usize];
        return STREAM_CMD_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_yprupdate(buffer: &[u8], offset: usize, length: usize, u: &mut YPRUpdate) -> i32 {
    if length < YPR_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }

    if buffer[offset + 0] == '!' as u8 && buffer[offset + 1] == 'y' as u8 {
        if !verify_checksum(buffer, offset, YPR_UPDATE_CHECKSUM_INDEX as usize) {
            return 0;
        }

        u.yaw = decode_protocol_float(buffer, offset + YPR_UPDATE_YAW_VALUE_INDEX as usize);
        u.pitch = decode_protocol_float(buffer, offset + YPR_UPDATE_PITCH_VALUE_INDEX as usize);
        u.roll = decode_protocol_float(buffer, offset + YPR_UPDATE_ROLL_VALUE_INDEX as usize);
        u.compass_heading =
            decode_protocol_float(buffer, offset + YPR_UPDATE_COMPASS_VALUE_INDEX as usize);
        return YPR_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_quaternion_update(
    buffer: &[u8],
    offset: usize,
    length: usize,
    u: &mut QuaternionUpdate,
) -> i32 {
    if length < QUATERNION_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR && buffer[offset + 1] == MSGID_QUATERNION_UPDATE {
        if !verify_checksum(buffer, offset, QUATERNION_UPDATE_CHECKSUM_INDEX as usize) {
            return 0;
        }

        u.q1 = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_QUAT1_VALUE_INDEX as usize,
        );
        u.q2 = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_QUAT2_VALUE_INDEX as usize,
        );
        u.q3 = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_QUAT3_VALUE_INDEX as usize,
        );
        u.q4 = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_QUAT4_VALUE_INDEX as usize,
        );
        u.accel_x = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_ACCEL_X_VALUE_INDEX as usize,
        );
        u.accel_y = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_ACCEL_Y_VALUE_INDEX as usize,
        );
        u.accel_z = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_ACCEL_Z_VALUE_INDEX as usize,
        );
        u.mag_x = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_MAG_X_VALUE_INDEX as usize,
        );
        u.mag_y = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_MAG_Y_VALUE_INDEX as usize,
        );
        u.mag_z = decode_u16_hex(
            buffer,
            offset + QUATERNION_UPDATE_MAG_Z_VALUE_INDEX as usize,
        );
        u.temp_c =
            decode_protocol_float(buffer, offset + QUATERNION_UPDATE_TEMP_VALUE_INDEX as usize);
        return QUATERNION_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

// pub fn decode_gyro_update(buffer: &[u8], offset: usize, length: usize, u: &mut GyroUpdate) -> i32 {
//     if length < GYRO_UPDATE_MESSAGE_LENGTH as usize {
//         return 0;
//     }
//     if buffer[offset + 0] == PACKET_START_CHAR && buffer[offset + 1] == MSGID_GYRO_UPDATE {
//         if !verify_checksum(buffer, offset, GYRO_UPDATE_CHECKSUM_INDEX as usize) {
//             return 0;
//         }
//
//         u.gyro_x = decode_u16_hex(buffer, offset + GYRO_UPDATE_GYRO_X_VALUE_INDEX as usize);
//         u.gyro_y = decode_u16_hex(buffer, offset + GYRO_UPDATE_GYRO_Y_VALUE_INDEX as usize);
//         u.gyro_z = decode_u16_hex(buffer, offset + GYRO_UPDATE_GYRO_Z_VALUE_INDEX as usize);
//         u.accel_x = decode_u16_hex(buffer, offset + GYRO_UPDATE_ACCEL_X_VALUE_INDEX as usize);
//         u.accel_y = decode_u16_hex(buffer, offset + GYRO_UPDATE_ACCEL_Y_VALUE_INDEX as usize);
//         u.accel_z = decode_u16_hex(buffer, offset + GYRO_UPDATE_ACCEL_Z_VALUE_INDEX as usize);
//         u.mag_x = decode_u16_hex(buffer, offset + GYRO_UPDATE_MAG_X_VALUE_INDEX as usize);
//         u.mag_y = decode_u16_hex(buffer, offset + GYRO_UPDATE_MAG_Y_VALUE_INDEX as usize);
//         u.mag_z = decode_u16_hex(buffer, offset + GYRO_UPDATE_MAG_Z_VALUE_INDEX as usize);
//         u.temp_c = decode_protocol_unsigned_hundredths_float(
//             buffer,
//             offset + GYRO_UPDATE_TEMP_VALUE_INDEX as usize,
//         );
//         return GYRO_UPDATE_MESSAGE_LENGTH;
//     }
//     return 0;
// }

pub fn encode_termination(buffer: &mut [u8], total_length: usize, content_length: usize) {
    if total_length >= CHECKSUM_LENGTH as usize + TERMINATOR_LENGTH as usize
        && total_length >= content_length + CHECKSUM_LENGTH as usize + TERMINATOR_LENGTH as usize
    {
        // Checksum
        let mut checksum = 0;
        for i in 0..content_length {
            checksum += buffer[i];
        }
        // convert checksum to two ascii bytes

        byte_to_hex(checksum, buffer, content_length);
        // Message Terminator
        buffer[content_length + CHECKSUM_LENGTH as usize + 0] = '\r' as u8;
        buffer[content_length + CHECKSUM_LENGTH as usize + 1] = '\n' as u8;
    }
}

pub const HEX_ARRAY: &[u8] = &[
    '0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8,
    '8' as u8, '9' as u8, 'A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8,
];

pub fn byte_to_hex(thebyte: u8, dest: &mut [u8], offset: usize) {
    let v = thebyte & 0xFF;
    dest[offset + 0] = HEX_ARRAY[(v >> 4) as usize];
    dest[offset + 1] = HEX_ARRAY[(v & 0x0F) as usize];
}

pub fn decode_u16_hex(uint16_string: &[u8], offset: usize) -> u16 {
    let mut decoded_uint16 = 0;
    let mut shift_left = 12;
    for i in offset..offset + 4 {
        let digit = if uint16_string[i] <= '9' as u8 {
            uint16_string[i] - ('0' as u8)
        } else {
            uint16_string[i] - ('A' as u8) + 10
        } as u16;
        decoded_uint16 += digit << shift_left;
        shift_left -= 4;
    }
    decoded_uint16
}

/* 0 to 655.35 */
pub fn decode_protocol_unsigned_hundredths_float_hex(
    uint8_unsigned_hundredths_float: &[u8],
    offset: usize,
) -> f32 {
    decode_u16_hex(uint8_unsigned_hundredths_float, offset) as f32 / 100.0
}

pub fn verify_checksum(buffer: &[u8], offset: usize, content_length: usize) -> bool {
    // Calculate Checksum
    let mut checksum = 0;
    for i in 0..content_length {
        checksum += buffer[offset + i];
    }

    // Decode Checksum
    let decoded_checksum = decode_u8_hex(buffer, offset + content_length);

    checksum == decoded_checksum
}

pub fn decode_u8_hex(checksum: &[u8], offset: usize) -> u8 {
    let first_digit = if checksum[0 + offset] <= '9' as u8 {
        checksum[0 + offset] - '0' as u8
    } else {
        checksum[0 + offset] - 'A' as u8 + 10
    };
    let second_digit = if checksum[0 + offset] <= '9' as u8 {
        checksum[0 + offset] - '0' as u8
    } else {
        checksum[0 + offset] - 'A' as u8 + 10
    };
    first_digit * 16 + second_digit
}

pub fn decode_protocol_float(buffer: &[u8], offset: usize) -> f32 {
    str::from_utf8(&buffer[offset..offset + PROTOCOL_FLOAT_LENGTH as usize])
        .unwrap()
        .parse::<f32>()
        .unwrap()
}

pub fn decode_ahrsupdate(buffer: &[u8], offset: usize, length: usize, u: &mut AHRSUpdate) -> i32 {
    if length < AHRS_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == AHRS_UPDATE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_AHRS_UPDATE
    {
        if !verify_checksum(buffer, offset, AHRS_UPDATE_MESSAGE_CHECKSUM_INDEX as usize) {
            return 0;
        }

        u.base.yaw = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_YAW_VALUE_INDEX as usize,
        );
        u.base.pitch = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_ROLL_VALUE_INDEX as usize,
        ); /* FIXME */
        u.base.roll = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_PITCH_VALUE_INDEX as usize,
        ); /* FIXME */
        u.base.compass_heading = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_HEADING_VALUE_INDEX as usize,
        );
        u.base.altitude =
            decode_protocol1616_float(buffer, offset + AHRS_UPDATE_ALTITUDE_VALUE_INDEX as usize);
        u.base.fused_heading = decode_protocol_unsigned_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_FUSED_HEADING_VALUE_INDEX as usize,
        );
        u.base.linear_accel_x = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX as usize,
        );
        u.base.linear_accel_y = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX as usize,
        );
        u.base.linear_accel_z = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX as usize,
        );
        u.cal_mag_x = decode_i16(buffer, offset + AHRS_UPDATE_CAL_MAG_X_VALUE_INDEX as usize);
        u.cal_mag_y = decode_i16(buffer, offset + AHRS_UPDATE_CAL_MAG_Y_VALUE_INDEX as usize);
        u.cal_mag_z = decode_i16(buffer, offset + AHRS_UPDATE_CAL_MAG_Z_VALUE_INDEX as usize);
        u.mag_field_norm_ratio = decode_protocol_unsigned_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_CAL_MAG_NORM_RATIO_VALUE_INDEX as usize,
        );
        u.mag_field_norm_scalar = decode_protocol1616_float(
            buffer,
            offset + AHRS_UPDATE_CAL_MAG_SCALAR_VALUE_INDEX as usize,
        );
        u.base.mpu_temp = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRS_UPDATE_MPU_TEMP_VAUE_INDEX as usize,
        );
        u.raw_mag_x = decode_i16(buffer, offset + AHRS_UPDATE_RAW_MAG_X_VALUE_INDEX as usize);
        u.raw_mag_y = decode_i16(buffer, offset + AHRS_UPDATE_RAW_MAG_Y_VALUE_INDEX as usize);
        u.raw_mag_z = decode_i16(buffer, offset + AHRS_UPDATE_RAW_MAG_Z_VALUE_INDEX as usize);
        /* AHRSPosUpdate:  Quaternions are signed int (16-bit resolution); divide by 32768 to yield +/- 1 radians */
        u.base.quat_w =
            decode_i16(buffer, offset + AHRS_UPDATE_QUAT_W_VALUE_INDEX as usize) as f32 / 32768.0;
        u.base.quat_x =
            decode_i16(buffer, offset + AHRS_UPDATE_QUAT_X_VALUE_INDEX as usize) as f32 / 32768.0;
        u.base.quat_y =
            decode_i16(buffer, offset + AHRS_UPDATE_QUAT_Y_VALUE_INDEX as usize) as f32 / 32768.0;
        u.base.quat_z =
            decode_i16(buffer, offset + AHRS_UPDATE_QUAT_Z_VALUE_INDEX as usize) as f32 / 32768.0;
        u.base.barometric_pressure = decode_protocol1616_float(
            buffer,
            offset + AHRS_UPDATE_BARO_PRESSURE_VALUE_INDEX as usize,
        );
        // u.base.baro_temp = decode_protocol_signed_hundredths_float(
        //     buffer,
        //     offset + AHRS_UPDATE_BARO_TEMP_VAUE_INDEX as usize,
        // );
        u.base.op_status = buffer[offset + AHRS_UPDATE_OPSTATUS_VALUE_INDEX as usize];
        u.base.sensor_status = buffer[offset + AHRS_UPDATE_SENSOR_STATUS_VALUE_INDEX as usize];
        u.base.cal_status = buffer[offset + AHRS_UPDATE_CAL_STATUS_VALUE_INDEX as usize];
        u.base.selftest_status = buffer[offset + AHRS_UPDATE_SELFTEST_STATUS_VALUE_INDEX as usize];
        return AHRS_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_ahrspos_update(
    buffer: &[u8],
    offset: usize,
    length: usize,
    u: &mut AHRSPosUpdate,
    base: &mut AHRSUpdateBase,
) -> i32 {
    if length < AHRSPOS_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == AHRSPOS_UPDATE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_AHRSPOS_UPDATE
    {
        if !verify_checksum(
            buffer,
            offset,
            AHRSPOS_UPDATE_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }
        base.yaw = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_YAW_VALUE_INDEX as usize,
        );
        base.pitch = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_ROLL_VALUE_INDEX as usize,
        ); /* FIXME */
        base.roll = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_PITCH_VALUE_INDEX as usize,
        ); /* FIXME */
        base.compass_heading = decode_protocol_unsigned_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_HEADING_VALUE_INDEX as usize,
        );
        base.altitude = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_UPDATE_ALTITUDE_VALUE_INDEX as usize,
        );
        base.fused_heading = decode_protocol_unsigned_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_FUSED_HEADING_VALUE_INDEX as usize,
        );
        base.linear_accel_x = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRSPOS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX as usize,
        );
        base.linear_accel_y = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRSPOS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX as usize,
        );
        base.linear_accel_z = decode_protocol_signed_thousandths_float(
            buffer,
            offset + AHRSPOS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX as usize,
        );
        u.vel_x =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_VEL_X_VALUE_INDEX as usize);
        u.vel_y =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_VEL_Y_VALUE_INDEX as usize);
        u.vel_z =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_VEL_Z_VALUE_INDEX as usize);
        u.disp_x =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_DISP_X_VALUE_INDEX as usize);
        u.disp_y =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_DISP_Y_VALUE_INDEX as usize);
        u.disp_z =
            decode_protocol1616_float(buffer, offset + AHRSPOS_UPDATE_DISP_Z_VALUE_INDEX as usize);
        base.mpu_temp = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_UPDATE_MPU_TEMP_VAUE_INDEX as usize,
        );
        /* AHRSPosUpdate:  Quaternions are signed int (16-bit resolution); divide by 32768 to yield +/- 1 radians */
        base.quat_w = decode_i16(buffer, offset + AHRSPOS_UPDATE_QUAT_W_VALUE_INDEX as usize)
            as f32
            / 32768.0;
        base.quat_x = decode_i16(buffer, offset + AHRSPOS_UPDATE_QUAT_X_VALUE_INDEX as usize)
            as f32
            / 32768.0;
        base.quat_y = decode_i16(buffer, offset + AHRSPOS_UPDATE_QUAT_Y_VALUE_INDEX as usize)
            as f32
            / 32768.0;
        base.quat_z = decode_i16(buffer, offset + AHRSPOS_UPDATE_QUAT_Z_VALUE_INDEX as usize)
            as f32
            / 32768.0;
        base.op_status = buffer[offset + AHRSPOS_UPDATE_OPSTATUS_VALUE_INDEX as usize];
        base.sensor_status = buffer[offset + AHRSPOS_UPDATE_SENSOR_STATUS_VALUE_INDEX as usize];
        base.cal_status = buffer[offset + AHRSPOS_UPDATE_CAL_STATUS_VALUE_INDEX as usize];
        base.selftest_status = buffer[offset + AHRSPOS_UPDATE_SELFTEST_STATUS_VALUE_INDEX as usize];
        return AHRSPOS_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_ahrspos_tsupdate(
    buffer: &[u8],
    offset: usize,
    length: usize,
    u: &mut AHRSPosTSUpdate,
    base: &mut AHRSUpdateBase,
) -> i32 {
    if length < AHRSPOS_TS_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == AHRSPOS_TS_UPDATE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_AHRSPOS_TS_UPDATE
    {
        if !verify_checksum(
            buffer,
            offset,
            AHRSPOS_TS_UPDATE_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }
        base.yaw =
            decode_protocol1616_float(buffer, offset + AHRSPOS_TS_UPDATE_YAW_VALUE_INDEX as usize);
        base.pitch =
            decode_protocol1616_float(buffer, offset + AHRSPOS_TS_UPDATE_ROLL_VALUE_INDEX as usize); /* FIXME */
        base.roll = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_PITCH_VALUE_INDEX as usize,
        ); /* FIXME */
        base.compass_heading = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_HEADING_VALUE_INDEX as usize,
        );
        base.altitude = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_ALTITUDE_VALUE_INDEX as usize,
        );
        base.fused_heading = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_FUSED_HEADING_VALUE_INDEX as usize,
        );
        base.linear_accel_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX as usize,
        );
        base.linear_accel_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX as usize,
        );
        base.linear_accel_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX as usize,
        );
        u.pos.vel_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_VEL_X_VALUE_INDEX as usize,
        );
        u.pos.vel_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_VEL_Y_VALUE_INDEX as usize,
        );
        u.pos.vel_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_VEL_Z_VALUE_INDEX as usize,
        );
        u.pos.disp_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_DISP_X_VALUE_INDEX as usize,
        );
        u.pos.disp_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_DISP_Y_VALUE_INDEX as usize,
        );
        u.pos.disp_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_DISP_Z_VALUE_INDEX as usize,
        );
        base.mpu_temp = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_MPU_TEMP_VAUE_INDEX as usize,
        );
        /* AHRSPosTSUpdate:  Quaternions are 16.16 format (32-bit resolution). */
        base.quat_w = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_QUAT_W_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_QUAT_X_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_QUAT_Y_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_UPDATE_QUAT_Z_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.op_status = buffer[offset + AHRSPOS_TS_UPDATE_OPSTATUS_VALUE_INDEX as usize];
        base.sensor_status = buffer[offset + AHRSPOS_TS_UPDATE_SENSOR_STATUS_VALUE_INDEX as usize];
        base.cal_status = buffer[offset + AHRSPOS_TS_UPDATE_CAL_STATUS_VALUE_INDEX as usize];
        base.selftest_status =
            buffer[offset + AHRSPOS_TS_UPDATE_SELFTEST_STATUS_VALUE_INDEX as usize];
        u.timestamp = decode_u32(buffer, offset + AHRSPOS_TS_UPDATE_TIMESTAMP_INDEX as usize);
        return AHRSPOS_TS_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn decode_ahrspos_tsraw_update(
    buffer: &[u8],
    offset: usize,
    length: usize,
    u: &mut AHRSPosTSRawUpdate,
    base: &mut AHRSUpdateBase,
) -> i32 {
    if length < AHRSPOS_TS_RAW_UPDATE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == AHRSPOS_TS_RAW_UPDATE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_AHRSPOS_TS_RAW_UPDATE
    {
        if !verify_checksum(
            buffer,
            offset,
            AHRSPOS_TS_RAW_UPDATE_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }
        base.yaw = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_YAW_VALUE_INDEX as usize,
        );
        base.pitch = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_ROLL_VALUE_INDEX as usize,
        ); /* FIXME */
        base.roll = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_PITCH_VALUE_INDEX as usize,
        ); /* FIXME */
        base.compass_heading = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_HEADING_VALUE_INDEX as usize,
        );
        base.altitude = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_ALTITUDE_VALUE_INDEX as usize,
        );
        base.fused_heading = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_FUSED_HEADING_VALUE_INDEX as usize,
        );
        base.linear_accel_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_X_VALUE_INDEX as usize,
        );
        base.linear_accel_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_Y_VALUE_INDEX as usize,
        );
        base.linear_accel_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_LINEAR_ACCEL_Z_VALUE_INDEX as usize,
        );
        u.pos.pos.vel_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_VEL_X_VALUE_INDEX as usize,
        );
        u.pos.pos.vel_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_VEL_Y_VALUE_INDEX as usize,
        );
        u.pos.pos.vel_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_VEL_Z_VALUE_INDEX as usize,
        );
        u.pos.pos.disp_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_DISP_X_VALUE_INDEX as usize,
        );
        u.pos.pos.disp_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_DISP_Y_VALUE_INDEX as usize,
        );
        u.pos.pos.disp_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_DISP_Z_VALUE_INDEX as usize,
        );
        base.mpu_temp = decode_protocol_signed_hundredths_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_MPU_TEMP_VAUE_INDEX as usize,
        );
        /* AHRSPosTSUpdate:  Quaternions are 16.16 format (32-bit resolution). */
        base.quat_w = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_QUAT_W_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_x = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_QUAT_X_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_y = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_QUAT_Y_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.quat_z = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_QUAT_Z_VALUE_INDEX as usize,
        ) as f32
            / 32768.0;
        base.op_status = buffer[offset + AHRSPOS_TS_RAW_UPDATE_OPSTATUS_VALUE_INDEX as usize];
        base.sensor_status =
            buffer[offset + AHRSPOS_TS_RAW_UPDATE_SENSOR_STATUS_VALUE_INDEX as usize];
        base.cal_status = buffer[offset + AHRSPOS_TS_RAW_UPDATE_CAL_STATUS_VALUE_INDEX as usize];
        base.selftest_status =
            buffer[offset + AHRSPOS_TS_RAW_UPDATE_SELFTEST_STATUS_VALUE_INDEX as usize];
        u.pos.timestamp = decode_u32(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_TIMESTAMP_INDEX as usize,
        );

        u.raw_gyro_x = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_GYRO_X_VALUE_INDEX as usize,
        );
        u.raw_gyro_y = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_GYRO_Y_VALUE_INDEX as usize,
        );
        u.raw_gyro_z = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_GYRO_Z_VALUE_INDEX as usize,
        );
        u.accel_x = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_ACC_X_VALUE_INDEX as usize,
        );
        u.accel_y = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_ACC_Y_VALUE_INDEX as usize,
        );
        u.accel_z = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_ACC_Z_VALUE_INDEX as usize,
        );
        u.delta_t_sec = decode_protocol1616_float(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_LAST_SAMPLE_DELTA_SEC_INDEX as usize,
        );
        u.gyro_bias_updated = decode_i16(
            buffer,
            offset + AHRSPOS_TS_RAW_UPDATE_GYRO_BIAS_UPDATED_BOOL_INDEX as usize,
        ) != 0;

        return AHRSPOS_TS_RAW_UPDATE_MESSAGE_LENGTH;
    }
    return 0;
}

/* Mag Cal, Tuning Variable, or Board ID Retrieval Request */
pub fn encode_data_get_request(buffer: &mut [u8], ty: u8, var_id: u8) -> i32 {
    // Header
    buffer[0] = PACKET_START_CHAR;
    buffer[1] = BINARY_PACKET_INDICATOR_CHAR as u8;
    buffer[2] = DATA_REQUEST_MESSAGE_LENGTH as u8 - 2;
    buffer[3] = MSGID_DATA_REQUEST;
    // Data
    buffer[DATA_REQUEST_DATATYPE_VALUE_INDEX as usize] = ty;
    buffer[DATA_REQUEST_VARIABLEID_VALUE_INDEX as usize] = var_id;
    // Footer
    encode_termination(
        buffer,
        DATA_REQUEST_MESSAGE_LENGTH as usize,
        DATA_REQUEST_MESSAGE_LENGTH as usize - 4,
    );
    return DATA_REQUEST_MESSAGE_LENGTH;
}

/* Mag Cal Data Storage Request */
pub fn encode_mag_cal_data_set_request(buffer: &mut [u8], d: &MagCalData) -> i32 {
    // Header
    buffer[0] = PACKET_START_CHAR;
    buffer[1] = BINARY_PACKET_INDICATOR_CHAR as u8;
    buffer[2] = MAG_CAL_CMD_MESSAGE_LENGTH as u8 - 2;
    buffer[3] = MSGID_MAG_CAL_CMD;

    // Data
    buffer[MAG_CAL_DATA_ACTION_VALUE_INDEX as usize] = d.action;
    for (i, val) in d.mag_bias.iter().enumerate() {
        encode_i16(*val, buffer, MAG_X_BIAS_VALUE_INDEX as usize + (i * 2));
    }
    for (i, row) in d.mag_xform.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            encode_protocol1616_float(
                *val,
                buffer,
                MAG_XFORM_1_1_VALUE_INDEX as usize + (i * 6) + (j * 2),
            );
        }
    }
    encode_protocol1616_float(
        d.earth_mag_field_norm,
        buffer,
        MAG_CAL_EARTH_MAG_FIELD_NORM_VALUE_INDEX as usize,
    );
    // Footer
    encode_termination(
        buffer,
        MAG_CAL_CMD_MESSAGE_LENGTH as usize,
        MAG_CAL_CMD_MESSAGE_LENGTH as usize - 4,
    );
    return MAG_CAL_CMD_MESSAGE_LENGTH;
}

/* Mag Cal Data Retrieval Response */
pub fn decode_mag_cal_data_get_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    d: &mut MagCalData,
) -> i32 {
    if length < MAG_CAL_CMD_MESSAGE_LENGTH as usize {
        return 0;
    };
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == MAG_CAL_CMD_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_MAG_CAL_CMD
    {
        if !verify_checksum(buffer, offset, MAG_CAL_CMD_MESSAGE_CHECKSUM_INDEX as usize) {
            return 0;
        }

        d.action = buffer[offset + MAG_CAL_DATA_ACTION_VALUE_INDEX as usize];
        for (i, val) in d.mag_bias.iter_mut().enumerate() {
            *val = decode_i16(buffer, offset + MAG_X_BIAS_VALUE_INDEX as usize + (i * 2));
        }
        for (i, row) in d.mag_xform.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate() {
                *val = decode_protocol1616_float(
                    buffer,
                    offset + MAG_XFORM_1_1_VALUE_INDEX as usize + (i * 6) + (j * 2),
                );
            }
        }
        d.earth_mag_field_norm = decode_protocol1616_float(
            buffer,
            offset + MAG_CAL_EARTH_MAG_FIELD_NORM_VALUE_INDEX as usize,
        );
        return MAG_CAL_CMD_MESSAGE_LENGTH;
    }
    return 0;
}

/* Tuning Variable Storage Request */
pub fn encode_tuning_var_set_request(buffer: &mut [u8], r: &TuningVar) -> i32 {
    // Header
    buffer[0] = PACKET_START_CHAR;
    buffer[1] = BINARY_PACKET_INDICATOR_CHAR as u8;
    buffer[2] = FUSION_TUNING_CMD_MESSAGE_LENGTH as u8 - 2;
    buffer[3] = MSGID_FUSION_TUNING_CMD;
    // Data
    buffer[FUSION_TUNING_DATA_ACTION_VALUE_INDEX as usize] = r.action;
    buffer[FUSION_TUNING_CMD_VAR_ID_VALUE_INDEX as usize] = r.var_id;
    encode_protocol1616_float(r.value, buffer, FUSION_TUNING_CMD_VAR_VALUE_INDEX as usize);
    // Footer
    encode_termination(
        buffer,
        FUSION_TUNING_CMD_MESSAGE_LENGTH as usize,
        FUSION_TUNING_CMD_MESSAGE_LENGTH as usize - 4,
    );
    return FUSION_TUNING_CMD_MESSAGE_LENGTH;
}

/* Tuning Variable Retrieval Response */
pub fn decode_tuning_var_get_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    r: &mut TuningVar,
) -> i32 {
    if length < FUSION_TUNING_CMD_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == FUSION_TUNING_CMD_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_FUSION_TUNING_CMD
    {
        if !verify_checksum(
            buffer,
            offset,
            FUSION_TUNING_CMD_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }

        // Data
        r.action = buffer[offset + FUSION_TUNING_DATA_ACTION_VALUE_INDEX as usize];
        r.var_id = buffer[offset + FUSION_TUNING_CMD_VAR_ID_VALUE_INDEX as usize];
        r.value =
            decode_protocol1616_float(buffer, offset + FUSION_TUNING_CMD_VAR_VALUE_INDEX as usize);
        return FUSION_TUNING_CMD_MESSAGE_LENGTH;
    }
    return 0;
}

pub fn encode_integration_control_cmd(buffer: &mut [u8], u: &IntegrationControl) -> i32 {
    // Header
    buffer[0] = PACKET_START_CHAR;
    buffer[1] = BINARY_PACKET_INDICATOR_CHAR as u8;
    buffer[2] = INTEGRATION_CONTROL_CMD_MESSAGE_LENGTH as u8 - 2;
    buffer[3] = MSGID_INTEGRATION_CONTROL_CMD;
    // Data
    buffer[INTEGRATION_CONTROL_CMD_ACTION_INDEX as usize] = u.action;
    encode_u32(
        u.parameter,
        buffer,
        INTEGRATION_CONTROL_CMD_PARAMETER_INDEX as usize,
    );
    // Footer
    encode_termination(
        buffer,
        INTEGRATION_CONTROL_CMD_MESSAGE_LENGTH as usize,
        INTEGRATION_CONTROL_CMD_MESSAGE_LENGTH as usize - 4,
    );
    return INTEGRATION_CONTROL_CMD_MESSAGE_LENGTH;
}

pub fn decode_integration_control_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    u: &mut IntegrationControl,
) -> i32 {
    if length < INTEGRATION_CONTROL_RESP_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == INTEGRATION_CONTROL_RESP_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_INTEGRATION_CONTROL_RESP
    {
        if !verify_checksum(
            buffer,
            offset,
            INTEGRATION_CONTROL_RESP_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }

        // Data
        u.action = buffer[offset + INTEGRATION_CONTROL_RESP_ACTION_INDEX as usize];
        u.parameter = decode_u32(
            buffer,
            offset + INTEGRATION_CONTROL_RESP_PARAMETER_INDEX as usize,
        );
        return INTEGRATION_CONTROL_RESP_MESSAGE_LENGTH;
    }
    return 0;
}

/* MagCal or Tuning Variable Storage Response */
pub fn decode_data_set_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    d: &mut DataSetResponse,
) -> i32 {
    if length < DATA_SET_RESPONSE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == DATA_SET_RESPONSE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_DATA_SET_RESPONSE
    {
        if !verify_checksum(
            buffer,
            offset,
            DATA_SET_RESPONSE_MESSAGE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }

        d.data_type = buffer[offset + DATA_SET_RESPONSE_DATATYPE_VALUE_INDEX as usize];
        d.var_id = buffer[offset + DATA_SET_RESPONSE_VARID_VALUE_INDEX as usize];
        d.status = buffer[offset + DATA_SET_RESPONSE_STATUS_VALUE_INDEX as usize];
        return DATA_SET_RESPONSE_MESSAGE_LENGTH;
    }
    return 0;
}

/* Board ID Retrieval Response */
pub fn decode_board_idget_response(
    buffer: &[u8],
    offset: usize,
    length: usize,
    id: &mut BoardID,
) -> i32 {
    if length < BOARD_IDENTITY_RESPONSE_MESSAGE_LENGTH as usize {
        return 0;
    }
    if buffer[offset + 0] == PACKET_START_CHAR
        && buffer[offset + 1] == BINARY_PACKET_INDICATOR_CHAR as u8
        && buffer[offset + 2] == BOARD_IDENTITY_RESPONSE_MESSAGE_LENGTH as u8 - 2
        && buffer[offset + 3] == MSGID_BOARD_IDENTITY_RESPONSE
    {
        if !verify_checksum(
            buffer,
            offset,
            BOARD_IDENTITY_RESPONSE_CHECKSUM_INDEX as usize,
        ) {
            return 0;
        }
        id.ty = buffer[offset + BOARD_IDENTITY_BOARDTYPE_VALUE_INDEX as usize];
        id.hw_rev = buffer[offset + BOARD_IDENTITY_HWREV_VALUE_INDEX as usize];
        id.fw_ver_major = buffer[offset + BOARD_IDENTITY_FW_VER_MAJOR as usize];
        id.fw_ver_minor = buffer[offset + BOARD_IDENTITY_FW_VER_MINOR as usize];
        id.fw_revision = decode_u16(
            buffer,
            offset + BOARD_IDENTITY_FW_VER_REVISION_VALUE_INDEX as usize,
        );
        for (i, val) in id.unique_id.iter_mut().enumerate() {
            *val = buffer[offset + BOARD_IDENTITY_UNIQUE_ID_0 as usize + i];
        }
        return BOARD_IDENTITY_RESPONSE_MESSAGE_LENGTH;
    }
    return 0;
}
