//! The VESC® firmware is an open source motor controller firmware, and this
//! library implements the necessary structures and functions to [`encode`]
//! commands and [`decode`] replies.
//!
//! # Examples
//!
//! ## Encoding a Command
//!
//! ```no_run
//! use vesc::{Command, ValuesMask};
//!
//! let mut buf = [0u8; 64];
//! let command = Command::GetValuesSelective(ValuesMask::RPM | ValuesMask::VOLTAGE_IN);
//! let frame_len = vesc::encode(command, &mut buf).unwrap();
//! let frame = &buf[..frame_len];
//! ```
//!
//! ## Decoding a Reply
//!
//! ```no_run
//! use vesc::CommandReply;
//!
//! match vesc::decode(&[2, 7, 50, 0, 0, 1, 128, 0, 0, 4, 210, 1, 176, 254, 22, 3]) {
//!     Ok((_, CommandReply::GetValuesSelective(values))) => {
//!         let rpm = values.rpm;
//!         let voltage_in = values.voltage_in;
//!     },
//!     _ => (),
//! }
//! ```
//!
//! ## Decoding a Stream
//!
//! ```no_run
//! use vesc::{Decoder, CommandReply};
//!
//! let mut decoder = Decoder::default();
//! decoder.feed(&[2, 7, 50, 0, 0, 1, 128, 0, 0, 4, 210, 1, 176, 254, 22, 3]);
//!
//! for reply in decoder.by_ref() {
//!     match reply {
//!         CommandReply::GetValuesSelective(values) => {
//!             let rpm = values.rpm;
//!             let voltage_in = values.voltage_in;
//!         },
//!         _ => (),
//!     }
//! }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

mod command;
mod decoder;
mod packer;

pub use command::{
    //
    Command,
    CommandReply,
    DecodeError,
    EncodeError,
    FaultCode,
    FwInfo,
    FwVersion,
    HwType,
    NrfFlags,
    QmlAppFlags,
    QmlHw,
    SetupValues,
    Stats,
    StatsMask,
    Values,
    ValuesMask,
    ValuesSetupMask,
    decode,
    encode,
};
pub use decoder::Decoder;
