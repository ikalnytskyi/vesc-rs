use bitflags::bitflags;

use super::packer::{Packer, Unpacker};

const CRC16: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
const FRAME_END: u8 = 3;
const FRAME_START_SHORT: u8 = 2;

/// Errors that can occur during command encoding or decoding.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    #[error("the output buffer provided for encoding is too small")]
    BufferTooSmall,

    #[error("the input buffer for decoding does not contain enough data for a complete frame")]
    IncompleteData,

    #[error("checksum mismatch: expected 0x{expected:X}, but found 0x{actual:X}")]
    ChecksumMismatch { expected: u16, actual: u16 },

    #[error("unrecognized or unsupported packet: {id}")]
    UnknownPacket { id: u8 },

    #[error("the frame structure is frame")]
    InvalidFrame,
}

#[repr(u8)]
enum CommandId {
    GetValues = 4,
    SetCurrent = 6,
    SetRpm = 8,
    ForwardCan = 34,
    GetValuesSelective = 50,
}

impl TryFrom<u8> for CommandId {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            id if id == CommandId::GetValues as u8 => Ok(CommandId::GetValues),
            id if id == CommandId::SetCurrent as u8 => Ok(CommandId::SetCurrent),
            id if id == CommandId::SetRpm as u8 => Ok(CommandId::SetRpm),
            id if id == CommandId::ForwardCan as u8 => Ok(CommandId::ForwardCan),
            id if id == CommandId::GetValuesSelective as u8 => Ok(CommandId::GetValuesSelective),
            id => Err(Error::UnknownPacket { id }),
        }
    }
}

/// A bitmask used with [`Command::GetValuesSelective`] to request specific
/// telemetry fields. This allows for efficient communication by requesting only
/// the data you need, reducing bandwidth and processing overhead. Each flag
/// corresponds to a field in the [`Values`] struct.
///
/// # Example
///
/// ```rust
/// use vesc::ValuesMask;
///
/// let mask = ValuesMask::RPM | ValuesMask::WATT_HOURS | ValuesMask::CONTROLLER_ID;
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ValuesMask(u32);

bitflags! {
    impl ValuesMask: u32 {
        const TEMP_MOSFET           = 1 << 0;
        const TEMP_MOTOR            = 1 << 1;
        const AVG_CURRENT_MOTOR     = 1 << 2;
        const AVG_CURRENT_INPUT     = 1 << 3;
        const AVG_CURRENT_D         = 1 << 4;
        const AVG_CURRENT_Q         = 1 << 5;
        const DUTY_CYCLE            = 1 << 6;
        const RPM                   = 1 << 7;
        const VOLTAGE_IN            = 1 << 8;
        const AMP_HOURS             = 1 << 9;
        const AMP_HOURS_CHARGED     = 1 << 10;
        const WATT_HOURS            = 1 << 11;
        const WATT_HOURS_CHARGED    = 1 << 12;
        const TACHOMETER            = 1 << 13;
        const TACHOMETER_ABS        = 1 << 14;
        const FAULT_CODE            = 1 << 15;
        const PID_POS               = 1 << 16;
        const CONTROLLER_ID         = 1 << 17;
        const TEMP_MOSFET_ALL       = 1 << 18;
        const AVG_VOLTAGE_D         = 1 << 19;
        const AVG_VOLTAGE_Q         = 1 << 20;
        const STATUS                = 1 << 21;
    }
}

/// Commands that can be sent to a VESC controller.
///
/// Each variant represents a different operation that can be performed on the
/// motor controller. Commands are encoded using the [`encode`] function and
/// sent over UART, USB, or other communication interfaces.
///
/// # Example
///
/// ```rust
/// use vesc::Command;
///
/// let command = Command::SetRpm(-1500);
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command<'a> {
    /// Requests the complete set of telemetry data from the VESC.
    GetValues,

    /// Sets the motor current in amperes. Positive values drive forward;
    /// negative values drive reverse.
    SetCurrent(f32),

    /// Sets the motor speed in revolutions per minute (RPM). Positive values
    /// drive forward; negative values drive reverse.
    SetRpm(i32),

    /// Forwards a command to another VESC controller on the CAN bus. Takes the
    /// target controller ID and the command to forward.
    ForwardCan(
        u8,
        #[cfg_attr(feature = "defmt", defmt(Debug2Format))] &'a Command<'a>,
    ),

    /// Requests a subset of telemetry data specified by a [`ValuesMask`]
    /// bitmask. Using a mask reduces communication overhead and processing time
    /// compared to [`GetValues`], making it more efficient when only selected
    /// data fields are needed.
    GetValuesSelective(ValuesMask),
}

impl<'a> Command<'a> {
    fn pack_into(&self, packer: &mut Packer) -> Result<(), Error> {
        match self {
            Self::GetValues => {
                packer.pack_u8(CommandId::GetValues as u8)?;
            }
            Self::SetCurrent(current) => {
                packer.pack_u8(CommandId::SetCurrent as u8)?;
                packer.pack_f32(*current, 1000.0)?;
            }
            Self::SetRpm(rpm) => {
                packer.pack_u8(CommandId::SetRpm as u8)?;
                packer.pack_i32(*rpm)?;
            }
            Self::ForwardCan(controller_id, command) => {
                packer.pack_u8(CommandId::ForwardCan as u8)?;
                packer.pack_u8(*controller_id)?;
                command.pack_into(packer)?;
            }
            Self::GetValuesSelective(mask) => {
                packer.pack_u8(CommandId::GetValuesSelective as u8)?;
                packer.pack_u32(mask.bits())?;
            }
        }
        Ok(())
    }
}

/// Telemetry data returned by the motor controller.
///
/// Contains temperatures, currents, voltages, rpm, and so on. Returned by
/// [`Command::GetValues`] or [`Command::GetValuesSelective`].
///
/// With [`Command::GetValuesSelective`], only the fields specified by the
/// [`ValuesMask`] are populated; all others remain at their default.
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Values {
    pub temp_mosfet: f32,
    pub temp_motor: f32,
    pub avg_current_motor: f32,
    pub avg_current_input: f32,
    pub avg_current_d: f32,
    pub avg_current_q: f32,
    pub duty_cycle: f32,
    pub rpm: f32,
    pub voltage_in: f32,
    pub amp_hours: f32,
    pub amp_hours_charged: f32,
    pub watt_hours: f32,
    pub watt_hours_charged: f32,
    pub tachometer: i32,
    pub tachometer_abs: i32,
    pub fault_code: u8,
    pub pid_pos: f32,
    pub controller_id: u8,
    pub temp_mosfet1: f32,
    pub temp_mosfet2: f32,
    pub temp_mosfet3: f32,
    pub avg_voltage_d: f32,
    pub avg_voltage_q: f32,
    pub status: u8,
}

/// Reply messages received from the VESC in response to commands.
///
/// These represent the various types of responses that can be received from the
/// controller after sending commands.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandReply {
    /// Complete telemetry data in response to [`Command::GetValues`]. Contains
    /// all available sensor readings and status information.
    GetValues(Values),

    /// Selective telemetry data in response to [`Command::GetValuesSelective`].
    /// Contains only the data fields that were requested via the
    /// [`ValuesMask`]. Non-requested fields will have default values.
    GetValuesSelective(Values),
}

impl CommandReply {
    fn unpack_from(unpacker: &mut Unpacker) -> Result<Self, Error> {
        Ok(match unpacker.unpack_u8()?.try_into()? {
            CommandId::GetValues => Self::unpack_get_values(unpacker)?,
            CommandId::GetValuesSelective => Self::unpack_get_values_selective(unpacker)?,
            id => return Err(Error::UnknownPacket { id: id as u8 }),
        })
    }

    fn unpack_get_values(unpacker: &mut Unpacker) -> Result<Self, Error> {
        let values = Values {
            temp_mosfet: unpacker.unpack_f16(10.0)?,
            temp_motor: unpacker.unpack_f16(10.0)?,
            avg_current_motor: unpacker.unpack_f32(100.0)?,
            avg_current_input: unpacker.unpack_f32(100.0)?,
            avg_current_d: unpacker.unpack_f32(100.0)?,
            avg_current_q: unpacker.unpack_f32(100.0)?,
            duty_cycle: unpacker.unpack_f16(1000.0)?,
            rpm: unpacker.unpack_f32(1.0)?,
            voltage_in: unpacker.unpack_f16(10.0)?,
            amp_hours: unpacker.unpack_f32(10000.0)?,
            amp_hours_charged: unpacker.unpack_f32(10000.0)?,
            watt_hours: unpacker.unpack_f32(10000.0)?,
            watt_hours_charged: unpacker.unpack_f32(10000.0)?,
            tachometer: unpacker.unpack_i32()?,
            tachometer_abs: unpacker.unpack_i32()?,
            fault_code: unpacker.unpack_u8()?,
            pid_pos: unpacker.unpack_f32(1000000.0)?,
            controller_id: unpacker.unpack_u8()?,
            temp_mosfet1: unpacker.unpack_f16(10.0)?,
            temp_mosfet2: unpacker.unpack_f16(10.0)?,
            temp_mosfet3: unpacker.unpack_f16(10.0)?,
            avg_voltage_d: unpacker.unpack_f32(1000.0)?,
            avg_voltage_q: unpacker.unpack_f32(1000.0)?,
            status: unpacker.unpack_u8()?,
        };
        Ok(CommandReply::GetValues(values))
    }

    fn unpack_get_values_selective(unpacker: &mut Unpacker) -> Result<Self, Error> {
        let mut values = Values::default();
        let mask = ValuesMask::from_bits_retain(unpacker.unpack_u32()?);

        if mask.contains(ValuesMask::TEMP_MOSFET) {
            values.temp_mosfet = unpacker.unpack_f16(10.0)?;
        }
        if mask.contains(ValuesMask::TEMP_MOTOR) {
            values.temp_motor = unpacker.unpack_f16(10.0)?;
        }
        if mask.contains(ValuesMask::AVG_CURRENT_MOTOR) {
            values.avg_current_motor = unpacker.unpack_f32(100.0)?;
        }
        if mask.contains(ValuesMask::AVG_CURRENT_INPUT) {
            values.avg_current_input = unpacker.unpack_f32(100.0)?;
        }
        if mask.contains(ValuesMask::AVG_CURRENT_D) {
            values.avg_current_d = unpacker.unpack_f32(100.0)?;
        }
        if mask.contains(ValuesMask::AVG_CURRENT_Q) {
            values.avg_current_q = unpacker.unpack_f32(100.0)?;
        }
        if mask.contains(ValuesMask::DUTY_CYCLE) {
            values.duty_cycle = unpacker.unpack_f16(1000.0)?;
        }
        if mask.contains(ValuesMask::RPM) {
            values.rpm = unpacker.unpack_f32(1.0)?;
        }
        if mask.contains(ValuesMask::VOLTAGE_IN) {
            values.voltage_in = unpacker.unpack_f16(10.0)?;
        }
        if mask.contains(ValuesMask::AMP_HOURS) {
            values.amp_hours = unpacker.unpack_f32(10000.0)?;
        }
        if mask.contains(ValuesMask::AMP_HOURS_CHARGED) {
            values.amp_hours_charged = unpacker.unpack_f32(10000.0)?;
        }
        if mask.contains(ValuesMask::WATT_HOURS) {
            values.watt_hours = unpacker.unpack_f32(10000.0)?;
        }
        if mask.contains(ValuesMask::WATT_HOURS_CHARGED) {
            values.watt_hours_charged = unpacker.unpack_f32(10000.0)?;
        }
        if mask.contains(ValuesMask::TACHOMETER) {
            values.tachometer = unpacker.unpack_i32()?;
        }
        if mask.contains(ValuesMask::TACHOMETER_ABS) {
            values.tachometer_abs = unpacker.unpack_i32()?;
        }
        if mask.contains(ValuesMask::FAULT_CODE) {
            values.fault_code = unpacker.unpack_u8()?;
        }
        if mask.contains(ValuesMask::PID_POS) {
            values.pid_pos = unpacker.unpack_f32(1000000.0)?;
        }
        if mask.contains(ValuesMask::CONTROLLER_ID) {
            values.controller_id = unpacker.unpack_u8()?;
        }
        if mask.contains(ValuesMask::TEMP_MOSFET_ALL) {
            values.temp_mosfet1 = unpacker.unpack_f16(10.0)?;
            values.temp_mosfet2 = unpacker.unpack_f16(10.0)?;
            values.temp_mosfet3 = unpacker.unpack_f16(10.0)?;
        }
        if mask.contains(ValuesMask::AVG_VOLTAGE_D) {
            values.avg_voltage_d = unpacker.unpack_f32(1000.0)?;
        }
        if mask.contains(ValuesMask::AVG_VOLTAGE_Q) {
            values.avg_voltage_q = unpacker.unpack_f32(1000.0)?;
        }
        if mask.contains(ValuesMask::STATUS) {
            values.status = unpacker.unpack_u8()?;
        }
        Ok(CommandReply::GetValuesSelective(values))
    }
}

/// Encodes a [`Command`] into a byte buffer.
///
/// Writes the encoded frame to `buf`. Returns the number of bytes written on
/// success, or an error if encoding fails.
///
/// # Example
///
///  ```no_run
///  use vesc::Command;
///
///  let mut buf = [0u8; 64];
///  match vesc::encode(Command::SetRpm(1500), &mut buf) {
///     Ok(len) => println!("encoded: {:?}", &buf[..len]),
///     _ => (),
///  }
/// ```
pub fn encode(command: Command, buf: &mut [u8]) -> Result<usize, Error> {
    let mut packer = Packer::new(buf);
    packer.pack_u8(FRAME_START_SHORT)?;
    packer.pack_u8(0)?;
    command.pack_into(&mut packer)?;
    let payload_len = packer.pos - 2;
    packer.buf[1] = payload_len as u8;
    packer.pack_u16(CRC16.checksum(&packer.buf[2..2 + payload_len]))?;
    packer.pack_u8(FRAME_END)?;
    Ok(packer.pos)
}

/// Decodes a [`CommandReply`] from a byte buffer.
///
/// Returns the consumed number of bytes and decoded reply on success, or an
/// error if the frame is invalid.
///
/// # Example
///
/// ```no_run
/// use vesc::CommandReply;
///
/// match vesc::decode(&[2, 7, 50, 0, 0, 1, 128, 0, 0, 4, 210, 1, 176, 254, 22, 3]) {
///     Ok((_, CommandReply::GetValuesSelective(values))) => {
///         let rpm = values.rpm;
///         let voltage_in = values.voltage_in;
///     }
///     _ => (),
/// }
/// ```
pub fn decode(buf: &[u8]) -> Result<(usize, CommandReply), Error> {
    let mut unpacker = Unpacker::new(buf);

    if unpacker.unpack_u8()? != FRAME_START_SHORT {
        return Err(Error::InvalidFrame);
    }
    let payload_len = unpacker.unpack_u8()? as usize;
    let reply = CommandReply::unpack_from(&mut unpacker)?;
    let payload = &unpacker.buf[unpacker.pos - payload_len..unpacker.pos];
    let checksum = unpacker.unpack_u16()?;
    let actual = CRC16.checksum(payload);
    if CRC16.checksum(payload) != checksum {
        return Err(Error::ChecksumMismatch {
            expected: checksum,
            actual,
        });
    }
    if unpacker.unpack_u8()? != FRAME_END {
        return Err(Error::InvalidFrame);
    }
    Ok((unpacker.pos, reply))
}
