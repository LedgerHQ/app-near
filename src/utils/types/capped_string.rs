use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};

use super::{
    elipsis_fields::ElipsisFields,
    strcat::{self, read_leftover},
};
pub struct CappedString<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
    leftover: usize,
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

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used]) }
    }

    #[allow(unused)]
    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn ui_fields<'a>(
        &'a self,
        title: &'a str,
        display_buf: &'a mut [u8; 20],
    ) -> ElipsisFields<'a> {
        if self.truncated() {
            let mut numtoa_buf = [0u8; 10];

            let elipsis_descr = strcat::concatenate(
                &[
                    "... ",
                    self.leftover.numtoa_str(10, &mut numtoa_buf),
                    " bytes",
                ],
                display_buf,
            )
            .unwrap(); // Fails if self.display_buf is too small
            ElipsisFields::Two([
                Field {
                    name: title,
                    value: self.as_str(),
                },
                Field {
                    name: title,
                    value: elipsis_descr,
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
