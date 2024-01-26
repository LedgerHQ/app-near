use ledger_device_sdk::ui::gadgets::Field;

use super::capped_string::ElipsisFields;

pub struct FmtBuffer<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
}

impl<const N: usize> FmtBuffer<N> {
    pub fn new() -> Self {
        FmtBuffer {
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

    pub fn ui_fields<'a>(&'a self, title: &'a str) -> ElipsisFields<'a> {
        if self.truncated() {
            ElipsisFields::Two([
                Field {
                    name: title,
                    value: self.as_str(),
                },
                Field {
                    name: title,
                    value: "...",
                },
            ])
        } else {
            return ElipsisFields::One([Field {
                name: title,
                value: self.as_str(),
            }]);
        }
    }
}

impl<const N: usize> FmtBuffer<N> {
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
