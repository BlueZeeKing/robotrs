use std::{fmt::Formatter, sync::Arc, thread, time::Duration, usize};

use bitflags::bitflags;
use io::IO;
use parking_lot::Mutex;
use protocol::{AHRSPosUpdate, AHRSUpdateBase, BoardID, GyroUpdate};
use thiserror::Error;

pub mod io;
pub mod protocol;
pub mod registers;

pub const DELAY_OVERHEAD_SECONDS: f64 = 0.004;

#[derive(Clone)]
pub struct NavX {
    data: Arc<Mutex<State>>,
    config: Arc<Mutex<Config>>,
}

impl NavX {
    pub fn new<I: IO + Send + 'static>(
        mut io: I,
        update_rate: u8,
    ) -> (Self, oneshot::Receiver<()>) {
        io.init().unwrap();

        io.write(registers::NAVX_REG_UPDATE_RATE_HZ as u8, update_rate)
            .unwrap();

        thread::sleep(Duration::from_millis(50));

        let config = get_configuration(&mut io).unwrap();

        let mut delay_time = 1.0 / update_rate as f64;
        if delay_time > DELAY_OVERHEAD_SECONDS {
            delay_time -= DELAY_OVERHEAD_SECONDS;
        }

        let mut has_displacement = config
            .state
            .capability_flags
            .contains(Capabilities::DISPLACEMENT);

        let config = Arc::new(Mutex::new(config));
        let data = Arc::new(Mutex::new(State::default()));

        let config2 = config.clone();
        let data2 = data.clone();

        let (sender, reciever) = oneshot::channel();

        let mut sender = Some(sender);

        thread::spawn(move || loop {
            let (state, board_state_update) = if has_displacement {
                let mut buffer =
                    [0; registers::NAVX_REG_LAST + 1 - registers::NAVX_REG_UPDATE_RATE_HZ];

                get_data(&mut io, &mut buffer, has_displacement)
            } else {
                let mut buffer = [0; registers::NAVX_REG_QUAT_OFFSET_Z_H + 1
                    - registers::NAVX_REG_UPDATE_RATE_HZ];

                get_data(&mut io, &mut buffer, has_displacement)
            }
            .unwrap();

            if let Some(mut config) = config2.try_lock() {
                config.merge(&board_state_update);
            }

            if let Some(mut data) = data2.try_lock() {
                *data = state;
                if let Some(data) = sender.take() {
                    let _ = data.send(());
                }
            }

            if board_state_update.update_rate_hz != update_rate {
                io.write(registers::NAVX_REG_UPDATE_RATE_HZ as u8, update_rate)
                    .unwrap();
            }

            has_displacement = board_state_update
                .capability_flags
                .contains(Capabilities::DISPLACEMENT);

            thread::sleep(Duration::from_secs_f64(delay_time));
        });

        (NavX { config, data }, reciever)
    }

    pub fn get_data(&self) -> State {
        self.data.lock().clone()
    }

    /// Returned in cw+ and degrees, aka the opposite of what we want
    pub fn heading(&self) -> f32 {
        self.data.lock().base.yaw
    }

    pub fn get_config(&self) -> Config {
        self.config.lock().clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub id: BoardID,
    pub state: BoardState,
}

impl Config {
    pub fn merge(&mut self, other: &StateUpdate) {
        self.state.capability_flags = other.capability_flags.clone();
        self.state.update_rate_hz = other.update_rate_hz;
        self.state.accel_fsr_g = other.accel_fsr_g;
        self.state.gyro_fsr_dps = other.gyro_fsr_dps;
    }
}

#[derive(Debug, Default, Clone)]
pub struct BoardState {
    pub op_status: u8,
    pub sensor_status: u16,
    pub cal_status: u8,
    pub selftest_status: u8,
    pub capability_flags: Capabilities,
    pub update_rate_hz: u8,
    pub accel_fsr_g: u16,
    pub gyro_fsr_dps: u16,
}

bitflags! {
    #[derive(Default, Debug, Clone)]
    pub struct Capabilities: u16 {
        const OMNIMOUNT = 0x0004;
        const DISPLACEMENT = 0x0040;
        const YAW_RESET = 0x0080;
        const POS_TIMESTAMP = 0x0100;
    }
}

#[derive(Error)]
enum ConfigError<I: IO> {
    #[error("IO error: {0}")]
    Io(I::Error),
    #[error("Invalid WHO_AM_I value")]
    InvalidWhoAmI,
}

impl<I: IO> std::fmt::Debug for ConfigError<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Io(arg0) => f.debug_tuple("Io").field(arg0).finish(),
            Self::InvalidWhoAmI => write!(f, "InvalidWhoAmI"),
        }
    }
}

fn get_configuration<I: IO>(io: &mut I) -> Result<Config, ConfigError<I>> {
    let mut buffer = [0; registers::NAVX_REG_SENSOR_STATUS_H as usize + 1];

    io.read(registers::NAVX_REG_WHOAMI as u8, &mut buffer)
        .map_err(|err| ConfigError::Io(err))?;

    if buffer[registers::NAVX_REG_WHOAMI] != 0x32 {
        return Err(ConfigError::InvalidWhoAmI);
    }

    let mut board_id = BoardID::default();

    board_id.hw_rev = buffer[registers::NAVX_REG_HW_REV as usize];
    board_id.fw_ver_major = buffer[registers::NAVX_REG_FW_VER_MAJOR as usize];
    board_id.fw_ver_minor = buffer[registers::NAVX_REG_FW_VER_MINOR as usize];
    board_id.ty = buffer[registers::NAVX_REG_WHOAMI as usize];

    let mut board_state = BoardState::default();

    board_state.cal_status = buffer[registers::NAVX_REG_CAL_STATUS as usize];
    board_state.op_status = buffer[registers::NAVX_REG_OP_STATUS as usize];
    board_state.selftest_status = buffer[registers::NAVX_REG_SELFTEST_STATUS as usize];
    board_state.sensor_status =
        protocol::decode_u16(&buffer, registers::NAVX_REG_SENSOR_STATUS_L as usize);
    board_state.gyro_fsr_dps =
        protocol::decode_u16(&buffer, registers::NAVX_REG_GYRO_FSR_DPS_L as usize);
    board_state.accel_fsr_g = buffer[registers::NAVX_REG_ACCEL_FSR_G as usize] as u16;
    board_state.update_rate_hz = buffer[registers::NAVX_REG_UPDATE_RATE_HZ as usize];
    board_state.capability_flags = Capabilities::from_bits_truncate(protocol::decode_u16(
        &buffer,
        registers::NAVX_REG_CAPABILITY_FLAGS_L as usize,
    ));

    Ok(Config {
        id: board_id,
        state: board_state,
    })
}

#[derive(Clone, Default)]
pub struct State {
    pub base: AHRSUpdateBase,
    pub pos: Option<AHRSPosUpdate>,
    pub raw: GyroUpdate,
}

#[derive(Clone, Default)]
pub struct StateUpdate {
    pub capability_flags: Capabilities,
    pub update_rate_hz: u8,
    pub accel_fsr_g: u16,
    pub gyro_fsr_dps: u16,
}

fn get_data<I: IO>(
    io: &mut I,
    buffer: &mut [u8],
    displacement: bool,
) -> Result<(State, StateUpdate), ConfigError<I>> {
    let start = registers::NAVX_REG_UPDATE_RATE_HZ;

    io.read(start as u8, buffer)
        .map_err(|err| ConfigError::Io(err))?;

    // let sensor_timestamp = decode_u32(buffer, registers::NAVX_REG_TIMESTAMP_L_L - start);

    let base = AHRSUpdateBase {
        yaw: protocol::decode_protocol_signed_hundredths_float(
            buffer,
            registers::NAVX_REG_YAW_L - start,
        ),
        pitch: protocol::decode_protocol_signed_hundredths_float(
            buffer,
            registers::NAVX_REG_PITCH_L - start,
        ),
        roll: protocol::decode_protocol_signed_hundredths_float(
            buffer,
            registers::NAVX_REG_ROLL_L - start,
        ),
        compass_heading: protocol::decode_protocol_unsigned_hundredths_float(
            buffer,
            registers::NAVX_REG_HEADING_L - start,
        ),
        altitude: protocol::decode_protocol1616_float(
            buffer,
            registers::NAVX_REG_ALTITUDE_D_L - start,
        ),
        fused_heading: protocol::decode_protocol_unsigned_hundredths_float(
            buffer,
            registers::NAVX_REG_FUSED_HEADING_L - start,
        ),
        linear_accel_x: protocol::decode_protocol_signed_thousandths_float(
            buffer,
            registers::NAVX_REG_LINEAR_ACC_X_L - start,
        ),
        linear_accel_y: protocol::decode_protocol_signed_thousandths_float(
            buffer,
            registers::NAVX_REG_LINEAR_ACC_Y_L - start,
        ),
        linear_accel_z: protocol::decode_protocol_signed_thousandths_float(
            buffer,
            registers::NAVX_REG_LINEAR_ACC_Z_L - start,
        ),
        mpu_temp: protocol::decode_protocol_signed_hundredths_float(
            buffer,
            registers::NAVX_REG_MPU_TEMP_C_L - start,
        ),
        quat_w: protocol::decode_i16(buffer, registers::NAVX_REG_QUAT_W_L - start) as f32 / 32768.0,
        quat_x: protocol::decode_i16(buffer, registers::NAVX_REG_QUAT_X_L - start) as f32 / 32768.0,
        quat_y: protocol::decode_i16(buffer, registers::NAVX_REG_QUAT_Y_L - start) as f32 / 32768.0,
        quat_z: protocol::decode_i16(buffer, registers::NAVX_REG_QUAT_Z_L - start) as f32 / 32768.0,
        barometric_pressure: protocol::decode_protocol1616_float(
            buffer,
            registers::NAVX_REG_PRESSURE_DL - start,
        ),
        op_status: buffer[registers::NAVX_REG_OP_STATUS - start],
        sensor_status: buffer[registers::NAVX_REG_SENSOR_STATUS_L - start],
        cal_status: buffer[registers::NAVX_REG_CAL_STATUS - start],
        selftest_status: buffer[registers::NAVX_REG_SELFTEST_STATUS - start],
    };

    let pos = if displacement {
        Some(AHRSPosUpdate {
            vel_x: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_VEL_X_I_L - start,
            ),
            vel_y: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_VEL_Y_I_L - start,
            ),
            vel_z: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_VEL_Z_I_L - start,
            ),
            disp_x: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_DISP_X_I_L - start,
            ),
            disp_y: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_DISP_Y_I_L - start,
            ),
            disp_z: protocol::decode_protocol1616_float(
                buffer,
                registers::NAVX_REG_DISP_Z_I_L - start,
            ),
        })
    } else {
        None
    };

    let board_state = StateUpdate {
        update_rate_hz: buffer[registers::NAVX_REG_UPDATE_RATE_HZ - start],
        gyro_fsr_dps: protocol::decode_u16(buffer, registers::NAVX_REG_GYRO_FSR_DPS_L - start),
        accel_fsr_g: buffer[registers::NAVX_REG_ACCEL_FSR_G - start] as u16,
        capability_flags: Capabilities::from_bits_truncate(protocol::decode_u16(
            buffer,
            registers::NAVX_REG_CAPABILITY_FLAGS_L - start,
        )),
    };

    let raw = GyroUpdate {
        gyro_x: protocol::decode_i16(buffer, registers::NAVX_REG_GYRO_X_L - start),
        gyro_y: protocol::decode_i16(buffer, registers::NAVX_REG_GYRO_Y_L - start),
        gyro_z: protocol::decode_i16(buffer, registers::NAVX_REG_GYRO_Z_L - start),
        accel_x: protocol::decode_i16(buffer, registers::NAVX_REG_ACC_X_L - start),
        accel_y: protocol::decode_i16(buffer, registers::NAVX_REG_ACC_Y_L - start),
        accel_z: protocol::decode_i16(buffer, registers::NAVX_REG_ACC_Z_L - start),
        mag_x: protocol::decode_i16(buffer, registers::NAVX_REG_MAG_X_L - start),
        mag_y: protocol::decode_i16(buffer, registers::NAVX_REG_MAG_Y_L - start),
        mag_z: protocol::decode_i16(buffer, registers::NAVX_REG_MAG_Z_L - start),
        temp_c: base.mpu_temp,
    };

    Ok((State { base, pos, raw }, board_state))
}
