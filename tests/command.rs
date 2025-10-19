use googletest::prelude::*;

use vesc::{self, Command, EncodeError, ValuesMask};

#[test]
fn encode_get_values() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::GetValues, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 4, 64, 132, 3]));
}

#[test]
fn encode_set_current() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetCurrent(0.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 6, 0, 0, 0, 0, 205, 133, 3]));

    let size = vesc::encode(Command::SetCurrent(1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 6, 0, 0, 3, 232, 228, 240, 3]));

    let size = vesc::encode(Command::SetCurrent(57.123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 6, 0, 0, 223, 35, 220, 157, 3]));

    let size = vesc::encode(Command::SetCurrent(-1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 6, 255, 255, 252, 24, 140, 208, 3]));

    let size = vesc::encode(Command::SetCurrent(-57.123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 6, 255, 255, 32, 221, 85, 115, 3]));
}

#[test]
fn encode_set_rpm() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetRpm(0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 8, 0, 0, 0, 0, 2, 45, 3]));

    let size = vesc::encode(Command::SetRpm(1), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 8, 0, 0, 0, 1, 18, 12, 3]));

    let size = vesc::encode(Command::SetRpm(1234), &mut buf).unwrap();
    assert_that!(buf[..size], eq(&[2, 5, 8, 0, 0, 4, 210, 37, 214, 3]));

    let size = vesc::encode(Command::SetRpm(-1), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 8, 255, 255, 255, 255, 155, 226, 3]));

    let size = vesc::encode(Command::SetRpm(-1234), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 8, 255, 255, 251, 46, 140, 122, 3]));
}

#[test]
fn encode_set_handbrake() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetHandbrake(0.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 10, 0, 0, 0, 0, 70, 174, 3]));

    let size = vesc::encode(Command::SetHandbrake(1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 10, 0, 0, 3, 232, 111, 219, 3]));

    let size = vesc::encode(Command::SetHandbrake(5.2), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 10, 0, 0, 20, 80, 211, 236, 3]));

    let size = vesc::encode(Command::SetHandbrake(-1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 10, 255, 255, 252, 24, 7, 251, 3]));

    let size = vesc::encode(Command::SetHandbrake(-5.2), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 10, 255, 255, 235, 176, 169, 253, 3]));
}

#[test]
fn encode_forward_can() {
    let mut buf = [0u8; 16];

    let command = Command::ForwardCan(1, &Command::SetRpm(1234));
    let size = vesc::encode(command, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 7, 34, 1, 8, 0, 0, 4, 210, 110, 99, 3]));

    let command = Command::ForwardCan(7, &Command::SetCurrent(57.123));
    let size = vesc::encode(command, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 7, 34, 7, 6, 0, 0, 223, 35, 26, 201, 3]));
}

#[test]
fn encode_get_values_selective() {
    let mut buf = [0u8; 16];

    let mask = ValuesMask::TEMP_MOSFET;
    let size = vesc::encode(Command::GetValuesSelective(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 50, 0, 0, 0, 1, 88, 76, 3]));

    let mask = ValuesMask::VOLTAGE_IN;
    let size = vesc::encode(Command::GetValuesSelective(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 50, 0, 0, 1, 0, 123, 92, 3]));

    let mask = ValuesMask::TEMP_MOSFET | ValuesMask::VOLTAGE_IN;
    let size = vesc::encode(Command::GetValuesSelective(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 50, 0, 0, 1, 1, 107, 125, 3]));

    let mask = ValuesMask::RPM | ValuesMask::WATT_HOURS | ValuesMask::CONTROLLER_ID;
    let size = vesc::encode(Command::GetValuesSelective(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 50, 0, 2, 8, 128, 62, 44, 3]));
}

#[test]
fn encode_buf_perfect_fit() {
    let mut buf = [0u8; 10];

    let size = vesc::encode(Command::SetRpm(0), &mut buf).unwrap();
    assert_that!(size, eq(buf.len()));
    assert_that!(buf[..size], eq([2, 5, 8, 0, 0, 0, 0, 2, 45, 3]));
}

#[test]
fn encode_buffer_too_small() {
    for n in 0..10 {
        let mut buf = vec![0u8; n];
        let result = vesc::encode(Command::SetRpm(0), &mut buf);
        assert_that!(result, err(eq(&EncodeError::BufferTooSmall)));
    }
}
