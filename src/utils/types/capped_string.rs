use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

use super::strcat::read_leftover;

#[derive(Clone)]
pub struct CappedString<const N: usize> {
    pub(in crate::utils::types) buffer: [u8; N],
    pub(in crate::utils::types) used: usize,
    pub(in crate::utils::types) truncated: bool,
    pub(in crate::utils::types) leftover: usize,
}

impl<const N: usize> CappedString<N> {
    pub fn new() -> Self {
        CappedString {
            buffer: [0u8; N],
            used: 0,
            leftover: 0,
            truncated: false,
        }
    }

    pub fn as_str(&mut self) -> &str {
        for byte in self.buffer[..self.used].iter_mut() {
            // NOTE: this workaround is needed until https://github.com/LedgerHQ/ledger-device-rust-sdk/issues/146
            // is handled at sdk level
            if *byte < 0x20 {
                // NOTE: this is a square glyph, of DEL display
                *byte = 0x7f;
            }
            // NOTE: this workaround is needed until https://github.com/LedgerHQ/ledger-device-rust-sdk/issues/124
            // is handled at sdk level
            if *byte > 0x7f {
                // NOTE: this is a square glyph, of DEL display
                *byte = 0x7f;
            }
        }
        // .unwrap() is ok because it's either based on complete deserialized `str`
        // based on previous validation by `core::str::from_utf8`,
        // or `self.used` index is equal to value of [`Utf8Error::valid_up_to()`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html#method.valid_up_to)
        // or it's equal to 0 after `CappedString::new()` was called
        core::str::from_utf8(&self.buffer[..self.used]).unwrap()
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn leftover(&self) -> usize {
        self.leftover
    }
}

impl<const N: usize> CappedString<N> {
    // BorshDeserialize counterpart to reduce size of stack frames
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;

        self.deserialize_with_bytes_count(reader, bytes_count)
    }

    pub fn deserialize_with_bytes_count<R: Read>(
        &mut self,
        reader: &mut R,
        bytes_count: u32,
    ) -> Result<()> {
        let truncated = bytes_count > (N as u32);
        self.used = 0;
        self.truncated = truncated;

        if !truncated {
            reader.read_exact(&mut self.buffer[0..(bytes_count as usize)])?;
            self.used = bytes_count as usize;

            // the whole string is expected to be correct
            core::str::from_utf8(&self.buffer[0..(bytes_count as usize)])
                .map_err(|_err| Error::from(ErrorKind::InvalidData))?;
        } else {
            let leftover = (bytes_count as usize) - self.buffer.len();
            reader.read_exact(&mut self.buffer)?;

            self.used = self.buffer.len();
            self.leftover = leftover;
            let postpone_error = match core::str::from_utf8(&self.buffer) {
                Ok(_result) => false,
                Err(err) => {
                    if err.error_len().is_some() {
                        true
                    } else {
                        let valid_utf8_up_to = err.valid_up_to();
                        self.used = valid_utf8_up_to;
                        false
                    }
                }
            };

            if leftover > 0 {
                read_leftover(leftover, reader)?;
            }

            if postpone_error {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }

        Ok(())
    }
}
