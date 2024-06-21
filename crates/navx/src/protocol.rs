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
pub struct AHRSUpdate {}

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

pub fn decode_u32(buffer: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap())
}

pub fn decode_i16(buffer: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap())
}

/// -327.68 to +327.68
pub fn decode_protocol_signed_hundredths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_i16(buffer, offset) as f32 / 100.0
}

/// 0 to 655.35
pub fn decode_protocol_unsigned_hundredths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u16(buffer, offset) as f32 / 100.0
}

/// -32.768 to +32.768
pub fn decode_protocol_signed_thousandths_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u16(buffer, offset) as f32 / 1000.0
}

/// In units of -1 to 1, multiplied by 16384

/// <int16>.<uint16> (-32768.9999 to 32767.9999)
pub fn decode_protocol1616_float(buffer: &[u8], offset: usize) -> f32 {
    decode_u32(buffer, offset) as f32 / 65536.0
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
