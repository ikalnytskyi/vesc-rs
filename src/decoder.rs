use crate::{CommandReply, DecodeError};

/// A streaming decoder for VESC communication protocol.
///
/// The `Decoder` maintains an internal buffer that accumulates incoming data
/// and extracts complete protocol frames as they become available. It
/// automatically handles frame synchronization, partial data, and buffer
/// management.
///
/// The decoder accepts data via [`feed`] and yields decoded frames through
/// the [`Iterator`] interface.
///
/// [`feed`]: Self::feed
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Decoder<const BUFLEN: usize = 512> {
    buf: [u8; BUFLEN],
    wpos: usize,
    rpos: usize,
}

impl<const BUFLEN: usize> Default for Decoder<BUFLEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BUFLEN: usize> Decoder<BUFLEN> {
    /// Creates a new decoder with an empty internal buffer.
    pub fn new() -> Self {
        Self {
            buf: [0; BUFLEN],
            rpos: 0,
            wpos: 0,
        }
    }

    /// Feeds new data into the decoder's internal buffer.
    ///
    /// Returns the number of bytes consumed from the input. If less than the
    /// input length is consumed, the remaining bytes should be re-fed in the
    /// next call.
    ///
    /// The decoder automatically manages buffer space by compacting processed
    /// data and will reset if a single frame exceeds buffer capacity.
    pub fn feed(&mut self, data: &[u8]) -> Result<usize, DecodeError> {
        if data.len() > self.buf.len().saturating_sub(self.wpos) {
            self.buf.copy_within(self.rpos..self.wpos, 0);
            self.wpos = self.wpos.saturating_sub(self.rpos);
            self.rpos = self.rpos.saturating_sub(self.rpos);
        }

        // Reset to prevent indefinite blocking if space reclamation cannot
        // help. This occurs when a single VESC message exceed buffer capacity.
        // Must never happen under normal circumstances unless internal buffer
        // length is decreased down to inadequate value or modified VESC
        // firmware is used.
        if self.wpos == self.buf.len() {
            self.rpos = 0;
            self.wpos = 0;
        }

        let copied = data.len().min(self.buf.len().saturating_sub(self.wpos));
        self.buf[self.wpos..self.wpos + copied].copy_from_slice(&data[..copied]);
        self.wpos += copied;
        Ok(copied)
    }
}

impl<const BUFLEN: usize> core::iter::Iterator for Decoder<BUFLEN> {
    type Item = CommandReply;

    /// Attempts to decode the next complete frame from the internal buffer.
    ///
    /// Returns `Some(CommandReply)` if a complete frame is available, or `None`
    /// if more data is needed. Automatically handles frame synchronization by
    /// skipping corrupted data.
    fn next(&mut self) -> Option<Self::Item> {
        while self.rpos < self.wpos {
            match crate::decode(&self.buf[self.rpos..self.wpos]) {
                Ok((consumed, reply)) => {
                    self.rpos += consumed;
                    return Some(reply);
                }
                Err(DecodeError::IncompleteData) => return None,
                _ => (),
            }
            self.rpos += 1;
        }
        None
    }
}
