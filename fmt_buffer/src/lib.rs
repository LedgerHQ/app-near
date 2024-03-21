#![no_std]
pub struct Buffer<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
    leftover: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        Buffer {
            buffer: [0u8; N],
            used: 0,
            truncated: false,
            leftover: 0,
        }
    }

    pub fn as_str(&mut self) -> &str {
        for byte in self.buffer[..self.used].iter_mut() {
            // NOTE: this workaround is needed until https://github.com/LedgerHQ/ledger-device-rust-sdk/issues/146
            // is handled at sdk level
            if *byte < 0x20 {
                *byte = 0x7f;
            }
            // NOTE: this workaround is needed until https://github.com/LedgerHQ/ledger-device-rust-sdk/issues/124
            // is handled at sdk level
            if *byte > 0x7f {
                // NOTE: this is a square glyph, of DEL display
                *byte = 0x7f;
            }
        }
        debug_assert!(self.used <= self.buffer.len());
        // .unwrap() is ok, as only bytes, comprising a sequence of valid utf8 chars
        // are going to be written to `self.buffer` on `self.write_str` calls
        core::str::from_utf8(&self.buffer[..self.used]).unwrap()
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn leftover(&self) -> usize {
        self.leftover
    }
}

impl<const N: usize> Buffer<N> {
    pub fn write_str(&mut self, s: &str) {
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();

        let raw_s_len = raw_s.len();
        let remaining_len = remaining_buf.len();

        let bytes_written = if remaining_len < raw_s_len {
            self.truncated = true;
            match s.char_indices().rfind(|&(ind, _char)| ind <= remaining_len) {
                None => {
                    // noop, truncating all reftover chars
                    0
                }
                Some((ind, _cahr)) => ind,
            }
        } else {
            raw_s_len
        };
        remaining_buf[..bytes_written].copy_from_slice(&raw_s[..bytes_written]);
        self.used += bytes_written;
        self.leftover += raw_s_len - bytes_written;
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    use numtoa::NumToA;

    #[test]
    pub fn test() {
        let mut buffer = Buffer::<30>::new();
        let mut numtoa = [0u8; 10];

        buffer.write_str(42u32.numtoa_str(10, &mut numtoa));
        buffer.write_str(" - 0x");
        buffer.write_str(42u32.numtoa_str(16, &mut numtoa));

        assert_eq!("42 - 0x2A", buffer.as_str());
        assert_eq!(false, buffer.truncated());
    }

    #[test]
    pub fn test_longer() {
        let mut buffer = Buffer::<30>::new();
        let mut numtoa = [0u8; 10];

        buffer.write_str(4000u32.numtoa_str(10, &mut numtoa));
        buffer.write_str(" - 0x");
        buffer.write_str(4001u32.numtoa_str(16, &mut numtoa));

        assert_eq!("4000 - 0xFA1", buffer.as_str());
        assert_eq!(false, buffer.truncated());
    }

    #[test]
    pub fn test_longer_trunc() {
        let mut buffer = Buffer::<30>::new();
        let mut numtoa = [0u8; 10];

        buffer.write_str("long: ");
        buffer.write_str(400000u32.numtoa_str(10, &mut numtoa));
        buffer.write_str(" - 0x");
        buffer.write_str(400100u32.numtoa_str(16, &mut numtoa));

        assert_eq!("long: 400000 - 0x61AE4", buffer.as_str());
        assert_eq!(false, buffer.truncated());
    }

    #[test]
    pub fn test_too_long() {
        let mut buffer = Buffer::<30>::new();
        let mut numtoa = [0u8; 10];

        buffer.write_str("toooooo long:    ");
        buffer.write_str(400000u32.numtoa_str(10, &mut numtoa));
        buffer.write_str(" - 0x");
        buffer.write_str(400100u32.numtoa_str(16, &mut numtoa));

        assert_eq!("toooooo long:    400000 - 0x61", buffer.as_str());
        assert_eq!(true, buffer.truncated());
    }

    #[test]
    pub fn test_too_long_over_the_end() {
        let mut buffer = Buffer::<30>::new();
        let mut numtoa = [0u8; 10];

        buffer.write_str("toooooo long:    ");
        buffer.write_str(400000u32.numtoa_str(10, &mut numtoa));
        buffer.write_str(" - 0x");
        buffer.write_str(400100u32.numtoa_str(16, &mut numtoa));

        assert_eq!("toooooo long:    400000 - 0x61", buffer.as_str());
        assert_eq!(true, buffer.truncated());

        buffer.write_str("some more");
        assert_eq!("toooooo long:    400000 - 0x61", buffer.as_str());
        assert_eq!(true, buffer.truncated());
    }

    #[test]
    pub fn test_rewrite_undisplayable_chars() {
        let mut buffer = Buffer::<30>::new();

        buffer.write_str("Prefix: ");

        buffer.write_str("\x0A\x0D");
        assert_eq!("Prefix: \x7F\x7F", buffer.as_str());
        assert_eq!(false, buffer.truncated());
    }
}
