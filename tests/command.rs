use googletest::prelude::*;

use vesc::{self, Command, EncodeError, StatsMask, ValuesMask, ValuesSetupMask};

#[test]
fn encode_fw_version() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::FwVersion, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 0, 0, 0, 3]));
}

#[test]
fn encode_fw_info() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::FwInfo, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 157, 82, 20, 3]));
}

#[test]
fn encode_get_values() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::GetValues, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 4, 64, 132, 3]));
}

#[test]
fn encode_set_duty() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetDuty(0.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 5, 0, 0, 0, 0, 35, 87, 3]));

    let size = vesc::encode(Command::SetDuty(0.1), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 5, 0, 0, 39, 16, 174, 23, 3]));

    let size = vesc::encode(Command::SetDuty(0.57123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 5, 0, 0, 223, 35, 50, 79, 3]));

    let size = vesc::encode(Command::SetDuty(-0.1), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 5, 255, 255, 216, 240, 212, 6, 3]));

    let size = vesc::encode(Command::SetDuty(-0.57123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 5, 255, 255, 32, 221, 187, 161, 3]));
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
fn encode_set_current_brake() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetCurrentBrake(0.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 7, 0, 0, 0, 0, 103, 212, 3]));

    let size = vesc::encode(Command::SetCurrentBrake(1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 7, 0, 0, 3, 232, 78, 161, 3]));

    let size = vesc::encode(Command::SetCurrentBrake(57.123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 7, 0, 0, 223, 35, 118, 204, 3]));

    let size = vesc::encode(Command::SetCurrentBrake(-1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 7, 255, 255, 252, 24, 38, 129, 3]));

    let size = vesc::encode(Command::SetCurrentBrake(-57.123), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 7, 255, 255, 32, 221, 255, 34, 3]));
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
fn encode_set_pos() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetPos(0.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 9, 0, 0, 0, 0, 168, 124, 3]));

    let size = vesc::encode(Command::SetPos(1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 9, 0, 15, 66, 64, 167, 39, 3]));

    let size = vesc::encode(Command::SetPos(0.1234), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 9, 0, 1, 226, 8, 104, 148, 3]));

    let size = vesc::encode(Command::SetPos(-1.0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 9, 255, 240, 189, 192, 177, 144, 3]));

    let size = vesc::encode(Command::SetPos(-0.1234), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 9, 255, 254, 29, 248, 0, 180, 3]));
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
fn encode_alive() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::Alive, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 30, 243, 255, 3]));
}

#[test]
fn encode_reboot() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::Reboot, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 29, 195, 156, 3]));
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
fn encode_get_values_setup_selective() {
    let mut buf = [0u8; 16];

    let mask = ValuesSetupMask::ODOMETER;
    let size = vesc::encode(Command::GetValuesSetupSelective(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 51, 0, 16, 0, 0, 161, 95, 3]));
}

#[test]
fn encode_set_odometer() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::SetOdometer(0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 110, 0, 0, 0, 0, 214, 116, 3]));

    let size = vesc::encode(Command::SetOdometer(1), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 110, 0, 0, 0, 1, 198, 85, 3]));

    let size = vesc::encode(Command::SetOdometer(123456), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 110, 0, 1, 226, 64, 223, 80, 3]));

    let size = vesc::encode(Command::SetOdometer(u32::MAX), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 5, 110, 255, 255, 255, 255, 79, 187, 3]));
}

#[test]
fn encode_get_stats() {
    let mut buf = [0u8; 16];

    let mask = StatsMask::SPEED_AVG;
    let size = vesc::encode(Command::GetStats(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 128, 0, 1, 43, 123, 3]));

    let mask = StatsMask::COUNT_TIME;
    let size = vesc::encode(Command::GetStats(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 128, 4, 0, 247, 158, 3]));

    let mask = StatsMask::SPEED_AVG | StatsMask::TEMP_MOTOR_MAX | StatsMask::COUNT_TIME;
    let size = vesc::encode(Command::GetStats(mask), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 128, 6, 1, 129, 221, 3]));
}

#[test]
fn encode_reset_stats() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::ResetStats(false), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 2, 129, 0, 40, 169, 3]));

    let size = vesc::encode(Command::ResetStats(true), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 2, 129, 1, 56, 136, 3]));
}

#[test]
fn encode_shutdown() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::Shutdown(false, false), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 156, 0, 0, 13, 88, 3]));

    let size = vesc::encode(Command::Shutdown(true, false), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 156, 1, 0, 62, 105, 3]));

    let size = vesc::encode(Command::Shutdown(true, true), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 156, 1, 1, 46, 72, 3]));
}

#[test]
fn encode_motor_estop() {
    let mut buf = [0u8; 16];

    let size = vesc::encode(Command::MotorEstop(0), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 159, 0, 0, 84, 8, 3]));

    let size = vesc::encode(Command::MotorEstop(1000), &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 3, 159, 3, 232, 125, 125, 3]));
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
