use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};
pub enum ElipsisFields<'a> {
    One([Field<'a>; 1]),
    Two([Field<'a>; 2]),
}

impl<'a> ElipsisFields<'a> {
    pub fn one(field: Field<'a>) -> Self {
        ElipsisFields::One([field])
    }
}

pub struct CappedString<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
}

impl<const N: usize> CappedString<N> {
    // #[allow(unused)]
    // const fn assert_size() {
    //     if N <= ELIPSIS_SIZE {
    //         panic!("smaller than ellipsis size");
    //     }
    // }

    pub fn new(truncated: bool) -> Self {
        CappedString {
            buffer: [0u8; N],
            used: 0,
            truncated,
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used]) }
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

fn read_leftover<R: Read>(leftover: usize, reader: &mut R) -> Result<()> {
    let mut leftover_buff = [0u8; 20];

    let iters = leftover / leftover_buff.len();
    let remainder = leftover % leftover_buff.len();

    for _i in 0..iters {
        reader.read_exact(&mut leftover_buff)?;
    }
    reader.read_exact(&mut leftover_buff[0..remainder])?;
    Ok(())
}

impl<const N: usize> CappedString<N> {
    // BorshDeserialize counterpart to reduce size of stack frames
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;
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

            match core::str::from_utf8(&self.buffer) {
                Ok(_result) => {
                    self.used = self.buffer.len();
                }
                Err(err) => {
                    if err.error_len().is_some() {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    let valid_utf8_up_to = err.valid_up_to();
                    self.used = valid_utf8_up_to;
                }
            }

            if leftover > 0 {
                read_leftover(leftover, reader)?;
            }
        }

        Ok(())
    }
}
