use googletest::prelude::*;
use vesc::{Command, CommandReply, DecodeError, Decoder};

const CONFIG: &[u8] = include_bytes!("fixtures/mcconf-6.05.bin");
const CONFIGS: &[&[u8]] = &[
    CONFIG,
    include_bytes!("fixtures/mcconf-6.06.bin"),
    include_bytes!("fixtures/mcconf-7.00.bin"),
    include_bytes!("fixtures/mcconf-8.00.bin"),
];

fn frame(config: &[u8]) -> Vec<u8> {
    let mut payload = vec![14];
    payload.extend_from_slice(config);
    let mut frame = vec![3];
    frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    frame.extend_from_slice(&payload);
    frame.extend_from_slice(
        &crc::Crc::<u16>::new(&crc::CRC_16_XMODEM)
            .checksum(&payload)
            .to_be_bytes(),
    );
    frame.push(3);
    frame
}

#[test]
fn encode_get_mcconf() {
    let mut buf = [0; 6];
    let size = vesc::encode(Command::GetMcConf, &mut buf).unwrap();
    assert_that!(buf[..size], eq([2, 1, 14, 225, 206, 3]));
}

#[test]
fn decode_mcconf_known_layouts() {
    for config_bytes in CONFIGS {
        let input = frame(config_bytes);
        let (consumed, CommandReply::GetMcConf(config)) = vesc::decode(&input).unwrap() else {
            panic!("expected motor configuration");
        };
        assert_that!(consumed, eq(config_bytes.len() + 7));
        assert_that!(config.l_current_max, eq(178.75));
        assert_that!(config.l_abs_current_max, eq(268.125));
        assert_that!(config.l_max_duty, eq(0.95));
        assert_that!(config.l_current_max_scale, eq(0.8));
        assert_that!(config.foc_fw_current_max, eq(100.5));
        assert_that!(config.foc_fw_duty_start, eq(0.9));
    }
}

#[test]
fn decode_mcconf_rejects_unknown_signature() {
    let mut config = CONFIG.to_vec();
    config[..4].copy_from_slice(&0x12345678u32.to_be_bytes());
    assert_that!(
        vesc::decode(&frame(&config)),
        err(eq(&DecodeError::UnsupportedMcConf {
            signature: 0x12345678
        }))
    );
}

#[test]
fn decode_mcconf_rejects_wrong_lengths() {
    for config_bytes in CONFIGS {
        for length in 0..config_bytes.len() {
            assert!(vesc::decode(&frame(&config_bytes[..length])).is_err());
        }
        let mut config = config_bytes.to_vec();
        config.push(0);
        assert_that!(
            vesc::decode(&frame(&config)),
            err(eq(&DecodeError::InvalidFrame))
        );
    }
}

#[test]
fn decode_long_frame_checks_truncation_crc_and_terminator() {
    let input = frame(CONFIG);
    for length in 0..input.len() {
        assert_that!(
            vesc::decode(&input[..length]),
            err(eq(&DecodeError::IncompleteData))
        );
    }
    let mut corrupt = input.clone();
    corrupt[20] ^= 1;
    assert!(matches!(
        vesc::decode(&corrupt),
        Err(DecodeError::ChecksumMismatch { .. })
    ));
    let last = corrupt.len() - 1;
    corrupt[last] = 0;
    assert_that!(vesc::decode(&corrupt), err(eq(&DecodeError::InvalidFrame)));
}

#[test]
fn default_stream_decoder_accepts_fragmented_configuration_and_next_reply() {
    for config_bytes in CONFIGS {
        let mut input = frame(config_bytes);
        input.extend_from_slice(&[2, 1, 129, 129, 169, 3]);
        let mut decoder = Decoder::default();
        let mut replies = Vec::new();
        for chunk in input.chunks(7) {
            assert_that!(decoder.feed(chunk).unwrap(), eq(chunk.len()));
            replies.extend(decoder.by_ref());
        }
        assert_that!(replies.len(), eq(2));
        assert!(matches!(replies[0], CommandReply::GetMcConf(_)));
        assert!(matches!(replies[1], CommandReply::ResetStats));
    }
}

#[test]
fn mcconf_fits_inside_existing_reply_storage() {
    assert!(core::mem::size_of::<vesc::McConf>() <= core::mem::size_of::<vesc::Values>());
}

#[path = "mcconf/groups.rs"]
mod groups;
