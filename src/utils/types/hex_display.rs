use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

use super::strcat::read_leftover;

/// A type with first stores a byte buffer into its internal buffer;
/// and then reuses it to display string hex representation of buffer/2 bytes
pub struct HexDisplay<const N: usize> {
    pub buffer: [u8; N],
    pub used: usize,
    pub truncated: bool,
    pub leftover: usize,
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

    #[allow(unused)]
    pub fn truncated(&self) -> bool {
        self.truncated
    }

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
            hex::encode_to_slice(tmp_buffer, &mut self.buffer[char_range]).unwrap();
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used * 2]) }
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
