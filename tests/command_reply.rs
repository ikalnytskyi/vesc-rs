use googletest::prelude::*;

use vesc::{CommandReply, DecodeError, Values};

#[test]
fn decode_get_values_zero_rpm() {
    let input = [
        2, 74, 4, 1, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        119, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 1, 116, 0, 0, 0, 0, 255, 255, 131, 64, 0, 2, 168, 254,
        0, 18, 6, 65, 224, 20, 1, 21, 252, 216, 252, 202, 0, 0, 0, 8, 0, 0, 0, 12, 0, 218, 113, 3,
    ];

    let expected = (
        eq(&79),
        pat!(&CommandReply::GetValues(pat!(Values {
            temp_mosfet: approx_eq(27.6),
            temp_motor: approx_eq(0.0),
            avg_current_motor: approx_eq(0.0),
            avg_current_input: approx_eq(0.0),
            avg_current_d: approx_eq(0.0),
            avg_current_q: approx_eq(0.0),
            duty_cycle: approx_eq(0.0),
            rpm: approx_eq(0.0),
            voltage_in: approx_eq(37.5),
            amp_hours: approx_eq(0.0009),
            amp_hours_charged: approx_eq(0.0),
            watt_hours: approx_eq(0.0372),
            watt_hours_charged: approx_eq(0.0),
            tachometer: eq(-31936),
            tachometer_abs: eq(174334),
            fault_code: eq(0),
            pid_pos: approx_eq(302.39996),
            controller_id: eq(20),
            temp_mosfet1: approx_eq(27.7),
            temp_mosfet2: approx_eq(-80.8),
            temp_mosfet3: approx_eq(-82.2),
            avg_voltage_d: approx_eq(0.008),
            avg_voltage_q: approx_eq(0.012),
            status: eq(0),
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_get_values_forward_rpm() {
    let input = [
        2, 74, 4, 1, 20, 0, 0, 0, 0, 0, 37, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 32, 0, 110, 0, 0, 3,
        251, 1, 125, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 2, 137, 0, 0, 0, 0, 255, 255, 111, 75, 0, 2,
        159, 199, 0, 4, 106, 124, 40, 1, 1, 21, 252, 76, 252, 13, 0, 0, 0, 229, 0, 0, 8, 214, 0,
        58, 151, 3,
    ];

    let expected = (
        eq(&79),
        pat!(&CommandReply::GetValues(pat!(Values {
            temp_mosfet: approx_eq(27.6),
            temp_motor: approx_eq(0.0),
            avg_current_motor: approx_eq(0.37),
            avg_current_input: approx_eq(0.03),
            avg_current_d: approx_eq(0.0),
            avg_current_q: approx_eq(0.32),
            duty_cycle: approx_eq(0.11),
            rpm: approx_eq(1019.0),
            voltage_in: approx_eq(38.1),
            amp_hours: approx_eq(0.0017),
            amp_hours_charged: approx_eq(0.0),
            watt_hours: approx_eq(0.0649),
            watt_hours_charged: approx_eq(0.0),
            tachometer: eq(-37045),
            tachometer_abs: eq(171975),
            fault_code: eq(0),
            pid_pos: approx_eq(74.08746),
            controller_id: eq(1),
            temp_mosfet1: approx_eq(27.7),
            temp_mosfet2: approx_eq(-94.8),
            temp_mosfet3: approx_eq(-101.1),
            avg_voltage_d: approx_eq(0.229),
            avg_voltage_q: approx_eq(2.262),
            status: eq(0),
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_get_values_reverse_rpm() {
    let input = [
        2, 74, 4, 1, 13, 0, 0, 0, 0, 0, 92, 0, 0, 0, 12, 0, 0, 0, 0, 255, 255, 255, 169, 255, 19,
        255, 255, 247, 94, 1, 117, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 1, 18, 0, 0, 0, 0, 255, 255, 145,
        186, 0, 2, 11, 64, 0, 13, 228, 230, 240, 20, 1, 13, 252, 115, 252, 76, 0, 0, 0, 230, 255,
        255, 240, 129, 0, 12, 51, 3,
    ];

    let expected = (
        eq(&79),
        pat!(&CommandReply::GetValues(pat!(Values {
            temp_mosfet: approx_eq(26.9),
            temp_motor: approx_eq(0.0),
            avg_current_motor: approx_eq(0.92),
            avg_current_input: approx_eq(0.12),
            avg_current_d: approx_eq(0.0),
            avg_current_q: approx_eq(-0.87),
            duty_cycle: approx_eq(-0.237),
            rpm: approx_eq(-2210.0),
            voltage_in: approx_eq(37.3),
            amp_hours: approx_eq(0.0007),
            amp_hours_charged: approx_eq(0.0),
            watt_hours: approx_eq(0.0274),
            watt_hours_charged: approx_eq(0.0),
            tachometer: eq(-28230),
            tachometer_abs: eq(133952),
            fault_code: eq(0),
            pid_pos: approx_eq(233.10513),
            controller_id: eq(20),
            temp_mosfet1: approx_eq(26.9),
            temp_mosfet2: approx_eq(-90.9),
            temp_mosfet3: approx_eq(-94.8),
            avg_voltage_d: approx_eq(0.23),
            avg_voltage_q: approx_eq(-3.967),
            status: eq(0),
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_get_values_selective_zero_rpm() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];

    let expected = (
        eq(&28),
        pat!(&CommandReply::GetValuesSelective(pat!(Values {
            avg_current_input: approx_eq(0.0),
            rpm: approx_eq(0.0),
            voltage_in: approx_eq(38.4),
            tachometer: eq(-25018),
            fault_code: eq(0),
            controller_id: eq(1),
            ..
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_get_values_selective_forward_rpm() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119, 255, 255, 170, 43, 0,
        20, 45, 58, 3,
    ];

    let expected = (
        eq(&28),
        pat!(&CommandReply::GetValuesSelective(pat!(Values {
            avg_current_input: approx_eq(0.04),
            rpm: approx_eq(989.0),
            voltage_in: approx_eq(37.5),
            tachometer: eq(-21973),
            fault_code: eq(0),
            controller_id: eq(20),
            ..
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_get_values_selective_reverse_rpm() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 20, 94, 130, 3,
    ];

    let expected = (
        eq(&28),
        pat!(&CommandReply::GetValuesSelective(pat!(Values {
            avg_current_input: approx_eq(0.1),
            rpm: approx_eq(-2347.0),
            voltage_in: approx_eq(37.4),
            tachometer: eq(-18982),
            fault_code: eq(0),
            controller_id: eq(20),
            ..
        }))),
    );
    assert_that!(vesc::decode(&input), ok(expected));
}

#[test]
fn decode_incomplete_data() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];
    let expected = &DecodeError::IncompleteData;

    for i in 1..input.len() {
        assert_that!(vesc::decode(&input[..i]), err(eq(expected)));
    }
}

#[test]
fn decode_checksum_mismatch() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 21, 94, 130, 3,
    ];

    let expected = &DecodeError::ChecksumMismatch {
        expected: 24194,
        actual: 20131,
    };
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_unknown_packet() {
    let input = [2, 3, 222, 4, 0, 178, 81, 3];
    let expected = &DecodeError::UnknownPacket { id: 222 };
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_invalid_frame_start() {
    let input = [
        7, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];
    let expected = &DecodeError::InvalidFrame;
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_invalid_frame_end() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 2,
    ];
    let expected = &DecodeError::InvalidFrame;
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_wrong_payload_len_gt_payload() {
    let mut input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];
    input[1] = input[1] + 1;

    let expected = &DecodeError::InvalidFrame;
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_wrong_payload_len_lt_payload() {
    let mut input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];
    input[1] = input[1] - 1;

    let expected = &DecodeError::InvalidFrame;
    assert_that!(vesc::decode(&input), err(eq(expected)));
}

#[test]
fn decode_invalid_frame_end_witch_checksum_mismatch() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 21, 94, 130, 2,
    ];

    let expected = &DecodeError::InvalidFrame;
    assert_that!(vesc::decode(&input), err(eq(expected)));
}
