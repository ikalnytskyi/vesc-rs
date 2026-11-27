//! Motor configuration replies and typed groups.
//!
//! [`McConf`] is the compact current-limit summary returned by the ordinary
//! streaming decoder. Use [`McConfView::decode`] on the retained received frame
//! to access all transmitted settings without enlarging [`crate::CommandReply`].

mod groups;
pub use groups::{AdditionalInfo, Bldc, Dc, Foc, General, McConfView, PidControllers};

use crate::{DecodeError, packer::Unpacker};

// Signature, configuration bytes, bytes between current scale and weakening.
const LAYOUTS: [(u32, usize, usize); 4] = [
    (1_065_524_471, 477, 223), // 6.05
    (788_332_866, 483, 227),   // 6.06
    (1_470_992_211, 488, 228), // 7.00
    (471_314_414, 492, 232),   // 8.00
];

fn layout_for_signature(signature: u32) -> Result<usize, DecodeError> {
    LAYOUTS
        .iter()
        .position(|layout| layout.0 == signature)
        .ok_or(DecodeError::UnsupportedMcConf { signature })
}

/// Compact current and field-weakening summary from a motor configuration.
///
/// Field names match the firmware. All settings are available through
/// [`McConfView`], which decodes groups from a retained configuration frame.
/// The reply signature selects a known layout; unknown signatures and incorrect
/// lengths are rejected. Firmware version does not need to be supplied by the
/// caller. Values are configured limits, not measurements or nameplate ratings.
///
/// Supported configuration replies fit the default [`crate::Decoder`].
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct McConf {
    /// Configured positive motor (phase) current limit, in amperes.
    /// The effective configured limit also includes `l_current_max_scale`.
    pub l_current_max: f32,
    /// Absolute motor overcurrent fault threshold, in amperes.
    pub l_abs_current_max: f32,
    /// Maximum configured duty cycle, as a fraction of one.
    pub l_max_duty: f32,
    /// Multiplier applied to `l_current_max`, as a fraction of one.
    pub l_current_max_scale: f32,
    /// Maximum field-weakening current, in amperes.
    pub foc_fw_current_max: f32,
    /// Field-weakening start, as a fraction of `l_max_duty`.
    pub foc_fw_duty_start: f32,
}

impl McConf {
    pub(crate) fn unpack_from(
        unpacker: &mut Unpacker,
        payload_len: usize,
    ) -> Result<Self, DecodeError> {
        let signature = unpacker.unpack_u32()?;
        let (_, config_len, before_field_weakening) = LAYOUTS[layout_for_signature(signature)?];
        // The command ID is also included in the frame payload length.
        if payload_len != config_len + 1 {
            return Err(DecodeError::InvalidFrame);
        }
        let fields = unpacker.consume(config_len - 4)?;
        let mut unpacker = Unpacker::new(fields);

        // Skip pwm_mode, comm_mode, motor_type, sensor_mode (four u8 fields).
        unpacker.consume(4)?;
        let l_current_max = unpacker.unpack_f32_auto()?;
        // l_current_min through l_in_current_map_filter (three f32, two f16).
        unpacker.consume(16)?;
        let l_abs_current_max = unpacker.unpack_f32_auto()?;
        // l_min_erpm through l_min_duty: configuration bytes 32..71.
        unpacker.consume(39)?;
        let l_max_duty = unpacker.unpack_f16(10000.0)?;
        // l_watt_max and l_watt_min (two f32).
        unpacker.consume(8)?;
        let l_current_max_scale = unpacker.unpack_f16(10000.0)?;
        // l_current_min_scale through foc_mtpa_mode (layout dependent).
        unpacker.consume(before_field_weakening)?;
        let foc_fw_current_max = unpacker.unpack_f32_auto()?;
        let foc_fw_duty_start = unpacker.unpack_f16(10000.0)?;

        Ok(Self {
            l_current_max,
            l_abs_current_max,
            l_max_duty,
            l_current_max_scale,
            foc_fw_current_max,
            foc_fw_duty_start,
        })
    }
}
