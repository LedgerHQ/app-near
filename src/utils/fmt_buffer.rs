pub struct TruncatingFmtBuffer<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
}

impl<const N: usize> TruncatingFmtBuffer<N> {

    pub fn new() -> Self {
        TruncatingFmtBuffer {
            buffer: [0u8; N],
            used: 0,
            truncated: false,
        }
    }

    pub fn as_str(&self) -> &str {
        debug_assert!(self.used <= self.buffer.len());
        use core::str::from_utf8_unchecked;
        unsafe { from_utf8_unchecked(&self.buffer[..self.used]) }
    }

    #[allow(unused)]
    pub fn truncated(&self) -> bool {
        self.truncated
        
    }
}

// NOTE: doing formatting with 
// impl<const N: usize> core::fmt::Write for TruncatingFmtBuffer<N> {
// doesn't work due to app crashes in speculos and hangs of app on device )
// potentially similar issue: https://github.com/rust-lang/rust/issues/44538
impl<const N: usize> TruncatingFmtBuffer<N> {
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
                    return;
                }
                Some((ind, _cahr)) => ind,
            }
        } else {
            raw_s_len
        };
        remaining_buf[..bytes_written].copy_from_slice(&raw_s[..bytes_written]);
        self.used += bytes_written;
    }
}

