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

    /// `true` when every stored byte is printable ASCII (0x20..=0x7e), i.e. when
    /// `as_str` shows each byte as its own glyph instead of the shared 0x7f square.
    ///
    /// Must be called before `as_str`, which rewrites the other bytes in place.
    pub fn is_printable_ascii(&self) -> bool {
        self.buffer[..self.used]
            .iter()
            .all(|byte| (0x20..=0x7e).contains(byte))
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

    /// Same as [`Self::deserialize_reader_in_place`], but a string longer than the buffer
    /// is an `InvalidData` error instead of being kept truncated.
    pub fn deserialize_reader_in_place_whole<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.deserialize_reader_in_place(reader)?;
        if self.truncated {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::CappedString;
    use borsh::io::ErrorKind;

    /// Borsh string of `len` bytes of `b'a'`, in a buffer large enough for 65 bytes.
    fn borsh_string(len: usize) -> ([u8; 4 + 65], usize) {
        let mut buf = [b'a'; 4 + 65];
        buf[..4].copy_from_slice(&(len as u32).to_le_bytes());
        (buf, 4 + len)
    }

    #[test_case]
    fn whole_accepts_string_that_fits() {
        let (buf, len) = borsh_string(64);
        let mut reader = &buf[..len];
        let mut s: CappedString<64> = CappedString::new();
        assert!(s.deserialize_reader_in_place_whole(&mut reader).is_ok());
        assert_eq!(s.as_str().len(), 64);
        assert!(reader.is_empty());
    }

    #[test_case]
    fn whole_rejects_string_one_byte_too_long() {
        let (buf, len) = borsh_string(65);
        let mut reader = &buf[..len];
        let mut s: CappedString<64> = CappedString::new();
        let err = s
            .deserialize_reader_in_place_whole(&mut reader)
            .unwrap_err();
        assert!(err.kind() == ErrorKind::InvalidData);
        // the whole string is consumed, so it is still covered by the hash
        assert!(reader.is_empty());
    }

    fn capped(bytes: &[u8]) -> CappedString<8> {
        let mut s: CappedString<8> = CappedString::new();
        assert!(s
            .deserialize_with_bytes_count(&mut &bytes[..], bytes.len() as u32)
            .is_ok());
        s
    }

    #[test_case]
    fn printable_ascii_accepts_space_to_tilde() {
        assert!(capped(b" az~{}").is_printable_ascii());
        assert!(capped(b"").is_printable_ascii());
    }

    #[test_case]
    fn printable_ascii_rejects_bytes_drawn_as_the_square_glyph() {
        assert!(!capped(b"a\nb").is_printable_ascii());
        assert!(!capped(b"a\x1fb").is_printable_ascii());
        assert!(!capped(b"a\x7fb").is_printable_ascii());
        assert!(!capped("caf\u{e9}".as_bytes()).is_printable_ascii());
    }

    #[test_case]
    fn in_place_keeps_string_one_byte_too_long_as_truncated() {
        let (buf, len) = borsh_string(65);
        let mut reader = &buf[..len];
        let mut s: CappedString<64> = CappedString::new();
        assert!(s.deserialize_reader_in_place(&mut reader).is_ok());
        assert!(s.truncated());
        assert_eq!(s.leftover(), 1);
    }
}
