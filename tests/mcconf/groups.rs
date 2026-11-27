// Included by mcconf.rs to reuse its frame helper.
use vesc::McConfView;

#[test]
fn all_groups_6_05() {
    let frame = super::frame(include_bytes!("../fixtures/mcconf-6.05.bin"));
    let (_, view) = McConfView::decode(&frame).unwrap();
    let settings = view.general().unwrap();
    assert_eq!(settings.motor_type, 2);
    assert_eq!(settings.l_current_max, 178.75_f32);
    assert_eq!(settings.l_current_min, 0.005859375_f32);
    assert_eq!(settings.l_in_current_max, 0.0068359375_f32);
    assert_eq!(settings.l_in_current_min, 0.0078125_f32);
    assert_eq!(settings.l_in_current_map_start, 0.0087_f32);
    assert_eq!(settings.l_in_current_map_filter, 0.0097_f32);
    assert_eq!(settings.l_abs_current_max, 268.125_f32);
    assert_eq!(settings.l_min_erpm, 0.01171875_f32);
    assert_eq!(settings.l_max_erpm, 0.0126953125_f32);
    assert_eq!(settings.l_erpm_start, 0.0136_f32);
    assert_eq!(settings.l_min_vin, 0_f32);
    assert_eq!(settings.l_max_vin, 0_f32);
    assert_eq!(settings.l_battery_cut_start, 0_f32);
    assert_eq!(settings.l_battery_cut_end, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_start, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_end, 0_f32);
    assert!(!settings.l_slow_abs_current);
    assert_eq!(settings.l_temp_fet_start, -24);
    assert_eq!(settings.l_temp_fet_end, -25);
    assert_eq!(settings.l_temp_motor_start, -26);
    assert_eq!(settings.l_temp_motor_end, -27);
    assert_eq!(settings.l_temp_accel_dec, 0.0273_f32);
    assert_eq!(settings.l_min_duty, 0.0283_f32);
    assert_eq!(settings.l_max_duty, 0.95_f32);
    assert_eq!(settings.l_watt_max, 0.030273438_f32);
    assert_eq!(settings.l_watt_min, 0.03125_f32);
    assert_eq!(settings.l_current_max_scale, 0.8_f32);
    assert_eq!(settings.l_current_min_scale, 0.0332_f32);
    assert_eq!(settings.l_duty_start, 0.0341_f32);
    assert_eq!(settings.cc_min_current, 0.14746094_f32);
    assert_eq!(settings.m_fault_stop_time_ms, -54);
    assert_eq!(settings.m_encoder_counts, 57);
    assert_eq!(settings.m_encoder_sin_amp, 0.154_f32);
    assert_eq!(settings.m_encoder_cos_amp, 0.155_f32);
    assert_eq!(settings.m_encoder_sin_offset, 0.156_f32);
    assert_eq!(settings.m_encoder_cos_offset, 0.157_f32);
    assert_eq!(settings.m_encoder_sincos_filter_constant, 0.158_f32);
    assert_eq!(settings.m_encoder_sincos_phase_correction, 0.159_f32);
    assert_eq!(settings.m_sensor_port_mode, 7);
    assert!(!settings.m_invert_direction);
    assert_eq!(settings.m_drv8301_oc_mode, 1);
    assert_eq!(settings.m_drv8301_oc_adj, -67);
    assert_eq!(settings.m_ntc_motor_beta, 0.16699219_f32);
    assert_eq!(settings.m_out_aux_mode, 2);
    assert_eq!(settings.m_motor_temp_sens_type, 1);
    assert_eq!(settings.m_ptc_motor_coeff, 0.16992188_f32);
    assert_eq!(settings.m_ntcx_ptcx_res, 0_f32);
    assert_eq!(settings.m_ntcx_ptcx_temp_base, 0.1_f32);
    assert_eq!(settings.m_batt_filter_const, -78);
    assert_eq!(settings.bms_type, 1);
    assert_eq!(settings.bms_limit_mode, 87);
    assert_eq!(settings.bms_t_limit_start, -88);
    assert_eq!(settings.bms_t_limit_end, -89);
    assert_eq!(settings.bms_soc_limit_start, 0.185_f32);
    assert_eq!(settings.bms_soc_limit_end, 0.186_f32);
    assert_eq!(settings.bms_vmin_limit_start, 0.187_f32);
    assert_eq!(settings.bms_vmin_limit_end, 0.188_f32);
    assert_eq!(settings.bms_vmax_limit_start, 0.189_f32);
    assert_eq!(settings.bms_vmax_limit_end, 0.19_f32);
    assert_eq!(settings.bms_fwd_can_mode, 0);
    assert_eq!(settings.l_additional_faults, None);
    assert_eq!(settings.l_erpm_abs_overspeed, None);
    assert_eq!(settings.l_in_current_max_scale, None);
    assert_eq!(settings.l_in_current_min_scale, None);
    let settings = view.foc().unwrap();
    assert_eq!(settings.foc_current_kp, 0.05078125_f32);
    assert_eq!(settings.foc_current_ki, 0.051757812_f32);
    assert_eq!(settings.foc_f_zv, 0.052734375_f32);
    assert_eq!(settings.foc_dt_us, 0.053710938_f32);
    assert!(settings.foc_encoder_inverted);
    assert_eq!(settings.foc_encoder_offset, 0.055664062_f32);
    assert_eq!(settings.foc_encoder_ratio, 0.056640625_f32);
    assert_eq!(settings.foc_sensor_mode, 4);
    assert_eq!(settings.foc_pll_kp, 0.05859375_f32);
    assert_eq!(settings.foc_pll_ki, 0.059570312_f32);
    assert_eq!(settings.foc_motor_l, 0.060546875_f32);
    assert_eq!(settings.foc_motor_ld_lq_diff, 0.061523438_f32);
    assert_eq!(settings.foc_motor_r, 0.0625_f32);
    assert_eq!(settings.foc_motor_flux_linkage, 0.06347656_f32);
    assert_eq!(settings.foc_observer_gain, 0.064453125_f32);
    assert_eq!(settings.foc_observer_gain_slow, 0.06542969_f32);
    assert_eq!(settings.foc_observer_offset, 0.066_f32);
    assert_eq!(settings.foc_duty_dowmramp_kp, 0.06738281_f32);
    assert_eq!(settings.foc_duty_dowmramp_ki, 0.068359375_f32);
    assert_eq!(settings.foc_start_curr_dec, 0.0693_f32);
    assert_eq!(settings.foc_start_curr_dec_rpm, 0.0703125_f32);
    assert_eq!(settings.foc_openloop_rpm, 0.07128906_f32);
    assert_eq!(settings.foc_openloop_rpm_low, 0.072_f32);
    assert_eq!(settings.foc_d_gain_scale_start, Some(0.073_f32));
    assert_eq!(settings.foc_d_gain_scale_max_mod, Some(0.074_f32));
    assert_eq!(settings.foc_sl_openloop_hyst, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_lock, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_ramp, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_boost_q, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_max_q, 0.08_f32);
    assert_eq!(
        settings.foc_hall_table,
        [-83, -84, -85, -86, -87, -88, -89, -90]
    );
    assert_eq!(settings.foc_hall_interp_erpm, 0.08886719_f32);
    assert_eq!(settings.foc_sl_erpm_start, 0.08984375_f32);
    assert_eq!(settings.foc_sl_erpm, 0.09082031_f32);
    assert_eq!(settings.foc_control_sample_mode, 0);
    assert_eq!(settings.foc_current_sample_mode, 1);
    assert_eq!(settings.foc_sat_comp_mode, 3);
    assert_eq!(settings.foc_sat_comp, 0.094_f32);
    assert!(settings.foc_temp_comp);
    assert_eq!(settings.foc_temp_comp_base_temp, 0.09_f32);
    assert_eq!(settings.foc_current_filter_const, 0.0976_f32);
    assert_eq!(settings.foc_cc_decoupling, 0);
    assert_eq!(settings.foc_observer_type, 3);
    assert_eq!(settings.foc_hfi_voltage_start, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_run, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_max, 0.1_f32);
    assert_eq!(settings.foc_hfi_gain, 0.103_f32);
    assert_eq!(settings.foc_hfi_max_err, 0.104_f32);
    assert_eq!(settings.foc_hfi_hyst, 0.1_f32);
    assert_eq!(settings.foc_sl_erpm_hfi, 0.10644531_f32);
    assert_eq!(settings.foc_hfi_start_samples, 10);
    assert_eq!(settings.foc_hfi_obs_ovr_sec, 0.10839844_f32);
    assert_eq!(settings.foc_hfi_samples, 0);
    assert_eq!(settings.foc_offsets_cal_on_boot, Some(false));
    assert_eq!(
        settings.foc_offsets_current,
        [0.111328125_f32, 0.11230469_f32, 0.11328125_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage,
        [0.1142_f32, 0.1152_f32, 0.1162_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage_undriven,
        [0.1171_f32, 0.1181_f32, 0.1191_f32]
    );
    assert!(!settings.foc_phase_filter_enable);
    assert!(settings.foc_phase_filter_disable_fault);
    assert_eq!(settings.foc_phase_filter_max_erpm, 0.12207031_f32);
    assert_eq!(settings.foc_mtpa_mode, 2);
    assert_eq!(settings.foc_fw_current_max, 100.5_f32);
    assert_eq!(settings.foc_fw_duty_start, 0.9_f32);
    assert_eq!(settings.foc_fw_ramp_time, 0.125_f32);
    assert_eq!(settings.foc_fw_q_current_factor, 0.1269_f32);
    assert_eq!(settings.foc_speed_soure, 0);
    assert!(settings.foc_short_ls_on_zero_duty);
    assert_eq!(settings.m_hall_extra_samples, -77);
    assert_eq!(settings.foc_hfi_amb_mode, None);
    assert_eq!(settings.foc_hfi_amb_current, None);
    assert_eq!(settings.foc_hfi_amb_tres, None);
    assert_eq!(settings.foc_offsets_cal_mode, None);
    assert_eq!(settings.foc_overmod_factor, None);
    assert_eq!(settings.foc_hfi_reset_erpm, None);
    assert_eq!(settings.foc_fw_backoff, None);
    assert_eq!(settings.foc_mag_vd_max, None);
    let settings = view.bldc().unwrap();
    assert_eq!(settings.pwm_mode, 0);
    assert_eq!(settings.comm_mode, 1);
    assert_eq!(settings.sensor_mode, 0);
    assert_eq!(settings.l_max_erpm_fbrake, 0.0146484375_f32);
    assert_eq!(settings.l_max_erpm_fbrake_cc, 0.015625_f32);
    assert_eq!(settings.sl_min_erpm, 0.03515625_f32);
    assert_eq!(settings.sl_min_erpm_cycle_int_limit, 0.036132812_f32);
    assert_eq!(
        settings.sl_max_fullbreak_current_dir_change,
        0.037109375_f32
    );
    assert_eq!(settings.sl_cycle_int_limit, 0_f32);
    assert_eq!(settings.sl_phase_advance_at_br, 0.039_f32);
    assert_eq!(settings.sl_cycle_int_rpm_br, 0.040039062_f32);
    assert_eq!(settings.sl_bemf_coupling_k, 0.041015625_f32);
    assert_eq!(settings.hall_table, [43, 44, 45, 46, 47, 48, 49, 50]);
    assert_eq!(settings.hall_sl_erpm, 0.049804688_f32);
    assert_eq!(settings.cc_startup_boost_duty, 0.1464_f32);
    assert_eq!(settings.cc_gain, 0.1484375_f32);
    assert_eq!(settings.cc_ramp_step_max, 0.1494_f32);
    assert_eq!(settings.m_duty_ramp_step, 0.1513_f32);
    assert_eq!(settings.m_current_backoff_gain, 0.15234375_f32);
    assert_eq!(settings.m_bldc_f_sw_min, 0.1640625_f32);
    assert_eq!(settings.m_bldc_f_sw_max, 0.16503906_f32);
    let settings = view.dc().unwrap();
    assert_eq!(settings.m_dc_f_sw, 0.16601562_f32);
    let settings = view.pid_controllers().unwrap();
    assert_eq!(settings.sp_pid_loop_rate, 6);
    assert_eq!(settings.s_pid_kp, 0.13085938_f32);
    assert_eq!(settings.s_pid_ki, 0.13183594_f32);
    assert_eq!(settings.s_pid_kd, 0.1328125_f32);
    assert_eq!(settings.s_pid_kd_filter, 0.1337_f32);
    assert_eq!(settings.s_pid_min_erpm, 0.13476562_f32);
    assert!(!settings.s_pid_allow_braking);
    assert_eq!(settings.s_pid_ramp_erpms_s, 0.13671875_f32);
    assert_eq!(settings.s_pid_speed_source, 2);
    assert_eq!(settings.p_pid_kp, 0.13867188_f32);
    assert_eq!(settings.p_pid_ki, 0.13964844_f32);
    assert_eq!(settings.p_pid_kd, 0.140625_f32);
    assert_eq!(settings.p_pid_kd_proc, 0.14160156_f32);
    assert_eq!(settings.p_pid_kd_filter, 0.1425_f32);
    assert_eq!(settings.p_pid_ang_div, 0.14355469_f32);
    assert_eq!(settings.p_pid_gain_dec_angle, 0.1_f32);
    assert_eq!(settings.p_pid_offset, 0.14550781_f32);
    let settings = view.additional_info().unwrap();
    assert_eq!(settings.si_motor_poles, -79);
    assert_eq!(settings.si_gear_ratio, 0.17578125_f32);
    assert_eq!(settings.si_wheel_diameter, 0.17675781_f32);
    assert_eq!(settings.si_battery_type, 1);
    assert_eq!(settings.si_battery_cells, -83);
    assert_eq!(settings.si_battery_ah, 0.1796875_f32);
    assert_eq!(settings.si_motor_nl_current, 0.18066406_f32);
}

#[test]
fn all_groups_6_06() {
    let frame = super::frame(include_bytes!("../fixtures/mcconf-6.06.bin"));
    let (_, view) = McConfView::decode(&frame).unwrap();
    let settings = view.general().unwrap();
    assert_eq!(settings.motor_type, 2);
    assert_eq!(settings.l_current_max, 178.75_f32);
    assert_eq!(settings.l_current_min, 0.005859375_f32);
    assert_eq!(settings.l_in_current_max, 0.0068359375_f32);
    assert_eq!(settings.l_in_current_min, 0.0078125_f32);
    assert_eq!(settings.l_in_current_map_start, 0.0087_f32);
    assert_eq!(settings.l_in_current_map_filter, 0.0097_f32);
    assert_eq!(settings.l_abs_current_max, 268.125_f32);
    assert_eq!(settings.l_min_erpm, 0.01171875_f32);
    assert_eq!(settings.l_max_erpm, 0.0126953125_f32);
    assert_eq!(settings.l_erpm_start, 0.0136_f32);
    assert_eq!(settings.l_min_vin, 0_f32);
    assert_eq!(settings.l_max_vin, 0_f32);
    assert_eq!(settings.l_battery_cut_start, 0_f32);
    assert_eq!(settings.l_battery_cut_end, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_start, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_end, 0_f32);
    assert!(!settings.l_slow_abs_current);
    assert_eq!(settings.l_temp_fet_start, -24);
    assert_eq!(settings.l_temp_fet_end, -25);
    assert_eq!(settings.l_temp_motor_start, -26);
    assert_eq!(settings.l_temp_motor_end, -27);
    assert_eq!(settings.l_temp_accel_dec, 0.0273_f32);
    assert_eq!(settings.l_min_duty, 0.0283_f32);
    assert_eq!(settings.l_max_duty, 0.95_f32);
    assert_eq!(settings.l_watt_max, 0.030273438_f32);
    assert_eq!(settings.l_watt_min, 0.03125_f32);
    assert_eq!(settings.l_current_max_scale, 0.8_f32);
    assert_eq!(settings.l_current_min_scale, 0.0332_f32);
    assert_eq!(settings.l_duty_start, 0.0341_f32);
    assert_eq!(settings.cc_min_current, 0.15136719_f32);
    assert_eq!(settings.m_fault_stop_time_ms, -58);
    assert_eq!(settings.m_encoder_counts, 61);
    assert_eq!(settings.m_encoder_sin_amp, 0.158_f32);
    assert_eq!(settings.m_encoder_cos_amp, 0.159_f32);
    assert_eq!(settings.m_encoder_sin_offset, 0.16_f32);
    assert_eq!(settings.m_encoder_cos_offset, 0.161_f32);
    assert_eq!(settings.m_encoder_sincos_filter_constant, 0.162_f32);
    assert_eq!(settings.m_encoder_sincos_phase_correction, 0.163_f32);
    assert_eq!(settings.m_sensor_port_mode, 2);
    assert!(!settings.m_invert_direction);
    assert_eq!(settings.m_drv8301_oc_mode, 1);
    assert_eq!(settings.m_drv8301_oc_adj, -71);
    assert_eq!(settings.m_ntc_motor_beta, 0.17089844_f32);
    assert_eq!(settings.m_out_aux_mode, 6);
    assert_eq!(settings.m_motor_temp_sens_type, 5);
    assert_eq!(settings.m_ptc_motor_coeff, 0.17382812_f32);
    assert_eq!(settings.m_ntcx_ptcx_res, 0_f32);
    assert_eq!(settings.m_ntcx_ptcx_temp_base, 0.1_f32);
    assert_eq!(settings.m_batt_filter_const, -82);
    assert_eq!(settings.bms_type, 1);
    assert_eq!(settings.bms_limit_mode, 91);
    assert_eq!(settings.bms_t_limit_start, -92);
    assert_eq!(settings.bms_t_limit_end, -93);
    assert_eq!(settings.bms_soc_limit_start, 0.189_f32);
    assert_eq!(settings.bms_soc_limit_end, 0.19_f32);
    assert_eq!(settings.bms_vmin_limit_start, 0.191_f32);
    assert_eq!(settings.bms_vmin_limit_end, 0.192_f32);
    assert_eq!(settings.bms_vmax_limit_start, 0.193_f32);
    assert_eq!(settings.bms_vmax_limit_end, 0.194_f32);
    assert_eq!(settings.bms_fwd_can_mode, 1);
    assert_eq!(settings.l_additional_faults, None);
    assert_eq!(settings.l_erpm_abs_overspeed, None);
    assert_eq!(settings.l_in_current_max_scale, None);
    assert_eq!(settings.l_in_current_min_scale, None);
    let settings = view.foc().unwrap();
    assert_eq!(settings.foc_current_kp, 0.05078125_f32);
    assert_eq!(settings.foc_current_ki, 0.051757812_f32);
    assert_eq!(settings.foc_f_zv, 0.052734375_f32);
    assert_eq!(settings.foc_dt_us, 0.053710938_f32);
    assert!(settings.foc_encoder_inverted);
    assert_eq!(settings.foc_encoder_offset, 0.055664062_f32);
    assert_eq!(settings.foc_encoder_ratio, 0.056640625_f32);
    assert_eq!(settings.foc_sensor_mode, 4);
    assert_eq!(settings.foc_pll_kp, 0.05859375_f32);
    assert_eq!(settings.foc_pll_ki, 0.059570312_f32);
    assert_eq!(settings.foc_motor_l, 0.060546875_f32);
    assert_eq!(settings.foc_motor_ld_lq_diff, 0.061523438_f32);
    assert_eq!(settings.foc_motor_r, 0.0625_f32);
    assert_eq!(settings.foc_motor_flux_linkage, 0.06347656_f32);
    assert_eq!(settings.foc_observer_gain, 0.064453125_f32);
    assert_eq!(settings.foc_observer_gain_slow, 0.06542969_f32);
    assert_eq!(settings.foc_observer_offset, 0.066_f32);
    assert_eq!(settings.foc_duty_dowmramp_kp, 0.06738281_f32);
    assert_eq!(settings.foc_duty_dowmramp_ki, 0.068359375_f32);
    assert_eq!(settings.foc_start_curr_dec, 0.0693_f32);
    assert_eq!(settings.foc_start_curr_dec_rpm, 0.0703125_f32);
    assert_eq!(settings.foc_openloop_rpm, 0.07128906_f32);
    assert_eq!(settings.foc_openloop_rpm_low, 0.072_f32);
    assert_eq!(settings.foc_d_gain_scale_start, Some(0.073_f32));
    assert_eq!(settings.foc_d_gain_scale_max_mod, Some(0.074_f32));
    assert_eq!(settings.foc_sl_openloop_hyst, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_lock, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_ramp, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_boost_q, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_max_q, 0.08_f32);
    assert_eq!(
        settings.foc_hall_table,
        [-83, -84, -85, -86, -87, -88, -89, -90]
    );
    assert_eq!(settings.foc_hall_interp_erpm, 0.08886719_f32);
    assert_eq!(settings.foc_sl_erpm_start, 0.08984375_f32);
    assert_eq!(settings.foc_sl_erpm, 0.09082031_f32);
    assert_eq!(settings.foc_control_sample_mode, 0);
    assert_eq!(settings.foc_current_sample_mode, 1);
    assert_eq!(settings.foc_sat_comp_mode, 3);
    assert_eq!(settings.foc_sat_comp, 0.094_f32);
    assert!(settings.foc_temp_comp);
    assert_eq!(settings.foc_temp_comp_base_temp, 0.09_f32);
    assert_eq!(settings.foc_current_filter_const, 0.0976_f32);
    assert_eq!(settings.foc_cc_decoupling, 0);
    assert_eq!(settings.foc_observer_type, 3);
    assert_eq!(settings.foc_hfi_voltage_start, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_run, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_max, 0.1_f32);
    assert_eq!(settings.foc_hfi_gain, 0.106_f32);
    assert_eq!(settings.foc_hfi_max_err, 0.107_f32);
    assert_eq!(settings.foc_hfi_hyst, 0.1_f32);
    assert_eq!(settings.foc_sl_erpm_hfi, 0.109375_f32);
    assert_eq!(settings.foc_hfi_start_samples, 13);
    assert_eq!(settings.foc_hfi_obs_ovr_sec, 0.111328125_f32);
    assert_eq!(settings.foc_hfi_samples, 0);
    assert_eq!(settings.foc_offsets_cal_on_boot, None);
    assert_eq!(
        settings.foc_offsets_current,
        [0.11425781_f32, 0.115234375_f32, 0.11621094_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage,
        [0.1171_f32, 0.1181_f32, 0.1191_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage_undriven,
        [0.1201_f32, 0.121_f32, 0.122_f32]
    );
    assert!(settings.foc_phase_filter_enable);
    assert!(!settings.foc_phase_filter_disable_fault);
    assert_eq!(settings.foc_phase_filter_max_erpm, 0.125_f32);
    assert_eq!(settings.foc_mtpa_mode, 2);
    assert_eq!(settings.foc_fw_current_max, 100.5_f32);
    assert_eq!(settings.foc_fw_duty_start, 0.9_f32);
    assert_eq!(settings.foc_fw_ramp_time, 0.128_f32);
    assert_eq!(settings.foc_fw_q_current_factor, 0.1298_f32);
    assert_eq!(settings.foc_speed_soure, 1);
    assert!(!settings.foc_short_ls_on_zero_duty);
    assert_eq!(settings.m_hall_extra_samples, -81);
    assert_eq!(settings.foc_hfi_amb_mode, Some(0));
    assert_eq!(settings.foc_hfi_amb_current, Some(0.1_f32));
    assert_eq!(settings.foc_hfi_amb_tres, Some(-5));
    assert_eq!(settings.foc_offsets_cal_mode, Some(16));
    assert_eq!(settings.foc_overmod_factor, Some(0.1328_f32));
    assert_eq!(settings.foc_hfi_reset_erpm, None);
    assert_eq!(settings.foc_fw_backoff, None);
    assert_eq!(settings.foc_mag_vd_max, None);
    let settings = view.bldc().unwrap();
    assert_eq!(settings.pwm_mode, 0);
    assert_eq!(settings.comm_mode, 1);
    assert_eq!(settings.sensor_mode, 0);
    assert_eq!(settings.l_max_erpm_fbrake, 0.0146484375_f32);
    assert_eq!(settings.l_max_erpm_fbrake_cc, 0.015625_f32);
    assert_eq!(settings.sl_min_erpm, 0.03515625_f32);
    assert_eq!(settings.sl_min_erpm_cycle_int_limit, 0.036132812_f32);
    assert_eq!(
        settings.sl_max_fullbreak_current_dir_change,
        0.037109375_f32
    );
    assert_eq!(settings.sl_cycle_int_limit, 0_f32);
    assert_eq!(settings.sl_phase_advance_at_br, 0.039_f32);
    assert_eq!(settings.sl_cycle_int_rpm_br, 0.040039062_f32);
    assert_eq!(settings.sl_bemf_coupling_k, 0.041015625_f32);
    assert_eq!(settings.hall_table, [43, 44, 45, 46, 47, 48, 49, 50]);
    assert_eq!(settings.hall_sl_erpm, 0.049804688_f32);
    assert_eq!(settings.cc_startup_boost_duty, 0.1503_f32);
    assert_eq!(settings.cc_gain, 0.15234375_f32);
    assert_eq!(settings.cc_ramp_step_max, 0.1533_f32);
    assert_eq!(settings.m_duty_ramp_step, 0.1552_f32);
    assert_eq!(settings.m_current_backoff_gain, 0.15625_f32);
    assert_eq!(settings.m_bldc_f_sw_min, 0.16796875_f32);
    assert_eq!(settings.m_bldc_f_sw_max, 0.16894531_f32);
    let settings = view.dc().unwrap();
    assert_eq!(settings.m_dc_f_sw, 0.16992188_f32);
    let settings = view.pid_controllers().unwrap();
    assert_eq!(settings.sp_pid_loop_rate, 1);
    assert_eq!(settings.s_pid_kp, 0.13476562_f32);
    assert_eq!(settings.s_pid_ki, 0.13574219_f32);
    assert_eq!(settings.s_pid_kd, 0.13671875_f32);
    assert_eq!(settings.s_pid_kd_filter, 0.1376_f32);
    assert_eq!(settings.s_pid_min_erpm, 0.13867188_f32);
    assert!(!settings.s_pid_allow_braking);
    assert_eq!(settings.s_pid_ramp_erpms_s, 0.140625_f32);
    assert_eq!(settings.s_pid_speed_source, 0);
    assert_eq!(settings.p_pid_kp, 0.14257812_f32);
    assert_eq!(settings.p_pid_ki, 0.14355469_f32);
    assert_eq!(settings.p_pid_kd, 0.14453125_f32);
    assert_eq!(settings.p_pid_kd_proc, 0.14550781_f32);
    assert_eq!(settings.p_pid_kd_filter, 0.1464_f32);
    assert_eq!(settings.p_pid_ang_div, 0.14746094_f32);
    assert_eq!(settings.p_pid_gain_dec_angle, 0.1_f32);
    assert_eq!(settings.p_pid_offset, 0.14941406_f32);
    let settings = view.additional_info().unwrap();
    assert_eq!(settings.si_motor_poles, -83);
    assert_eq!(settings.si_gear_ratio, 0.1796875_f32);
    assert_eq!(settings.si_wheel_diameter, 0.18066406_f32);
    assert_eq!(settings.si_battery_type, 2);
    assert_eq!(settings.si_battery_cells, -87);
    assert_eq!(settings.si_battery_ah, 0.18359375_f32);
    assert_eq!(settings.si_motor_nl_current, 0.18457031_f32);
}

#[test]
fn all_groups_7_00() {
    let frame = super::frame(include_bytes!("../fixtures/mcconf-7.00.bin"));
    let (_, view) = McConfView::decode(&frame).unwrap();
    let settings = view.general().unwrap();
    assert_eq!(settings.motor_type, 2);
    assert_eq!(settings.l_current_max, 178.75_f32);
    assert_eq!(settings.l_current_min, 0.005859375_f32);
    assert_eq!(settings.l_in_current_max, 0.0068359375_f32);
    assert_eq!(settings.l_in_current_min, 0.0078125_f32);
    assert_eq!(settings.l_in_current_map_start, 0.0087_f32);
    assert_eq!(settings.l_in_current_map_filter, 0.0097_f32);
    assert_eq!(settings.l_abs_current_max, 268.125_f32);
    assert_eq!(settings.l_min_erpm, 0.01171875_f32);
    assert_eq!(settings.l_max_erpm, 0.0126953125_f32);
    assert_eq!(settings.l_erpm_start, 0.0136_f32);
    assert_eq!(settings.l_min_vin, 0_f32);
    assert_eq!(settings.l_max_vin, 0_f32);
    assert_eq!(settings.l_battery_cut_start, 0_f32);
    assert_eq!(settings.l_battery_cut_end, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_start, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_end, 0_f32);
    assert!(!settings.l_slow_abs_current);
    assert_eq!(settings.l_temp_fet_start, -24);
    assert_eq!(settings.l_temp_fet_end, -25);
    assert_eq!(settings.l_temp_motor_start, -26);
    assert_eq!(settings.l_temp_motor_end, -27);
    assert_eq!(settings.l_temp_accel_dec, 0.0273_f32);
    assert_eq!(settings.l_min_duty, 0.0283_f32);
    assert_eq!(settings.l_max_duty, 0.95_f32);
    assert_eq!(settings.l_watt_max, 0.030273438_f32);
    assert_eq!(settings.l_watt_min, 0.03125_f32);
    assert_eq!(settings.l_current_max_scale, 0.8_f32);
    assert_eq!(settings.l_current_min_scale, 0.0332_f32);
    assert_eq!(settings.l_duty_start, 0.0341_f32);
    assert_eq!(settings.cc_min_current, 0.15332031_f32);
    assert_eq!(settings.m_fault_stop_time_ms, -60);
    assert_eq!(settings.m_encoder_counts, 63);
    assert_eq!(settings.m_encoder_sin_amp, 0.16_f32);
    assert_eq!(settings.m_encoder_cos_amp, 0.161_f32);
    assert_eq!(settings.m_encoder_sin_offset, 0.162_f32);
    assert_eq!(settings.m_encoder_cos_offset, 0.163_f32);
    assert_eq!(settings.m_encoder_sincos_filter_constant, 0.164_f32);
    assert_eq!(settings.m_encoder_sincos_phase_correction, 0.165_f32);
    assert_eq!(settings.m_sensor_port_mode, 16);
    assert!(!settings.m_invert_direction);
    assert_eq!(settings.m_drv8301_oc_mode, 3);
    assert_eq!(settings.m_drv8301_oc_adj, -73);
    assert_eq!(settings.m_ntc_motor_beta, 0.17285156_f32);
    assert_eq!(settings.m_out_aux_mode, 8);
    assert_eq!(settings.m_motor_temp_sens_type, 7);
    assert_eq!(settings.m_ptc_motor_coeff, 0.17578125_f32);
    assert_eq!(settings.m_ntcx_ptcx_res, 0_f32);
    assert_eq!(settings.m_ntcx_ptcx_temp_base, 0.1_f32);
    assert_eq!(settings.m_batt_filter_const, -84);
    assert_eq!(settings.bms_type, 1);
    assert_eq!(settings.bms_limit_mode, 93);
    assert_eq!(settings.bms_t_limit_start, -94);
    assert_eq!(settings.bms_t_limit_end, -95);
    assert_eq!(settings.bms_soc_limit_start, 0.191_f32);
    assert_eq!(settings.bms_soc_limit_end, 0.192_f32);
    assert_eq!(settings.bms_vmin_limit_start, 0.193_f32);
    assert_eq!(settings.bms_vmin_limit_end, 0.194_f32);
    assert_eq!(settings.bms_vmax_limit_start, 0.195_f32);
    assert_eq!(settings.bms_vmax_limit_end, 0.196_f32);
    assert_eq!(settings.bms_fwd_can_mode, 0);
    assert_eq!(settings.l_additional_faults, Some(36));
    assert_eq!(settings.l_erpm_abs_overspeed, None);
    assert_eq!(settings.l_in_current_max_scale, None);
    assert_eq!(settings.l_in_current_min_scale, None);
    let settings = view.foc().unwrap();
    assert_eq!(settings.foc_current_kp, 0.051757812_f32);
    assert_eq!(settings.foc_current_ki, 0.052734375_f32);
    assert_eq!(settings.foc_f_zv, 0.053710938_f32);
    assert_eq!(settings.foc_dt_us, 0.0546875_f32);
    assert!(!settings.foc_encoder_inverted);
    assert_eq!(settings.foc_encoder_offset, 0.056640625_f32);
    assert_eq!(settings.foc_encoder_ratio, 0.057617188_f32);
    assert_eq!(settings.foc_sensor_mode, 9);
    assert_eq!(settings.foc_pll_kp, 0.059570312_f32);
    assert_eq!(settings.foc_pll_ki, 0.060546875_f32);
    assert_eq!(settings.foc_motor_l, 0.061523438_f32);
    assert_eq!(settings.foc_motor_ld_lq_diff, 0.0625_f32);
    assert_eq!(settings.foc_motor_r, 0.06347656_f32);
    assert_eq!(settings.foc_motor_flux_linkage, 0.064453125_f32);
    assert_eq!(settings.foc_observer_gain, 0.06542969_f32);
    assert_eq!(settings.foc_observer_gain_slow, 0.06640625_f32);
    assert_eq!(settings.foc_observer_offset, 0.067_f32);
    assert_eq!(settings.foc_duty_dowmramp_kp, 0.068359375_f32);
    assert_eq!(settings.foc_duty_dowmramp_ki, 0.06933594_f32);
    assert_eq!(settings.foc_start_curr_dec, 0.0703_f32);
    assert_eq!(settings.foc_start_curr_dec_rpm, 0.07128906_f32);
    assert_eq!(settings.foc_openloop_rpm, 0.072265625_f32);
    assert_eq!(settings.foc_openloop_rpm_low, 0.073_f32);
    assert_eq!(settings.foc_d_gain_scale_start, None);
    assert_eq!(settings.foc_d_gain_scale_max_mod, None);
    assert_eq!(settings.foc_sl_openloop_hyst, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_lock, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_ramp, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_boost_q, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_max_q, 0.07_f32);
    assert_eq!(
        settings.foc_hall_table,
        [-82, -83, -84, -85, -86, -87, -88, -89]
    );
    assert_eq!(settings.foc_hall_interp_erpm, 0.087890625_f32);
    assert_eq!(settings.foc_sl_erpm_start, 0.08886719_f32);
    assert_eq!(settings.foc_sl_erpm, 0.08984375_f32);
    assert_eq!(settings.foc_control_sample_mode, 2);
    assert_eq!(settings.foc_current_sample_mode, 0);
    assert_eq!(settings.foc_sat_comp_mode, 2);
    assert_eq!(settings.foc_sat_comp, 0.093_f32);
    assert!(!settings.foc_temp_comp);
    assert_eq!(settings.foc_temp_comp_base_temp, 0.09_f32);
    assert_eq!(settings.foc_current_filter_const, 0.0966_f32);
    assert_eq!(settings.foc_cc_decoupling, 3);
    assert_eq!(settings.foc_observer_type, 2);
    assert_eq!(settings.foc_hfi_voltage_start, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_run, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_max, 0.1_f32);
    assert_eq!(settings.foc_hfi_gain, 0.105_f32);
    assert_eq!(settings.foc_hfi_max_err, 0.106_f32);
    assert_eq!(settings.foc_hfi_hyst, 0.1_f32);
    assert_eq!(settings.foc_sl_erpm_hfi, 0.10839844_f32);
    assert_eq!(settings.foc_hfi_start_samples, 13);
    assert_eq!(settings.foc_hfi_obs_ovr_sec, 0.111328125_f32);
    assert_eq!(settings.foc_hfi_samples, 0);
    assert_eq!(settings.foc_offsets_cal_on_boot, None);
    assert_eq!(
        settings.foc_offsets_current,
        [0.11425781_f32, 0.115234375_f32, 0.11621094_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage,
        [0.1171_f32, 0.1181_f32, 0.1191_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage_undriven,
        [0.1201_f32, 0.121_f32, 0.122_f32]
    );
    assert!(settings.foc_phase_filter_enable);
    assert!(!settings.foc_phase_filter_disable_fault);
    assert_eq!(settings.foc_phase_filter_max_erpm, 0.125_f32);
    assert_eq!(settings.foc_mtpa_mode, 2);
    assert_eq!(settings.foc_fw_current_max, 100.5_f32);
    assert_eq!(settings.foc_fw_duty_start, 0.9_f32);
    assert_eq!(settings.foc_fw_ramp_time, 0.128_f32);
    assert_eq!(settings.foc_fw_q_current_factor, 0.1298_f32);
    assert_eq!(settings.foc_speed_soure, 0);
    assert!(settings.foc_short_ls_on_zero_duty);
    assert_eq!(settings.m_hall_extra_samples, -83);
    assert_eq!(settings.foc_hfi_amb_mode, Some(2));
    assert_eq!(settings.foc_hfi_amb_current, Some(0.1_f32));
    assert_eq!(settings.foc_hfi_amb_tres, Some(-4));
    assert_eq!(settings.foc_offsets_cal_mode, Some(16));
    assert_eq!(settings.foc_overmod_factor, Some(0.1337_f32));
    assert_eq!(settings.foc_hfi_reset_erpm, Some(0.109375_f32));
    assert_eq!(settings.foc_fw_backoff, Some(0.13_f32));
    assert_eq!(settings.foc_mag_vd_max, Some(0.1347_f32));
    let settings = view.bldc().unwrap();
    assert_eq!(settings.pwm_mode, 0);
    assert_eq!(settings.comm_mode, 1);
    assert_eq!(settings.sensor_mode, 0);
    assert_eq!(settings.l_max_erpm_fbrake, 0.0146484375_f32);
    assert_eq!(settings.l_max_erpm_fbrake_cc, 0.015625_f32);
    assert_eq!(settings.sl_min_erpm, 0.036132812_f32);
    assert_eq!(settings.sl_min_erpm_cycle_int_limit, 0.037109375_f32);
    assert_eq!(
        settings.sl_max_fullbreak_current_dir_change,
        0.038085938_f32
    );
    assert_eq!(settings.sl_cycle_int_limit, 0_f32);
    assert_eq!(settings.sl_phase_advance_at_br, 0.04_f32);
    assert_eq!(settings.sl_cycle_int_rpm_br, 0.041015625_f32);
    assert_eq!(settings.sl_bemf_coupling_k, 0.041992188_f32);
    assert_eq!(settings.hall_table, [44, 45, 46, 47, 48, 49, 50, 51]);
    assert_eq!(settings.hall_sl_erpm, 0.05078125_f32);
    assert_eq!(settings.cc_startup_boost_duty, 0.1523_f32);
    assert_eq!(settings.cc_gain, 0.15429688_f32);
    assert_eq!(settings.cc_ramp_step_max, 0.1552_f32);
    assert_eq!(settings.m_duty_ramp_step, 0.1572_f32);
    assert_eq!(settings.m_current_backoff_gain, 0.15820312_f32);
    assert_eq!(settings.m_bldc_f_sw_min, 0.16992188_f32);
    assert_eq!(settings.m_bldc_f_sw_max, 0.17089844_f32);
    let settings = view.dc().unwrap();
    assert_eq!(settings.m_dc_f_sw, 0.171875_f32);
    let settings = view.pid_controllers().unwrap();
    assert_eq!(settings.sp_pid_loop_rate, 3);
    assert_eq!(settings.s_pid_kp, 0.13671875_f32);
    assert_eq!(settings.s_pid_ki, 0.13769531_f32);
    assert_eq!(settings.s_pid_kd, 0.13867188_f32);
    assert_eq!(settings.s_pid_kd_filter, 0.1396_f32);
    assert_eq!(settings.s_pid_min_erpm, 0.140625_f32);
    assert!(!settings.s_pid_allow_braking);
    assert_eq!(settings.s_pid_ramp_erpms_s, 0.14257812_f32);
    assert_eq!(settings.s_pid_speed_source, 2);
    assert_eq!(settings.p_pid_kp, 0.14453125_f32);
    assert_eq!(settings.p_pid_ki, 0.14550781_f32);
    assert_eq!(settings.p_pid_kd, 0.14648438_f32);
    assert_eq!(settings.p_pid_kd_proc, 0.14746094_f32);
    assert_eq!(settings.p_pid_kd_filter, 0.1484_f32);
    assert_eq!(settings.p_pid_ang_div, 0.14941406_f32);
    assert_eq!(settings.p_pid_gain_dec_angle, 0.1_f32);
    assert_eq!(settings.p_pid_offset, 0.15136719_f32);
    let settings = view.additional_info().unwrap();
    assert_eq!(settings.si_motor_poles, -85);
    assert_eq!(settings.si_gear_ratio, 0.18164062_f32);
    assert_eq!(settings.si_wheel_diameter, 0.18261719_f32);
    assert_eq!(settings.si_battery_type, 1);
    assert_eq!(settings.si_battery_cells, -89);
    assert_eq!(settings.si_battery_ah, 0.18554688_f32);
    assert_eq!(settings.si_motor_nl_current, 0.18652344_f32);
}

#[test]
fn all_groups_8_00() {
    let frame = super::frame(include_bytes!("../fixtures/mcconf-8.00.bin"));
    let (_, view) = McConfView::decode(&frame).unwrap();
    let settings = view.general().unwrap();
    assert_eq!(settings.motor_type, 2);
    assert_eq!(settings.l_current_max, 178.75_f32);
    assert_eq!(settings.l_current_min, 0.005859375_f32);
    assert_eq!(settings.l_in_current_max, 0.0068359375_f32);
    assert_eq!(settings.l_in_current_min, 0.0078125_f32);
    assert_eq!(settings.l_in_current_map_start, 0.0087_f32);
    assert_eq!(settings.l_in_current_map_filter, 0.0097_f32);
    assert_eq!(settings.l_abs_current_max, 268.125_f32);
    assert_eq!(settings.l_min_erpm, 0.01171875_f32);
    assert_eq!(settings.l_max_erpm, 0.0126953125_f32);
    assert_eq!(settings.l_erpm_start, 0.0146_f32);
    assert_eq!(settings.l_min_vin, 0_f32);
    assert_eq!(settings.l_max_vin, 0_f32);
    assert_eq!(settings.l_battery_cut_start, 0_f32);
    assert_eq!(settings.l_battery_cut_end, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_start, 0_f32);
    assert_eq!(settings.l_battery_regen_cut_end, 0_f32);
    assert!(settings.l_slow_abs_current);
    assert_eq!(settings.l_temp_fet_start, -25);
    assert_eq!(settings.l_temp_fet_end, -26);
    assert_eq!(settings.l_temp_motor_start, -27);
    assert_eq!(settings.l_temp_motor_end, -28);
    assert_eq!(settings.l_temp_accel_dec, 0.0283_f32);
    assert_eq!(settings.l_min_duty, 0.0292_f32);
    assert_eq!(settings.l_max_duty, 0.95_f32);
    assert_eq!(settings.l_watt_max, 0.03125_f32);
    assert_eq!(settings.l_watt_min, 0.032226562_f32);
    assert_eq!(settings.l_current_max_scale, 0.8_f32);
    assert_eq!(settings.l_current_min_scale, 0.0341_f32);
    assert_eq!(settings.l_duty_start, 0.0371_f32);
    assert_eq!(settings.cc_min_current, 0.15625_f32);
    assert_eq!(settings.m_fault_stop_time_ms, -63);
    assert_eq!(settings.m_encoder_counts, 66);
    assert_eq!(settings.m_encoder_sin_amp, 0.163_f32);
    assert_eq!(settings.m_encoder_cos_amp, 0.164_f32);
    assert_eq!(settings.m_encoder_sin_offset, 0.165_f32);
    assert_eq!(settings.m_encoder_cos_offset, 0.166_f32);
    assert_eq!(settings.m_encoder_sincos_filter_constant, 0.166_f32);
    assert_eq!(settings.m_encoder_sincos_phase_correction, 0.167_f32);
    assert_eq!(settings.m_sensor_port_mode, 1);
    assert!(settings.m_invert_direction);
    assert_eq!(settings.m_drv8301_oc_mode, 2);
    assert_eq!(settings.m_drv8301_oc_adj, -76);
    assert_eq!(settings.m_ntc_motor_beta, 0.17578125_f32);
    assert_eq!(settings.m_out_aux_mode, 11);
    assert_eq!(settings.m_motor_temp_sens_type, 1);
    assert_eq!(settings.m_ptc_motor_coeff, 0.17871094_f32);
    assert_eq!(settings.m_ntcx_ptcx_res, 0_f32);
    assert_eq!(settings.m_ntcx_ptcx_temp_base, 0.1_f32);
    assert_eq!(settings.m_batt_filter_const, -87);
    assert_eq!(settings.bms_type, 0);
    assert_eq!(settings.bms_limit_mode, 96);
    assert_eq!(settings.bms_t_limit_start, -97);
    assert_eq!(settings.bms_t_limit_end, -98);
    assert_eq!(settings.bms_soc_limit_start, 0.194_f32);
    assert_eq!(settings.bms_soc_limit_end, 0.195_f32);
    assert_eq!(settings.bms_vmin_limit_start, 0.196_f32);
    assert_eq!(settings.bms_vmin_limit_end, 0.197_f32);
    assert_eq!(settings.bms_vmax_limit_start, 0.198_f32);
    assert_eq!(settings.bms_vmax_limit_end, 0.199_f32);
    assert_eq!(settings.bms_fwd_can_mode, 0);
    assert_eq!(settings.l_additional_faults, Some(39));
    assert_eq!(settings.l_erpm_abs_overspeed, Some(0.013671875_f32));
    assert_eq!(settings.l_in_current_max_scale, Some(0.0351_f32));
    assert_eq!(settings.l_in_current_min_scale, Some(0.0361_f32));
    let settings = view.foc().unwrap();
    assert_eq!(settings.foc_current_kp, 0.0546875_f32);
    assert_eq!(settings.foc_current_ki, 0.055664062_f32);
    assert_eq!(settings.foc_f_zv, 0.056640625_f32);
    assert_eq!(settings.foc_dt_us, 0.057617188_f32);
    assert!(settings.foc_encoder_inverted);
    assert_eq!(settings.foc_encoder_offset, 0.059570312_f32);
    assert_eq!(settings.foc_encoder_ratio, 0.060546875_f32);
    assert_eq!(settings.foc_sensor_mode, 2);
    assert_eq!(settings.foc_pll_kp, 0.0625_f32);
    assert_eq!(settings.foc_pll_ki, 0.06347656_f32);
    assert_eq!(settings.foc_motor_l, 0.064453125_f32);
    assert_eq!(settings.foc_motor_ld_lq_diff, 0.06542969_f32);
    assert_eq!(settings.foc_motor_r, 0.06640625_f32);
    assert_eq!(settings.foc_motor_flux_linkage, 0.06738281_f32);
    assert_eq!(settings.foc_observer_gain, 0.068359375_f32);
    assert_eq!(settings.foc_observer_gain_slow, 0.06933594_f32);
    assert_eq!(settings.foc_observer_offset, 0.07_f32);
    assert_eq!(settings.foc_duty_dowmramp_kp, 0.07128906_f32);
    assert_eq!(settings.foc_duty_dowmramp_ki, 0.072265625_f32);
    assert_eq!(settings.foc_start_curr_dec, 0.0732_f32);
    assert_eq!(settings.foc_start_curr_dec_rpm, 0.07421875_f32);
    assert_eq!(settings.foc_openloop_rpm, 0.07519531_f32);
    assert_eq!(settings.foc_openloop_rpm_low, 0.076_f32);
    assert_eq!(settings.foc_d_gain_scale_start, None);
    assert_eq!(settings.foc_d_gain_scale_max_mod, None);
    assert_eq!(settings.foc_sl_openloop_hyst, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_lock, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time_ramp, 0.07_f32);
    assert_eq!(settings.foc_sl_openloop_time, 0.08_f32);
    assert_eq!(settings.foc_sl_openloop_boost_q, 0.08_f32);
    assert_eq!(settings.foc_sl_openloop_max_q, 0.08_f32);
    assert_eq!(
        settings.foc_hall_table,
        [-85, -86, -87, -88, -89, -90, -91, -92]
    );
    assert_eq!(settings.foc_hall_interp_erpm, 0.09082031_f32);
    assert_eq!(settings.foc_sl_erpm_start, 0.091796875_f32);
    assert_eq!(settings.foc_sl_erpm, 0.09277344_f32);
    assert_eq!(settings.foc_control_sample_mode, 2);
    assert_eq!(settings.foc_current_sample_mode, 0);
    assert_eq!(settings.foc_sat_comp_mode, 1);
    assert_eq!(settings.foc_sat_comp, 0.096_f32);
    assert!(settings.foc_temp_comp);
    assert_eq!(settings.foc_temp_comp_base_temp, 0.09_f32);
    assert_eq!(settings.foc_current_filter_const, 0.0996_f32);
    assert_eq!(settings.foc_cc_decoupling, 2);
    assert_eq!(settings.foc_observer_type, 5);
    assert_eq!(settings.foc_hfi_voltage_start, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_run, 0.1_f32);
    assert_eq!(settings.foc_hfi_voltage_max, 0.1_f32);
    assert_eq!(settings.foc_hfi_gain, 0.108_f32);
    assert_eq!(settings.foc_hfi_max_err, 0.109_f32);
    assert_eq!(settings.foc_hfi_hyst, 0.11_f32);
    assert_eq!(settings.foc_sl_erpm_hfi, 0.111328125_f32);
    assert_eq!(settings.foc_hfi_start_samples, 16);
    assert_eq!(settings.foc_hfi_obs_ovr_sec, 0.11425781_f32);
    assert_eq!(settings.foc_hfi_samples, 0);
    assert_eq!(settings.foc_offsets_cal_on_boot, None);
    assert_eq!(
        settings.foc_offsets_current,
        [0.1171875_f32, 0.11816406_f32, 0.119140625_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage,
        [0.1201_f32, 0.121_f32, 0.122_f32]
    );
    assert_eq!(
        settings.foc_offsets_voltage_undriven,
        [0.123_f32, 0.124_f32, 0.125_f32]
    );
    assert!(!settings.foc_phase_filter_enable);
    assert!(settings.foc_phase_filter_disable_fault);
    assert_eq!(settings.foc_phase_filter_max_erpm, 0.12792969_f32);
    assert_eq!(settings.foc_mtpa_mode, 2);
    assert_eq!(settings.foc_fw_current_max, 100.5_f32);
    assert_eq!(settings.foc_fw_duty_start, 0.9_f32);
    assert_eq!(settings.foc_fw_ramp_time, 0.131_f32);
    assert_eq!(settings.foc_fw_q_current_factor, 0.1328_f32);
    assert_eq!(settings.foc_speed_soure, 1);
    assert!(!settings.foc_short_ls_on_zero_duty);
    assert_eq!(settings.m_hall_extra_samples, -86);
    assert_eq!(settings.foc_hfi_amb_mode, Some(2));
    assert_eq!(settings.foc_hfi_amb_current, Some(0.1_f32));
    assert_eq!(settings.foc_hfi_amb_tres, Some(-7));
    assert_eq!(settings.foc_offsets_cal_mode, Some(19));
    assert_eq!(settings.foc_overmod_factor, Some(0.1367_f32));
    assert_eq!(settings.foc_hfi_reset_erpm, Some(0.11230469_f32));
    assert_eq!(settings.foc_fw_backoff, Some(0.133_f32));
    assert_eq!(settings.foc_mag_vd_max, Some(0.1376_f32));
    let settings = view.bldc().unwrap();
    assert_eq!(settings.pwm_mode, 0);
    assert_eq!(settings.comm_mode, 1);
    assert_eq!(settings.sensor_mode, 0);
    assert_eq!(settings.l_max_erpm_fbrake, 0_f32);
    assert_eq!(settings.l_max_erpm_fbrake_cc, 0_f32);
    assert_eq!(settings.sl_min_erpm, 0.0390625_f32);
    assert_eq!(settings.sl_min_erpm_cycle_int_limit, 0.040039062_f32);
    assert_eq!(
        settings.sl_max_fullbreak_current_dir_change,
        0.041015625_f32
    );
    assert_eq!(settings.sl_cycle_int_limit, 0_f32);
    assert_eq!(settings.sl_phase_advance_at_br, 0.0429_f32);
    assert_eq!(settings.sl_cycle_int_rpm_br, 0.043945312_f32);
    assert_eq!(settings.sl_bemf_coupling_k, 0.044921875_f32);
    assert_eq!(settings.hall_table, [47, 48, 49, 50, 51, 52, 53, 54]);
    assert_eq!(settings.hall_sl_erpm, 0.053710938_f32);
    assert_eq!(settings.cc_startup_boost_duty, 0.1552_f32);
    assert_eq!(settings.cc_gain, 0.15722656_f32);
    assert_eq!(settings.cc_ramp_step_max, 0.1582_f32);
    assert_eq!(settings.m_duty_ramp_step, 0.1601_f32);
    assert_eq!(settings.m_current_backoff_gain, 0.16113281_f32);
    assert_eq!(settings.m_bldc_f_sw_min, 0.17285156_f32);
    assert_eq!(settings.m_bldc_f_sw_max, 0.17382812_f32);
    let settings = view.dc().unwrap();
    assert_eq!(settings.m_dc_f_sw, 0.17480469_f32);
    let settings = view.pid_controllers().unwrap();
    assert_eq!(settings.sp_pid_loop_rate, 6);
    assert_eq!(settings.s_pid_kp, 0.13964844_f32);
    assert_eq!(settings.s_pid_ki, 0.140625_f32);
    assert_eq!(settings.s_pid_kd, 0.14160156_f32);
    assert_eq!(settings.s_pid_kd_filter, 0.1425_f32);
    assert_eq!(settings.s_pid_min_erpm, 0.14355469_f32);
    assert!(settings.s_pid_allow_braking);
    assert_eq!(settings.s_pid_ramp_erpms_s, 0.14550781_f32);
    assert_eq!(settings.s_pid_speed_source, 2);
    assert_eq!(settings.p_pid_kp, 0.14746094_f32);
    assert_eq!(settings.p_pid_ki, 0.1484375_f32);
    assert_eq!(settings.p_pid_kd, 0.14941406_f32);
    assert_eq!(settings.p_pid_kd_proc, 0.15039062_f32);
    assert_eq!(settings.p_pid_kd_filter, 0.1513_f32);
    assert_eq!(settings.p_pid_ang_div, 0.15234375_f32);
    assert_eq!(settings.p_pid_gain_dec_angle, 0.1_f32);
    assert_eq!(settings.p_pid_offset, 0.15429688_f32);
    let settings = view.additional_info().unwrap();
    assert_eq!(settings.si_motor_poles, -88);
    assert_eq!(settings.si_gear_ratio, 0.18457031_f32);
    assert_eq!(settings.si_wheel_diameter, 0.18554688_f32);
    assert_eq!(settings.si_battery_type, 1);
    assert_eq!(settings.si_battery_cells, -92);
    assert_eq!(settings.si_battery_ah, 0.18847656_f32);
    assert_eq!(settings.si_motor_nl_current, 0.18945312_f32);
}

#[test]
fn view_borrows_only_the_configuration_and_preserves_the_next_frame() {
    for config in super::CONFIGS {
        let mut frame = super::frame(config);
        let first_len = frame.len();
        frame.extend_from_slice(&[2, 1, 129, 129, 169, 3]);
        let (consumed, view) = McConfView::decode(&frame).unwrap();
        assert_eq!(consumed, first_len);
        assert_eq!(view.as_bytes(), *config);
        assert_eq!(view.as_bytes().as_ptr(), frame[4..].as_ptr());
        assert!(matches!(
            vesc::decode(&frame[consumed..]).unwrap().1,
            vesc::CommandReply::ResetStats
        ));
    }
}

#[test]
fn retained_group_outlives_the_frame() {
    let general = {
        let frame = super::frame(super::CONFIG);
        let (_, view) = McConfView::decode(&frame).unwrap();
        view.general().unwrap()
    };
    assert_eq!(general.l_current_max, 178.75);
}

#[test]
fn view_validates_frames_before_exposing_groups() {
    let frame = super::frame(super::CONFIG);
    for length in 0..frame.len() {
        assert!(McConfView::decode(&frame[..length]).is_err());
    }
    let mut corrupt = frame;
    corrupt[20] ^= 1;
    assert!(matches!(
        McConfView::decode(&corrupt),
        Err(vesc::DecodeError::ChecksumMismatch { .. })
    ));
    assert!(matches!(
        McConfView::decode(&[2, 1, 129, 129, 169, 3]),
        Err(vesc::DecodeError::InvalidFrame)
    ));
    let mut unknown = super::CONFIG.to_vec();
    unknown[..4].copy_from_slice(&0x12345678u32.to_be_bytes());
    assert!(matches!(
        McConfView::decode(&super::frame(&unknown)),
        Err(vesc::DecodeError::UnsupportedMcConf {
            signature: 0x12345678
        })
    ));
}

#[test]
fn streaming_group_decode_uses_the_existing_receive_buffer() {
    for config in super::CONFIGS {
        let frame = super::frame(config);
        let mut decoder = vesc::Decoder::default();
        let mut general = None;
        for chunk in frame.chunks(7) {
            assert_eq!(decoder.feed(chunk).unwrap(), chunk.len());
            if let Some(view) = decoder.next_mcconf().unwrap() {
                general = Some(view.general().unwrap());
            }
        }
        assert_eq!(general.unwrap().l_current_max, 178.75);
        assert!(decoder.next().is_none());
    }
}

#[test]
fn streaming_group_decode_leaves_other_replies_available() {
    let mut frame = vec![2, 1, 129, 129, 169, 3];
    frame.extend_from_slice(&super::frame(super::CONFIG));
    let mut decoder = vesc::Decoder::default();
    assert_eq!(decoder.feed(&frame).unwrap(), frame.len());
    assert!(matches!(
        decoder.next_mcconf(),
        Err(vesc::DecodeError::InvalidFrame)
    ));
    assert!(matches!(
        decoder.next(),
        Some(vesc::CommandReply::ResetStats)
    ));
    let view = decoder.next_mcconf().unwrap().unwrap();
    assert_eq!(view.foc().unwrap().foc_fw_current_max, 100.5);
    assert!(decoder.next().is_none());
}
