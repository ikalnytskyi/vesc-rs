use googletest::prelude::*;
use vesc::{
    AmpHours, AvgCurrent, AvgVoltage, CommandReply, Decoder, DutyCycle, FaultCode, Position,
    Temperature, Values, Voltage, WattHours,
};

#[test]
fn decoder_decodes_single_packet() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119, 255, 255, 170, 43, 0,
        20, 45, 58, 3,
    ];

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(4)),
        rpm: eq(989),
        voltage_in: eq(Voltage::from_raw(375)),
        tachometer: eq(-21973),
        fault_code: eq(FaultCode::None),
        controller_id: eq(20),
        ..
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_decodes_packet_fed_in_chunks() {
    let input = [
        2, 74, 4, 1, 20, 0, 0, 0, 0, 0, 37, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 32, 0, 110, 0, 0, 3,
        251, 1, 125, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 2, 137, 0, 0, 0, 0, 255, 255, 111, 75, 0, 2,
        159, 199, 0, 4, 106, 124, 40, 1, 1, 21, 252, 76, 252, 13, 0, 0, 0, 229, 0, 0, 8, 214, 0,
        58, 151, 3,
    ];
    let mut decoder = Decoder::default();

    for chunk in input.chunks(5) {
        decoder.feed(&chunk).unwrap();
    }

    let expected = pat!(CommandReply::GetValues(pat!(Values {
        temp_mosfet: eq(Temperature::from_raw(276)),
        temp_motor: eq(Temperature::from_raw(0)),
        avg_current_motor: eq(AvgCurrent::from_raw(37)),
        avg_current_input: eq(AvgCurrent::from_raw(3)),
        avg_current_d: eq(AvgCurrent::from_raw(0)),
        avg_current_q: eq(AvgCurrent::from_raw(32)),
        duty_cycle: eq(DutyCycle::from_raw(110)),
        rpm: eq(1019),
        voltage_in: eq(Voltage::from_raw(381)),
        amp_hours: eq(AmpHours::from_raw(17)),
        amp_hours_charged: eq(AmpHours::from_raw(0)),
        watt_hours: eq(WattHours::from_raw(649)),
        watt_hours_charged: eq(WattHours::from_raw(0)),
        tachometer: eq(-37045),
        tachometer_abs: eq(171975),
        fault_code: eq(FaultCode::None),
        pid_pos: eq(Position::from_raw(74087464)),
        controller_id: eq(1),
        temp_mosfet1: eq(Temperature::from_raw(277)),
        temp_mosfet2: eq(Temperature::from_raw(-948)),
        temp_mosfet3: eq(Temperature::from_raw(-1011)),
        avg_voltage_d: eq(AvgVoltage::from_raw(229)),
        avg_voltage_q: eq(AvgVoltage::from_raw(2262)),
        status: eq(0),
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_returns_none_until_packet_is_complete() {
    let input = [
        2, 74, 4, 1, 20, 0, 0, 0, 0, 0, 37, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 32, 0, 110, 0, 0, 3,
        251, 1, 125, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 2, 137, 0, 0, 0, 0, 255, 255, 111, 75, 0, 2,
        159, 199, 0, 4, 106, 124, 40, 1, 1, 21, 252, 76, 252, 13, 0, 0, 0, 229, 0, 0, 8, 214, 0,
        58, 151, 3,
    ];
    let mut decoder = Decoder::default();

    for (i, chunk) in input.chunks(input.len() / 5).enumerate() {
        decoder.feed(&chunk).unwrap();
        if i < 5 {
            assert_that!(decoder.next(), none());
        }
    }

    let expected = pat!(CommandReply::GetValues(pat!(Values {
        temp_mosfet: eq(Temperature::from_raw(276)),
        temp_motor: eq(Temperature::from_raw(0)),
        avg_current_motor: eq(AvgCurrent::from_raw(37)),
        avg_current_input: eq(AvgCurrent::from_raw(3)),
        avg_current_d: eq(AvgCurrent::from_raw(0)),
        avg_current_q: eq(AvgCurrent::from_raw(32)),
        duty_cycle: eq(DutyCycle::from_raw(110)),
        rpm: eq(1019),
        voltage_in: eq(Voltage::from_raw(381)),
        amp_hours: eq(AmpHours::from_raw(17)),
        amp_hours_charged: eq(AmpHours::from_raw(0)),
        watt_hours: eq(WattHours::from_raw(649)),
        watt_hours_charged: eq(WattHours::from_raw(0)),
        tachometer: eq(-37045),
        tachometer_abs: eq(171975),
        fault_code: eq(FaultCode::None),
        pid_pos: eq(Position::from_raw(74087464)),
        controller_id: eq(1),
        temp_mosfet1: eq(Temperature::from_raw(277)),
        temp_mosfet2: eq(Temperature::from_raw(-948)),
        temp_mosfet3: eq(Temperature::from_raw(-1011)),
        avg_voltage_d: eq(AvgVoltage::from_raw(229)),
        avg_voltage_q: eq(AvgVoltage::from_raw(2262)),
        status: eq(0),
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_decodes_two_packets_from_single_feed() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3, 2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119, 255, 255,
        170, 43, 0, 20, 45, 58, 3,
    ];
    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(0)),
        rpm: eq(0),
        voltage_in: eq(Voltage::from_raw(384)),
        tachometer: eq(-25018),
        fault_code: eq(FaultCode::None),
        controller_id: eq(1),
        ..
    })));
    assert_that!(decoder.next(), some(expected));

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(4)),
        rpm: eq(989),
        voltage_in: eq(Voltage::from_raw(375)),
        tachometer: eq(-21973),
        fault_code: eq(FaultCode::None),
        controller_id: eq(20),
        ..
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_decodes_two_packets_from_separate_feeds() {
    let input_one = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158, 70, 0, 1,
        63, 148, 3,
    ];
    let input_two = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119, 255, 255, 170, 43, 0,
        20, 45, 58, 3,
    ];

    let mut decoder = Decoder::default();
    decoder.feed(&input_one).unwrap();
    decoder.feed(&input_two).unwrap();

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(0)),
        rpm: eq(0),
        voltage_in: eq(Voltage::from_raw(384)),
        tachometer: eq(-25018),
        fault_code: eq(FaultCode::None),
        controller_id: eq(1),
        ..
    })));
    assert_that!(decoder.next(), some(expected));

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(4)),
        rpm: eq(989),
        voltage_in: eq(Voltage::from_raw(375)),
        tachometer: eq(-21973),
        fault_code: eq(FaultCode::None),
        controller_id: eq(20),
        ..
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_ignores_incomplete_packet() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119, 255, 255, 170, 43, 0,
        20, 45, 58,
    ];

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();
    assert_that!(decoder.next(), none());
}

#[test]
fn decoder_ignores_packet_with_bad_checksum() {
    let input = [
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 21, 94, 130, 3,
    ];

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();
    assert_that!(decoder.next(), none());
}

#[test]
fn decoder_ignores_unknown_packet_id() {
    let input = [2, 3, 222, 4, 0, 178, 81, 3];

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();
    assert_that!(decoder.next(), none());
}

#[test]
fn decoder_skips_junk_bytes_between_packets() {
    let input = [
        10, 34, 12, 2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 128, 255, 255, 158,
        70, 0, 1, 63, 148, 3, 4, 178, 255, 2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3,
        221, 1, 119, 255, 255, 170, 43, 0, 20, 45, 58, 3,
    ];
    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(0)),
        rpm: eq(0),
        voltage_in: eq(Voltage::from_raw(384)),
        tachometer: eq(-25018),
        fault_code: eq(FaultCode::None),
        controller_id: eq(1),
        ..
    })));
    assert_that!(decoder.next(), some(expected));

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(4)),
        rpm: eq(989),
        voltage_in: eq(Voltage::from_raw(375)),
        tachometer: eq(-21973),
        fault_code: eq(FaultCode::None),
        controller_id: eq(20),
        ..
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_recovers_from_false_start_byte() {
    let input = [
        2, 5, 50, 0, 0, 0, 1, 2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 4, 0, 0, 3, 221, 1, 119,
        255, 255, 170, 43, 0, 20, 45, 58, 3,
    ];

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();

    let expected = pat!(CommandReply::GetValuesSelective(pat!(Values {
        avg_current_input: eq(AvgCurrent::from_raw(4)),
        rpm: eq(989),
        voltage_in: eq(Voltage::from_raw(375)),
        tachometer: eq(-21973),
        fault_code: eq(FaultCode::None),
        controller_id: eq(20),
        ..
    })));
    assert_that!(decoder.next(), some(expected));
}

#[test]
fn decoder_iterator_collects_all_valid_packets() {
    let mut input = vec![];

    // Junk
    input.extend_from_slice(&[2, 45, 4]);

    // GetValues
    input.extend_from_slice(&[
        2, 74, 4, 1, 20, 0, 0, 0, 0, 0, 37, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 32, 0, 110, 0, 0, 3,
        251, 1, 125, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 2, 137, 0, 0, 0, 0, 255, 255, 111, 75, 0, 2,
        159, 199, 0, 4, 106, 124, 40, 1, 1, 21, 252, 76, 252, 13, 0, 0, 0, 229, 0, 0, 8, 214, 0,
        58, 151, 3,
    ]);

    // GetValuesSelective (bad checksum)
    input.extend_from_slice(&[
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 21, 94, 130, 3,
    ]);

    // GetValuesSelective
    input.extend_from_slice(&[
        2, 23, 50, 0, 2, 161, 138, 0, 0, 0, 0, 0, 10, 255, 255, 246, 213, 1, 118, 255, 255, 181,
        218, 0, 20, 94, 130, 3,
    ]);

    // GetValuesSelective (unknown packet)
    input.extend_from_slice(&[2, 3, 222, 4, 0, 178, 81, 3]);

    let mut decoder = Decoder::default();
    decoder.feed(&input).unwrap();

    let expected = elements_are![
        pat!(&CommandReply::GetValues(pat!(Values {
            temp_mosfet: eq(Temperature::from_raw(276)),
            temp_motor: eq(Temperature::from_raw(0)),
            avg_current_motor: eq(AvgCurrent::from_raw(37)),
            avg_current_input: eq(AvgCurrent::from_raw(3)),
            avg_current_d: eq(AvgCurrent::from_raw(0)),
            avg_current_q: eq(AvgCurrent::from_raw(32)),
            duty_cycle: eq(DutyCycle::from_raw(110)),
            rpm: eq(1019),
            voltage_in: eq(Voltage::from_raw(381)),
            amp_hours: eq(AmpHours::from_raw(17)),
            amp_hours_charged: eq(AmpHours::from_raw(0)),
            watt_hours: eq(WattHours::from_raw(649)),
            watt_hours_charged: eq(WattHours::from_raw(0)),
            tachometer: eq(-37045),
            tachometer_abs: eq(171975),
            fault_code: eq(FaultCode::None),
            pid_pos: eq(Position::from_raw(74087464)),
            controller_id: eq(1),
            temp_mosfet1: eq(Temperature::from_raw(277)),
            temp_mosfet2: eq(Temperature::from_raw(-948)),
            temp_mosfet3: eq(Temperature::from_raw(-1011)),
            avg_voltage_d: eq(AvgVoltage::from_raw(229)),
            avg_voltage_q: eq(AvgVoltage::from_raw(2262)),
            status: eq(0),
        }))),
        pat!(&CommandReply::GetValuesSelective(pat!(Values {
            avg_current_input: eq(AvgCurrent::from_raw(10)),
            rpm: eq(-2347),
            voltage_in: eq(Voltage::from_raw(374)),
            tachometer: eq(-18982),
            fault_code: eq(FaultCode::None),
            controller_id: eq(20),
            ..
        }))),
    ];

    let replies = decoder.by_ref().collect::<Vec<_>>();
    assert_that!(replies, expected);
}
