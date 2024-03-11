use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

use crate::utils::types::strcat::read_leftover;

use super::capped_string::CappedString;

impl<const N: usize> From<CappedString<N>> for HexDisplay<N> {
    fn from(value: CappedString<N>) -> Self {
        Self {
            buffer: value.buffer,
            used: value.used,
            truncated: value.truncated,
            leftover: value.leftover,
        }
    }
}

/// A type with first stores a byte buffer into its internal buffer;
/// and then reuses it to display string hex representation of buffer/2 bytes
pub struct HexDisplay<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
    leftover: usize,
}

impl<const N: usize> HexDisplay<N> {
    pub fn new() -> Self {
        HexDisplay {
            buffer: [0u8; N],
            used: 0,
            leftover: 0,
            truncated: false,
        }
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn leftover(&self) -> usize {
        self.leftover
    }

    /// this method must only be called once
    pub fn reformat(&mut self) {
        let prev_used = self.used;
        let mut new_used = self.used * 2;
        if new_used > self.buffer.len() {
            new_used = self.buffer.len()
        }

        self.used = new_used / 2;

        self.leftover += prev_used - self.used;
        if self.leftover > 0 {
            self.truncated = true;
        }
        let mut tmp_buffer = [0u8; 1];

        for ind in (0..self.used).rev() {
            let char_range = ind * 2..=ind * 2 + 1;
            tmp_buffer.copy_from_slice(&self.buffer[ind..ind + 1]);
            // .unwrap() is ok, as `2 == 1 * 2` holds true
            hex::encode_to_slice(tmp_buffer, &mut self.buffer[char_range]).unwrap();
        }
    }

    /// # Panics
    ///
    /// this method should be only called after `reformat` is called;
    /// otherwise it may panic with out of slice bounds access
    pub fn as_str(&self) -> &str {
        // .unwrap() is ok, as buffer contains only bytes, encoding hex chars
        core::str::from_utf8(&self.buffer[..self.used * 2]).unwrap()
    }
}

impl<const N: usize> HexDisplay<N> {
    #[allow(unused)]
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
        } else {
            let leftover = (bytes_count as usize) - self.buffer.len();
            reader.read_exact(&mut self.buffer)?;

            self.used = self.buffer.len();
            self.leftover = leftover;
            if leftover > 0 {
                read_leftover(leftover, reader)?;
            }
        }

        Ok(())
    }
}
