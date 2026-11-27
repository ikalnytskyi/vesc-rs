//! Typed views of motor configuration settings, grouped like VESC Tool.
//!
//! A configuration frame is read once. [`McConfView`] borrows that frame and
//! decodes only the requested group. No heap allocation or packet copy is used.
//! Shared settings belong to their first VESC Tool category: shared BLDC/DC
//! settings are in [`Bldc`], and the shared FOC/PID position source is in [`Foc`].
//!
//! Layouts follow VESC Tool's `res/config/*/parameters_mcconf.xml` schemas.
//! Firmware field names are retained; array suffixes and BMS dots become Rust
//! arrays and underscores. Enum and bitfield values retain their raw wire codes.

use crate::{CommandReply, DecodeError, packer::Unpacker};

/// Borrowed, validated motor configuration with on-demand group decoding.
///
/// Use [`Self::decode`] with a complete received frame, then call only the group
/// accessors needed by the application. Keeping a group does not require keeping
/// this view. The source frame must remain available while using the view.
///
/// ```no_run
/// use vesc::McConfView;
/// # let frame = [0u8; 512];
/// let (_, config) = McConfView::decode(&frame)?;
/// let general = config.general()?;
/// let foc = config.foc()?;
/// let current_limit = general.l_current_max * general.l_current_max_scale;
/// let field_weakening_current = foc.foc_fw_current_max;
/// # Ok::<(), vesc::DecodeError>(())
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct McConfView<'a> {
    data: &'a [u8],
    layout: usize,
}

impl<'a> McConfView<'a> {
    /// Validates a complete configuration reply, including CRC and signature.
    /// Returns the consumed frame length and a view borrowing the configuration.
    /// Trailing frames are not consumed. The input is the same framed byte
    /// format accepted by [`crate::decode`], not bare CAN fragments.
    pub fn decode(frame: &'a [u8]) -> Result<(usize, Self), DecodeError> {
        let (consumed, reply) = crate::decode(frame)?;
        if !matches!(reply, CommandReply::GetMcConf(_)) {
            return Err(DecodeError::InvalidFrame);
        }
        let header_len = if frame[0] == 2 { 2 } else { 3 };
        let data = &frame[header_len + 1..consumed - 3];
        let signature = Unpacker::new(data).unpack_u32()?;
        let layout = super::layout_for_signature(signature)?;
        Ok((consumed, Self { data, layout }))
    }

    /// Complete serialized configuration, including its signature.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.data
    }

    fn at(&self, offset: usize) -> Result<Unpacker<'a>, DecodeError> {
        Ok(Unpacker::new(
            self.data.get(offset..).ok_or(DecodeError::IncompleteData)?,
        ))
    }

    /// Decodes the General settings from the retained frame.
    pub fn general(&self) -> Result<General, DecodeError> {
        Ok(General {
            motor_type: self.at(6)?.unpack_u8()?,
            l_current_max: self.at(8)?.unpack_f32_auto()?,
            l_current_min: self.at(12)?.unpack_f32_auto()?,
            l_in_current_max: self.at(16)?.unpack_f32_auto()?,
            l_in_current_min: self.at(20)?.unpack_f32_auto()?,
            l_in_current_map_start: self.at(24)?.unpack_f16(10000.0)?,
            l_in_current_map_filter: self.at(26)?.unpack_f16(10000.0)?,
            l_abs_current_max: self.at(28)?.unpack_f32_auto()?,
            l_min_erpm: self.at(32)?.unpack_f32_auto()?,
            l_max_erpm: self.at(36)?.unpack_f32_auto()?,
            l_erpm_start: self
                .at([40, 40, 40, 44][self.layout])?
                .unpack_f16(10000.0)?,
            l_min_vin: self.at(50)?.unpack_f16(10.0)?,
            l_max_vin: self.at(52)?.unpack_f16(10.0)?,
            l_battery_cut_start: self.at(54)?.unpack_f16(10.0)?,
            l_battery_cut_end: self.at(56)?.unpack_f16(10.0)?,
            l_battery_regen_cut_start: self.at(58)?.unpack_f16(10.0)?,
            l_battery_regen_cut_end: self.at(60)?.unpack_f16(10.0)?,
            l_slow_abs_current: self.at(62)?.unpack_u8()? != 0,
            l_temp_fet_start: self.at(63)?.unpack_u8()? as i8,
            l_temp_fet_end: self.at(64)?.unpack_u8()? as i8,
            l_temp_motor_start: self.at(65)?.unpack_u8()? as i8,
            l_temp_motor_end: self.at(66)?.unpack_u8()? as i8,
            l_temp_accel_dec: self.at(67)?.unpack_f16(10000.0)?,
            l_min_duty: self.at(69)?.unpack_f16(10000.0)?,
            l_max_duty: self.at(71)?.unpack_f16(10000.0)?,
            l_watt_max: self.at(73)?.unpack_f32_auto()?,
            l_watt_min: self.at(77)?.unpack_f32_auto()?,
            l_current_max_scale: self.at(81)?.unpack_f16(10000.0)?,
            l_current_min_scale: self.at(83)?.unpack_f16(10000.0)?,
            l_duty_start: self
                .at([85, 85, 85, 89][self.layout])?
                .unpack_f16(10000.0)?,
            cc_min_current: self
                .at([373, 379, 384, 388][self.layout])?
                .unpack_f32_auto()?,
            m_fault_stop_time_ms: self.at([383, 389, 394, 398][self.layout])?.unpack_i32()?,
            m_encoder_counts: self.at([393, 399, 404, 408][self.layout])?.unpack_u32()?,
            m_encoder_sin_amp: self
                .at([397, 403, 408, 412][self.layout])?
                .unpack_f16(1000.0)?,
            m_encoder_cos_amp: self
                .at([399, 405, 410, 414][self.layout])?
                .unpack_f16(1000.0)?,
            m_encoder_sin_offset: self
                .at([401, 407, 412, 416][self.layout])?
                .unpack_f16(1000.0)?,
            m_encoder_cos_offset: self
                .at([403, 409, 414, 418][self.layout])?
                .unpack_f16(1000.0)?,
            m_encoder_sincos_filter_constant: self
                .at([405, 411, 416, 420][self.layout])?
                .unpack_f16(1000.0)?,
            m_encoder_sincos_phase_correction: self
                .at([407, 413, 418, 422][self.layout])?
                .unpack_f16(1000.0)?,
            m_sensor_port_mode: self.at([409, 415, 420, 424][self.layout])?.unpack_u8()?,
            m_invert_direction: self.at([410, 416, 421, 425][self.layout])?.unpack_u8()? != 0,
            m_drv8301_oc_mode: self.at([411, 417, 422, 426][self.layout])?.unpack_u8()?,
            m_drv8301_oc_adj: self.at([412, 418, 423, 427][self.layout])?.unpack_u8()? as i8,
            m_ntc_motor_beta: self
                .at([425, 431, 436, 440][self.layout])?
                .unpack_f32_auto()?,
            m_out_aux_mode: self.at([429, 435, 440, 444][self.layout])?.unpack_u8()?,
            m_motor_temp_sens_type: self.at([430, 436, 441, 445][self.layout])?.unpack_u8()?,
            m_ptc_motor_coeff: self
                .at([431, 437, 442, 446][self.layout])?
                .unpack_f32_auto()?,
            m_ntcx_ptcx_res: self
                .at([435, 441, 446, 450][self.layout])?
                .unpack_f16(0.1)?,
            m_ntcx_ptcx_temp_base: self
                .at([437, 443, 448, 452][self.layout])?
                .unpack_f16(10.0)?,
            m_batt_filter_const: self.at([440, 446, 451, 455][self.layout])?.unpack_u8()? as i8,
            bms_type: self.at([460, 466, 471, 475][self.layout])?.unpack_u8()?,
            bms_limit_mode: self.at([461, 467, 472, 476][self.layout])?.unpack_u8()?,
            bms_t_limit_start: self.at([462, 468, 473, 477][self.layout])?.unpack_u8()? as i8,
            bms_t_limit_end: self.at([463, 469, 474, 478][self.layout])?.unpack_u8()? as i8,
            bms_soc_limit_start: self
                .at([464, 470, 475, 479][self.layout])?
                .unpack_f16(1000.0)?,
            bms_soc_limit_end: self
                .at([466, 472, 477, 481][self.layout])?
                .unpack_f16(1000.0)?,
            bms_vmin_limit_start: self
                .at([468, 474, 479, 483][self.layout])?
                .unpack_f16(1000.0)?,
            bms_vmin_limit_end: self
                .at([470, 476, 481, 485][self.layout])?
                .unpack_f16(1000.0)?,
            bms_vmax_limit_start: self
                .at([472, 478, 483, 487][self.layout])?
                .unpack_f16(1000.0)?,
            bms_vmax_limit_end: self
                .at([474, 480, 485, 489][self.layout])?
                .unpack_f16(1000.0)?,
            bms_fwd_can_mode: self.at([476, 482, 487, 491][self.layout])?.unpack_u8()?,
            l_additional_faults: match [None, None, Some(87), Some(91)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_u8()?),
                None => None,
            },
            l_erpm_abs_overspeed: match [None, None, None, Some(40)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f32_auto()?),
                None => None,
            },
            l_in_current_max_scale: match [None, None, None, Some(85)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(10000.0)?),
                None => None,
            },
            l_in_current_min_scale: match [None, None, None, Some(87)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(10000.0)?),
                None => None,
            },
        })
    }

    /// Decodes the FOC settings from the retained frame.
    pub fn foc(&self) -> Result<Foc, DecodeError> {
        Ok(Foc {
            foc_current_kp: self
                .at([123, 123, 124, 128][self.layout])?
                .unpack_f32_auto()?,
            foc_current_ki: self
                .at([127, 127, 128, 132][self.layout])?
                .unpack_f32_auto()?,
            foc_f_zv: self
                .at([131, 131, 132, 136][self.layout])?
                .unpack_f32_auto()?,
            foc_dt_us: self
                .at([135, 135, 136, 140][self.layout])?
                .unpack_f32_auto()?,
            foc_encoder_inverted: self.at([139, 139, 140, 144][self.layout])?.unpack_u8()? != 0,
            foc_encoder_offset: self
                .at([140, 140, 141, 145][self.layout])?
                .unpack_f32_auto()?,
            foc_encoder_ratio: self
                .at([144, 144, 145, 149][self.layout])?
                .unpack_f32_auto()?,
            foc_sensor_mode: self.at([148, 148, 149, 153][self.layout])?.unpack_u8()?,
            foc_pll_kp: self
                .at([149, 149, 150, 154][self.layout])?
                .unpack_f32_auto()?,
            foc_pll_ki: self
                .at([153, 153, 154, 158][self.layout])?
                .unpack_f32_auto()?,
            foc_motor_l: self
                .at([157, 157, 158, 162][self.layout])?
                .unpack_f32_auto()?,
            foc_motor_ld_lq_diff: self
                .at([161, 161, 162, 166][self.layout])?
                .unpack_f32_auto()?,
            foc_motor_r: self
                .at([165, 165, 166, 170][self.layout])?
                .unpack_f32_auto()?,
            foc_motor_flux_linkage: self
                .at([169, 169, 170, 174][self.layout])?
                .unpack_f32_auto()?,
            foc_observer_gain: self
                .at([173, 173, 174, 178][self.layout])?
                .unpack_f32_auto()?,
            foc_observer_gain_slow: self
                .at([177, 177, 178, 182][self.layout])?
                .unpack_f32_auto()?,
            foc_observer_offset: self
                .at([181, 181, 182, 186][self.layout])?
                .unpack_f16(1000.0)?,
            foc_duty_dowmramp_kp: self
                .at([183, 183, 184, 188][self.layout])?
                .unpack_f32_auto()?,
            foc_duty_dowmramp_ki: self
                .at([187, 187, 188, 192][self.layout])?
                .unpack_f32_auto()?,
            foc_start_curr_dec: self
                .at([191, 191, 192, 196][self.layout])?
                .unpack_f16(10000.0)?,
            foc_start_curr_dec_rpm: self
                .at([193, 193, 194, 198][self.layout])?
                .unpack_f32_auto()?,
            foc_openloop_rpm: self
                .at([197, 197, 198, 202][self.layout])?
                .unpack_f32_auto()?,
            foc_openloop_rpm_low: self
                .at([201, 201, 202, 206][self.layout])?
                .unpack_f16(1000.0)?,
            foc_d_gain_scale_start: match [Some(203), Some(203), None, None][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(1000.0)?),
                None => None,
            },
            foc_d_gain_scale_max_mod: match [Some(205), Some(205), None, None][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(1000.0)?),
                None => None,
            },
            foc_sl_openloop_hyst: self
                .at([207, 207, 204, 208][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_openloop_time_lock: self
                .at([209, 209, 206, 210][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_openloop_time_ramp: self
                .at([211, 211, 208, 212][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_openloop_time: self
                .at([213, 213, 210, 214][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_openloop_boost_q: self
                .at([215, 215, 212, 216][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_openloop_max_q: self
                .at([217, 217, 214, 218][self.layout])?
                .unpack_f16(100.0)?,
            foc_hall_table: [
                self.at([219, 219, 216, 220][self.layout])?.unpack_u8()? as i8,
                self.at([220, 220, 217, 221][self.layout])?.unpack_u8()? as i8,
                self.at([221, 221, 218, 222][self.layout])?.unpack_u8()? as i8,
                self.at([222, 222, 219, 223][self.layout])?.unpack_u8()? as i8,
                self.at([223, 223, 220, 224][self.layout])?.unpack_u8()? as i8,
                self.at([224, 224, 221, 225][self.layout])?.unpack_u8()? as i8,
                self.at([225, 225, 222, 226][self.layout])?.unpack_u8()? as i8,
                self.at([226, 226, 223, 227][self.layout])?.unpack_u8()? as i8,
            ],
            foc_hall_interp_erpm: self
                .at([227, 227, 224, 228][self.layout])?
                .unpack_f32_auto()?,
            foc_sl_erpm_start: self
                .at([231, 231, 228, 232][self.layout])?
                .unpack_f32_auto()?,
            foc_sl_erpm: self
                .at([235, 235, 232, 236][self.layout])?
                .unpack_f32_auto()?,
            foc_control_sample_mode: self.at([239, 239, 236, 240][self.layout])?.unpack_u8()?,
            foc_current_sample_mode: self.at([240, 240, 237, 241][self.layout])?.unpack_u8()?,
            foc_sat_comp_mode: self.at([241, 241, 238, 242][self.layout])?.unpack_u8()?,
            foc_sat_comp: self
                .at([242, 242, 239, 243][self.layout])?
                .unpack_f16(1000.0)?,
            foc_temp_comp: self.at([244, 244, 241, 245][self.layout])?.unpack_u8()? != 0,
            foc_temp_comp_base_temp: self
                .at([245, 245, 242, 246][self.layout])?
                .unpack_f16(100.0)?,
            foc_current_filter_const: self
                .at([247, 247, 244, 248][self.layout])?
                .unpack_f16(10000.0)?,
            foc_cc_decoupling: self.at([249, 249, 246, 250][self.layout])?.unpack_u8()?,
            foc_observer_type: self.at([250, 250, 247, 251][self.layout])?.unpack_u8()?,
            foc_hfi_voltage_start: self
                .at([251, 255, 252, 256][self.layout])?
                .unpack_f16(10.0)?,
            foc_hfi_voltage_run: self
                .at([253, 257, 254, 258][self.layout])?
                .unpack_f16(10.0)?,
            foc_hfi_voltage_max: self
                .at([255, 259, 256, 260][self.layout])?
                .unpack_f16(10.0)?,
            foc_hfi_gain: self
                .at([257, 261, 258, 262][self.layout])?
                .unpack_f16(1000.0)?,
            foc_hfi_max_err: self
                .at([259, 263, 260, 264][self.layout])?
                .unpack_f16(1000.0)?,
            foc_hfi_hyst: self
                .at([261, 265, 262, 266][self.layout])?
                .unpack_f16(100.0)?,
            foc_sl_erpm_hfi: self
                .at([263, 267, 264, 268][self.layout])?
                .unpack_f32_auto()?,
            foc_hfi_start_samples: self.at([267, 271, 272, 276][self.layout])?.unpack_u16()?,
            foc_hfi_obs_ovr_sec: self
                .at([269, 273, 274, 278][self.layout])?
                .unpack_f32_auto()?,
            foc_hfi_samples: self.at([273, 277, 278, 282][self.layout])?.unpack_u8()?,
            foc_offsets_cal_on_boot: match [Some(274), None, None, None][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_u8()? != 0),
                None => None,
            },
            foc_offsets_current: [
                self.at([275, 279, 280, 284][self.layout])?
                    .unpack_f32_auto()?,
                self.at([279, 283, 284, 288][self.layout])?
                    .unpack_f32_auto()?,
                self.at([283, 287, 288, 292][self.layout])?
                    .unpack_f32_auto()?,
            ],
            foc_offsets_voltage: [
                self.at([287, 291, 292, 296][self.layout])?
                    .unpack_f16(10000.0)?,
                self.at([289, 293, 294, 298][self.layout])?
                    .unpack_f16(10000.0)?,
                self.at([291, 295, 296, 300][self.layout])?
                    .unpack_f16(10000.0)?,
            ],
            foc_offsets_voltage_undriven: [
                self.at([293, 297, 298, 302][self.layout])?
                    .unpack_f16(10000.0)?,
                self.at([295, 299, 300, 304][self.layout])?
                    .unpack_f16(10000.0)?,
                self.at([297, 301, 302, 306][self.layout])?
                    .unpack_f16(10000.0)?,
            ],
            foc_phase_filter_enable: self.at([299, 303, 304, 308][self.layout])?.unpack_u8()? != 0,
            foc_phase_filter_disable_fault: self
                .at([300, 304, 305, 309][self.layout])?
                .unpack_u8()?
                != 0,
            foc_phase_filter_max_erpm: self
                .at([301, 305, 306, 310][self.layout])?
                .unpack_f32_auto()?,
            foc_mtpa_mode: self.at([305, 309, 310, 314][self.layout])?.unpack_u8()?,
            foc_fw_current_max: self
                .at([306, 310, 311, 315][self.layout])?
                .unpack_f32_auto()?,
            foc_fw_duty_start: self
                .at([310, 314, 315, 319][self.layout])?
                .unpack_f16(10000.0)?,
            foc_fw_ramp_time: self
                .at([312, 316, 317, 321][self.layout])?
                .unpack_f16(1000.0)?,
            foc_fw_q_current_factor: self
                .at([314, 318, 319, 323][self.layout])?
                .unpack_f16(10000.0)?,
            foc_speed_soure: self.at([316, 320, 323, 327][self.layout])?.unpack_u8()?,
            foc_short_ls_on_zero_duty: self.at([317, 321, 324, 328][self.layout])?.unpack_u8()?
                != 0,
            m_hall_extra_samples: self.at([439, 445, 450, 454][self.layout])?.unpack_u8()? as i8,
            foc_hfi_amb_mode: match [None, Some(251), Some(248), Some(252)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_u8()?),
                None => None,
            },
            foc_hfi_amb_current: match [None, Some(252), Some(249), Some(253)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(10.0)?),
                None => None,
            },
            foc_hfi_amb_tres: match [None, Some(254), Some(251), Some(255)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_u8()? as i8),
                None => None,
            },
            foc_offsets_cal_mode: match [None, Some(278), Some(279), Some(283)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_u8()?),
                None => None,
            },
            foc_overmod_factor: match [None, Some(322), Some(325), Some(329)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(10000.0)?),
                None => None,
            },
            foc_hfi_reset_erpm: match [None, None, Some(268), Some(272)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f32_auto()?),
                None => None,
            },
            foc_fw_backoff: match [None, None, Some(321), Some(325)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(1000.0)?),
                None => None,
            },
            foc_mag_vd_max: match [None, None, Some(327), Some(331)][self.layout] {
                Some(offset) => Some(self.at(offset)?.unpack_f16(10000.0)?),
                None => None,
            },
        })
    }

    /// Decodes the BLDC settings from the retained frame.
    pub fn bldc(&self) -> Result<Bldc, DecodeError> {
        Ok(Bldc {
            pwm_mode: self.at(4)?.unpack_u8()?,
            comm_mode: self.at(5)?.unpack_u8()?,
            sensor_mode: self.at(7)?.unpack_u8()?,
            l_max_erpm_fbrake: match self.layout {
                3 => self.at([42, 42, 42, 46][self.layout])?.unpack_f16(0.1)?,
                _ => self.at([42, 42, 42, 46][self.layout])?.unpack_f32_auto()?,
            },
            l_max_erpm_fbrake_cc: match self.layout {
                3 => self.at([46, 46, 46, 48][self.layout])?.unpack_f16(0.1)?,
                _ => self.at([46, 46, 46, 48][self.layout])?.unpack_f32_auto()?,
            },
            sl_min_erpm: self.at([87, 87, 88, 92][self.layout])?.unpack_f32_auto()?,
            sl_min_erpm_cycle_int_limit: self
                .at([91, 91, 92, 96][self.layout])?
                .unpack_f32_auto()?,
            sl_max_fullbreak_current_dir_change: self
                .at([95, 95, 96, 100][self.layout])?
                .unpack_f32_auto()?,
            sl_cycle_int_limit: self.at([99, 99, 100, 104][self.layout])?.unpack_f16(10.0)?,
            sl_phase_advance_at_br: self
                .at([101, 101, 102, 106][self.layout])?
                .unpack_f16(10000.0)?,
            sl_cycle_int_rpm_br: self
                .at([103, 103, 104, 108][self.layout])?
                .unpack_f32_auto()?,
            sl_bemf_coupling_k: self
                .at([107, 107, 108, 112][self.layout])?
                .unpack_f32_auto()?,
            hall_table: [
                self.at([111, 111, 112, 116][self.layout])?.unpack_u8()?,
                self.at([112, 112, 113, 117][self.layout])?.unpack_u8()?,
                self.at([113, 113, 114, 118][self.layout])?.unpack_u8()?,
                self.at([114, 114, 115, 119][self.layout])?.unpack_u8()?,
                self.at([115, 115, 116, 120][self.layout])?.unpack_u8()?,
                self.at([116, 116, 117, 121][self.layout])?.unpack_u8()?,
                self.at([117, 117, 118, 122][self.layout])?.unpack_u8()?,
                self.at([118, 118, 119, 123][self.layout])?.unpack_u8()?,
            ],
            hall_sl_erpm: self
                .at([119, 119, 120, 124][self.layout])?
                .unpack_f32_auto()?,
            cc_startup_boost_duty: self
                .at([371, 377, 382, 386][self.layout])?
                .unpack_f16(10000.0)?,
            cc_gain: self
                .at([377, 383, 388, 392][self.layout])?
                .unpack_f32_auto()?,
            cc_ramp_step_max: self
                .at([381, 387, 392, 396][self.layout])?
                .unpack_f16(10000.0)?,
            m_duty_ramp_step: self
                .at([387, 393, 398, 402][self.layout])?
                .unpack_f16(10000.0)?,
            m_current_backoff_gain: self
                .at([389, 395, 400, 404][self.layout])?
                .unpack_f32_auto()?,
            m_bldc_f_sw_min: self
                .at([413, 419, 424, 428][self.layout])?
                .unpack_f32_auto()?,
            m_bldc_f_sw_max: self
                .at([417, 423, 428, 432][self.layout])?
                .unpack_f32_auto()?,
        })
    }

    /// Decodes the DC settings from the retained frame.
    pub fn dc(&self) -> Result<Dc, DecodeError> {
        Ok(Dc {
            m_dc_f_sw: self
                .at([421, 427, 432, 436][self.layout])?
                .unpack_f32_auto()?,
        })
    }

    /// Decodes the PID Controllers settings from the retained frame.
    pub fn pid_controllers(&self) -> Result<PidControllers, DecodeError> {
        Ok(PidControllers {
            sp_pid_loop_rate: self.at([318, 324, 329, 333][self.layout])?.unpack_u8()?,
            s_pid_kp: self
                .at([319, 325, 330, 334][self.layout])?
                .unpack_f32_auto()?,
            s_pid_ki: self
                .at([323, 329, 334, 338][self.layout])?
                .unpack_f32_auto()?,
            s_pid_kd: self
                .at([327, 333, 338, 342][self.layout])?
                .unpack_f32_auto()?,
            s_pid_kd_filter: self
                .at([331, 337, 342, 346][self.layout])?
                .unpack_f16(10000.0)?,
            s_pid_min_erpm: self
                .at([333, 339, 344, 348][self.layout])?
                .unpack_f32_auto()?,
            s_pid_allow_braking: self.at([337, 343, 348, 352][self.layout])?.unpack_u8()? != 0,
            s_pid_ramp_erpms_s: self
                .at([338, 344, 349, 353][self.layout])?
                .unpack_f32_auto()?,
            s_pid_speed_source: self.at([342, 348, 353, 357][self.layout])?.unpack_u8()?,
            p_pid_kp: self
                .at([343, 349, 354, 358][self.layout])?
                .unpack_f32_auto()?,
            p_pid_ki: self
                .at([347, 353, 358, 362][self.layout])?
                .unpack_f32_auto()?,
            p_pid_kd: self
                .at([351, 357, 362, 366][self.layout])?
                .unpack_f32_auto()?,
            p_pid_kd_proc: self
                .at([355, 361, 366, 370][self.layout])?
                .unpack_f32_auto()?,
            p_pid_kd_filter: self
                .at([359, 365, 370, 374][self.layout])?
                .unpack_f16(10000.0)?,
            p_pid_ang_div: self
                .at([361, 367, 372, 376][self.layout])?
                .unpack_f32_auto()?,
            p_pid_gain_dec_angle: self
                .at([365, 371, 376, 380][self.layout])?
                .unpack_f16(10.0)?,
            p_pid_offset: self
                .at([367, 373, 378, 382][self.layout])?
                .unpack_f32_auto()?,
        })
    }

    /// Decodes the Additional Info settings from the retained frame.
    pub fn additional_info(&self) -> Result<AdditionalInfo, DecodeError> {
        Ok(AdditionalInfo {
            si_motor_poles: self.at([441, 447, 452, 456][self.layout])?.unpack_u8()? as i8,
            si_gear_ratio: self
                .at([442, 448, 453, 457][self.layout])?
                .unpack_f32_auto()?,
            si_wheel_diameter: self
                .at([446, 452, 457, 461][self.layout])?
                .unpack_f32_auto()?,
            si_battery_type: self.at([450, 456, 461, 465][self.layout])?.unpack_u8()?,
            si_battery_cells: self.at([451, 457, 462, 466][self.layout])?.unpack_u8()? as i8,
            si_battery_ah: self
                .at([452, 458, 463, 467][self.layout])?
                .unpack_f32_auto()?,
            si_motor_nl_current: self
                .at([456, 462, 467, 471][self.layout])?
                .unpack_f32_auto()?,
        })
    }
}

/// General motor settings.
/// Fields absent from the reply layout are `None`, not zero.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct General {
    /// Motor Type. Raw enum code from the matching VESC Tool schema.
    pub motor_type: u8,
    /// Motor Current Max. Unit: `A`.
    pub l_current_max: f32,
    /// Motor Current Max Brake. Unit: `A`.
    pub l_current_min: f32,
    /// Battery Current Max. Unit: `A`.
    pub l_in_current_max: f32,
    /// Battery Current Max Regen. Unit: `A`.
    pub l_in_current_min: f32,
    /// Input Current Limit Map Start. Fraction of one.
    pub l_in_current_map_start: f32,
    /// Input Current Map Filter.
    pub l_in_current_map_filter: f32,
    /// Absolute Maximum Current. Unit: `A`.
    pub l_abs_current_max: f32,
    /// Max ERPM Reverse.
    pub l_min_erpm: f32,
    /// Max ERPM.
    pub l_max_erpm: f32,
    /// ERPM Limit Start. Fraction of one.
    pub l_erpm_start: f32,
    /// Minimum Input Voltage. Unit: `V`.
    pub l_min_vin: f32,
    /// Maximum Input Voltage. Unit: `V`.
    pub l_max_vin: f32,
    /// Battery Voltage Cutoff Start. Unit: `V`.
    pub l_battery_cut_start: f32,
    /// Battery Voltage Cutoff End. Unit: `V`.
    pub l_battery_cut_end: f32,
    /// Battery Voltage Regen Cutoff Start. Unit: `V`.
    pub l_battery_regen_cut_start: f32,
    /// Battery Voltage Regen Cutoff End. Unit: `V`.
    pub l_battery_regen_cut_end: f32,
    /// Slow ABS Current Limit.
    pub l_slow_abs_current: bool,
    /// MOSFET Temp Cutoff Start. Unit: `°C`.
    pub l_temp_fet_start: i8,
    /// MOSFET Temp Cutoff End. Unit: `°C`.
    pub l_temp_fet_end: i8,
    /// Motor Temp Cutoff Start. Unit: `°C`.
    pub l_temp_motor_start: i8,
    /// Motor Temp Cutoff End. Unit: `°C`.
    pub l_temp_motor_end: i8,
    /// Acceleration Temperature Decrease. Fraction of one.
    pub l_temp_accel_dec: f32,
    /// Minimum Duty Cycle. Fraction of one.
    pub l_min_duty: f32,
    /// Maximum Duty Cycle. Fraction of one.
    pub l_max_duty: f32,
    /// Maximum Wattage. Unit: `W`.
    pub l_watt_max: f32,
    /// Maximum Braking Wattage. Unit: `W`.
    pub l_watt_min: f32,
    /// Motor Current Max Scale. Fraction of one.
    pub l_current_max_scale: f32,
    /// Motor Current Min Scale. Fraction of one.
    pub l_current_min_scale: f32,
    /// Duty Cycle Current Limit Start. Fraction of one.
    pub l_duty_start: f32,
    /// Minimum Current. Unit: `A`.
    pub cc_min_current: f32,
    /// Fault Stop Time. Unit: `ms`.
    pub m_fault_stop_time_ms: i32,
    /// Encoder counts.
    pub m_encoder_counts: u32,
    /// Sine Amplitude. Unit: `V`.
    pub m_encoder_sin_amp: f32,
    /// Cosine Amplitude. Unit: `V`.
    pub m_encoder_cos_amp: f32,
    /// Sine Offset. Unit: `V`.
    pub m_encoder_sin_offset: f32,
    /// Cosine Offset. Unit: `V`.
    pub m_encoder_cos_offset: f32,
    /// Sin/Cos Filter Constant.
    pub m_encoder_sincos_filter_constant: f32,
    /// Sin/Cos Phase Correction. Unit: `°`.
    pub m_encoder_sincos_phase_correction: f32,
    /// Sensor Port Mode. Raw enum code from the matching VESC Tool schema.
    pub m_sensor_port_mode: u8,
    /// Invert Motor Direction.
    pub m_invert_direction: bool,
    /// DRV8301 OC Mode. Raw enum code from the matching VESC Tool schema.
    pub m_drv8301_oc_mode: u8,
    /// DRV8301 OC Adjustment.
    pub m_drv8301_oc_adj: i8,
    /// Beta Value for Motor Thermistor. Unit: `K`.
    pub m_ntc_motor_beta: f32,
    /// Auxiliary Output Mode. Raw enum code from the matching VESC Tool schema.
    pub m_out_aux_mode: u8,
    /// Motor Temperature Sensor Type. Raw enum code from the matching VESC Tool schema.
    pub m_motor_temp_sens_type: u8,
    /// Coefficient for PTC Motor Thermistor. Unit: `%/K`.
    pub m_ptc_motor_coeff: f32,
    /// Custom NTC/PTC Resistance. VESC Tool displays the value × 0.001 in `kΩ`.
    pub m_ntcx_ptcx_res: f32,
    /// Custom NTC/PTC Base Temperature. Unit: `°C`.
    pub m_ntcx_ptcx_temp_base: f32,
    /// Battery Filter Constant.
    pub m_batt_filter_const: i8,
    /// BMS Type. Raw enum code from the matching VESC Tool schema.
    pub bms_type: u8,
    /// BMS Limit Mode.
    pub bms_limit_mode: u8,
    /// Temperature Limit Start. Unit: `°C`.
    pub bms_t_limit_start: i8,
    /// Temperature Limit End. Unit: `°C`.
    pub bms_t_limit_end: i8,
    /// SOC Limit Start. Fraction of one.
    pub bms_soc_limit_start: f32,
    /// SOC Limit End. Fraction of one.
    pub bms_soc_limit_end: f32,
    /// VCell Min Limit Start. Unit: `V`.
    pub bms_vmin_limit_start: f32,
    /// VCell Min Limit End. Unit: `V`.
    pub bms_vmin_limit_end: f32,
    /// VCell Max Limit Start. Unit: `V`.
    pub bms_vmax_limit_start: f32,
    /// VCell Max Limit End. Unit: `V`.
    pub bms_vmax_limit_end: f32,
    /// Forward CAN to Local. Raw enum code from the matching VESC Tool schema.
    pub bms_fwd_can_mode: u8,
    /// Additional Faults.
    pub l_additional_faults: Option<u8>,
    /// ERPM ABS Overspeed.
    pub l_erpm_abs_overspeed: Option<f32>,
    /// Battery Current Max Scale. Fraction of one.
    pub l_in_current_max_scale: Option<f32>,
    /// Battery Current Min Scale. Fraction of one.
    pub l_in_current_min_scale: Option<f32>,
}

/// FOC motor settings.
/// Fields absent from the reply layout are `None`, not zero.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Foc {
    /// Current KP.
    pub foc_current_kp: f32,
    /// Current KI.
    pub foc_current_ki: f32,
    /// Zero Vector Frequency. VESC Tool displays the value × 0.001 in `kHz`.
    pub foc_f_zv: f32,
    /// Dead Time Compensation. Unit: `µS`.
    pub foc_dt_us: f32,
    /// Encoder Inverted.
    pub foc_encoder_inverted: bool,
    /// Encoder Offset.
    pub foc_encoder_offset: f32,
    /// Encoder Ratio.
    pub foc_encoder_ratio: f32,
    /// Sensor Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_sensor_mode: u8,
    /// Speed Tracker Kp.
    pub foc_pll_kp: f32,
    /// Speed Tracker Ki.
    pub foc_pll_ki: f32,
    /// Motor Inductance (L). VESC Tool displays the value × 1000000.0 in `µH`.
    pub foc_motor_l: f32,
    /// Motor Inductance Difference (Lq - Ld). VESC Tool displays the value × 1000000.0 in `µH`.
    pub foc_motor_ld_lq_diff: f32,
    /// Motor Resistance (R). VESC Tool displays the value × 1000.0 in `mΩ`.
    pub foc_motor_r: f32,
    /// Motor Flux Linkage (λ). VESC Tool displays the value × 1000.0 in `mWb`.
    pub foc_motor_flux_linkage: f32,
    /// Observer Gain (x1M).
    pub foc_observer_gain: f32,
    /// Observer Gain At Minimum Duty. Fraction of one.
    pub foc_observer_gain_slow: f32,
    /// Observer Offset.
    pub foc_observer_offset: f32,
    /// Duty Downramp Kp.
    pub foc_duty_dowmramp_kp: f32,
    /// Duty Downramp Ki.
    pub foc_duty_dowmramp_ki: f32,
    /// Start Current Decrease. Fraction of one.
    pub foc_start_curr_dec: f32,
    /// Start Current Decrease ERPM.
    pub foc_start_curr_dec_rpm: f32,
    /// Openloop ERPM.
    pub foc_openloop_rpm: f32,
    /// Openloop ERPM at Min Current. Fraction of one.
    pub foc_openloop_rpm_low: f32,
    /// D Axis Gain Scaling Start. Fraction of one.
    pub foc_d_gain_scale_start: Option<f32>,
    /// D Axis Gain Scaling at Max Mod. Fraction of one.
    pub foc_d_gain_scale_max_mod: Option<f32>,
    /// Openloop Hysteresis. Unit: `S`.
    pub foc_sl_openloop_hyst: f32,
    /// Openloop Lock Time. Unit: `S`.
    pub foc_sl_openloop_time_lock: f32,
    /// Openloop Ramp Time. Unit: `S`.
    pub foc_sl_openloop_time_ramp: f32,
    /// Openloop Time. Unit: `S`.
    pub foc_sl_openloop_time: f32,
    /// Openloop Current Boost. Unit: `A`.
    pub foc_sl_openloop_boost_q: f32,
    /// Openloop Current Max. Unit: `A`.
    pub foc_sl_openloop_max_q: f32,
    /// Hall Table.
    pub foc_hall_table: [i8; 8],
    /// Hall Interpolation ERPM.
    pub foc_hall_interp_erpm: f32,
    /// Sensored ERPM Start.
    pub foc_sl_erpm_start: f32,
    /// Sensorless ERPM.
    pub foc_sl_erpm: f32,
    /// Control Sample Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_control_sample_mode: u8,
    /// Current Sample Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_current_sample_mode: u8,
    /// Saturation Compensation Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_sat_comp_mode: u8,
    /// Saturation Compensation Factor. Fraction of one.
    pub foc_sat_comp: f32,
    /// Temp Comp.
    pub foc_temp_comp: bool,
    /// Temp Comp Base Temp. Unit: `°C`.
    pub foc_temp_comp_base_temp: f32,
    /// Current Filter Constant.
    pub foc_current_filter_const: f32,
    /// Current Controller Decoupling. Raw enum code from the matching VESC Tool schema.
    pub foc_cc_decoupling: u8,
    /// Observer Type. Raw enum code from the matching VESC Tool schema.
    pub foc_observer_type: u8,
    /// HFI Start Voltage. Unit: `V`.
    pub foc_hfi_voltage_start: f32,
    /// HFI Run Voltage. Unit: `V`.
    pub foc_hfi_voltage_run: f32,
    /// HFI Max Voltage. Unit: `V`.
    pub foc_hfi_voltage_max: f32,
    /// HFI Gain.
    pub foc_hfi_gain: f32,
    /// HFI Max Error.
    pub foc_hfi_max_err: f32,
    /// HFI Current Hysteresis. Unit: `A`.
    pub foc_hfi_hyst: f32,
    /// Sensorless ERPM HFI.
    pub foc_sl_erpm_hfi: f32,
    /// HFI Start Samples.
    pub foc_hfi_start_samples: u16,
    /// HFI Observer Override Time. VESC Tool displays the value × 1000.0 in `ms`.
    pub foc_hfi_obs_ovr_sec: f32,
    /// HFI Samples. Raw enum code from the matching VESC Tool schema.
    pub foc_hfi_samples: u8,
    /// Run calibration at boot.
    pub foc_offsets_cal_on_boot: Option<bool>,
    /// Current Offset.
    pub foc_offsets_current: [f32; 3],
    /// Voltage Offset. Unit: `V`.
    pub foc_offsets_voltage: [f32; 3],
    /// Voltage Offset Undriven. Unit: `V`.
    pub foc_offsets_voltage_undriven: [f32; 3],
    /// Enable Phase Filters.
    pub foc_phase_filter_enable: bool,
    /// Disable Phase Filter Fault Code.
    pub foc_phase_filter_disable_fault: bool,
    /// Maximum ERPM for phase filters. Unit: `ERPM`.
    pub foc_phase_filter_max_erpm: f32,
    /// MTPA Algorithm Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_mtpa_mode: u8,
    /// Field Weakening Current Max. Unit: `A`.
    pub foc_fw_current_max: f32,
    /// Field Weakening Duty Start. Fraction of one.
    pub foc_fw_duty_start: f32,
    /// Field Weakening Ramp Time. VESC Tool displays the value × 1000.0 in `ms`.
    pub foc_fw_ramp_time: f32,
    /// Q Axis Current Factor. Fraction of one.
    pub foc_fw_q_current_factor: f32,
    /// Speed Tracker Position Source. Raw enum code from the matching VESC Tool schema.
    pub foc_speed_soure: u8,
    /// Short Low-Side FETs on Zero Duty.
    pub foc_short_ls_on_zero_duty: bool,
    /// Hall Sensor Extra Samples.
    pub m_hall_extra_samples: i8,
    /// HFI Ambiguity Resolve Mode. Raw enum code from the matching VESC Tool schema.
    pub foc_hfi_amb_mode: Option<u8>,
    /// HFI Ambiguity Resolve Current. Unit: `A`.
    pub foc_hfi_amb_current: Option<f32>,
    /// HFI Ambiguity Resolve Threshold. Unit: `%`.
    pub foc_hfi_amb_tres: Option<i8>,
    /// Offset Calibration Mode.
    pub foc_offsets_cal_mode: Option<u8>,
    /// Overmodulation Factor.
    pub foc_overmod_factor: Option<f32>,
    /// HFI Reset ERPM.
    pub foc_hfi_reset_erpm: Option<f32>,
    /// Backoff Gain.
    pub foc_fw_backoff: Option<f32>,
    /// Maximum VD Magnitude.
    pub foc_mag_vd_max: Option<f32>,
}

/// BLDC motor settings.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bldc {
    /// PWM Mode. Raw enum code from the matching VESC Tool schema.
    pub pwm_mode: u8,
    /// Commutation Mode. Raw enum code from the matching VESC Tool schema.
    pub comm_mode: u8,
    /// Sensor Mode. Raw enum code from the matching VESC Tool schema.
    pub sensor_mode: u8,
    /// Max ERPM Full Brake.
    pub l_max_erpm_fbrake: f32,
    /// Max ERPM Full Brake Current Control.
    pub l_max_erpm_fbrake_cc: f32,
    /// Minimum ERPM.
    pub sl_min_erpm: f32,
    /// Minimum ERPM Integrator.
    pub sl_min_erpm_cycle_int_limit: f32,
    /// Max Brake Current at Direction Change. Unit: `A`.
    pub sl_max_fullbreak_current_dir_change: f32,
    /// Cycle Integrator Limit.
    pub sl_cycle_int_limit: f32,
    /// Phase Advance at BR ERPM.
    pub sl_phase_advance_at_br: f32,
    /// BR ERPM.
    pub sl_cycle_int_rpm_br: f32,
    /// BEMF Coupling.
    pub sl_bemf_coupling_k: f32,
    /// Hall Table.
    pub hall_table: [u8; 8],
    /// Sensorless ERPM Hybrid.
    pub hall_sl_erpm: f32,
    /// Startup boost.
    pub cc_startup_boost_duty: f32,
    /// Current Controller Gain.
    pub cc_gain: f32,
    /// Current Control Ramp Step Max.
    pub cc_ramp_step_max: f32,
    /// Duty Ramp Step Max.
    pub m_duty_ramp_step: f32,
    /// Current Backoff Gain.
    pub m_current_backoff_gain: f32,
    /// Minimum Switching Frequency. VESC Tool displays the value × 0.001 in `kHz`.
    pub m_bldc_f_sw_min: f32,
    /// Maximum Switching Frequency. VESC Tool displays the value × 0.001 in `kHz`.
    pub m_bldc_f_sw_max: f32,
}

/// DC motor settings.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Dc {
    /// Switching Frequency. VESC Tool displays the value × 0.001 in `kHz`.
    pub m_dc_f_sw: f32,
}

/// PID Controllers motor settings.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PidControllers {
    /// PID Loop Rate. Raw enum code from the matching VESC Tool schema.
    pub sp_pid_loop_rate: u8,
    /// Speed PID Kp.
    pub s_pid_kp: f32,
    /// Speed PID Ki.
    pub s_pid_ki: f32,
    /// Speed PID Kd.
    pub s_pid_kd: f32,
    /// Speed PID Kd Filter.
    pub s_pid_kd_filter: f32,
    /// Minimum ERPM.
    pub s_pid_min_erpm: f32,
    /// Allow Braking.
    pub s_pid_allow_braking: bool,
    /// Ramp eRPMs per second.
    pub s_pid_ramp_erpms_s: f32,
    /// Speed Source. Raw enum code from the matching VESC Tool schema.
    pub s_pid_speed_source: u8,
    /// Position PID Kp.
    pub p_pid_kp: f32,
    /// Position PID Ki.
    pub p_pid_ki: f32,
    /// Position PID Kd.
    pub p_pid_kd: f32,
    /// Position PID Kd Process.
    pub p_pid_kd_proc: f32,
    /// Position PID Kd Filter.
    pub p_pid_kd_filter: f32,
    /// Position Angle Division.
    pub p_pid_ang_div: f32,
    /// Gain Decrease Angle. Unit: `°`.
    pub p_pid_gain_dec_angle: f32,
    /// Position PID Offset Angle. Unit: `°`.
    pub p_pid_offset: f32,
}

/// Additional Info motor settings.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdditionalInfo {
    /// Motor Poles.
    pub si_motor_poles: i8,
    /// Gear Ratio.
    pub si_gear_ratio: f32,
    /// Wheel Diameter. VESC Tool displays the value × 1000.0 in `mm`.
    pub si_wheel_diameter: f32,
    /// Battery Type. Raw enum code from the matching VESC Tool schema.
    pub si_battery_type: u8,
    /// Battery Cells Series.
    pub si_battery_cells: i8,
    /// Battery Capacity. Unit: `Ah`.
    pub si_battery_ah: f32,
    /// Motor No Load Current. Unit: `A`.
    pub si_motor_nl_current: f32,
}
