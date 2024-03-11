use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

use super::{
    capped_string::CappedString,
    hex_display::HexDisplay,
    strcat::{self, concatenate},
};

use fmt_buffer::Buffer;

pub enum ElipsisFields<'a> {
    One([Field<'a>; 1]),
    Two([Field<'a>; 2]),
}

impl<'a> ElipsisFields<'a> {
    pub fn one(field: Field<'a>) -> Self {
        ElipsisFields::One([field])
    }

    pub fn from_fmt_buffer<const N: usize>(
        source: &'a Buffer<N>,
        title: &'a str,
        display_buf: &'a mut [u8; 20],
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = [0u8; 10];

            let elipsis_descr = strcat::concatenate(
                &[
                    "... ",
                    // numtoa_buf has to be at least 10 bytes for usize/u32(4 bytes) : ok
                    source.leftover().numtoa_str(10, &mut numtoa_buf),
                    " bytes",
                ],
                display_buf,
            )
            .unwrap(); // .unwrap() is ok because display_buf.len(): 20 >= 20
            ElipsisFields::Two([
                Field {
                    name: title,
                    value: source.as_str(),
                },
                Field {
                    name: title,
                    value: elipsis_descr,
                },
            ])
        } else {
            return ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }]);
        }
    }

    pub fn from_hex_display<const N: usize>(
        source: &'a HexDisplay<N>,
        title: &'a str,
        display_buf: &'a mut [u8; 20],
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = [0u8; 10];

            let elipsis_descr = concatenate(
                &[
                    "... ",
                    // numtoa_buf has to be at least 10 bytes for usize/u32(4 bytes) : ok
                    source.leftover().numtoa_str(10, &mut numtoa_buf),
                    " bytes",
                ],
                display_buf,
            )
            .unwrap(); // .unwrap() is ok because display_buf.len(): 20 >= 20
            ElipsisFields::Two([
                Field {
                    name: title,
                    value: source.as_str(),
                },
                Field {
                    name: title,
                    value: elipsis_descr,
                },
            ])
        } else {
            return ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }]);
        }
    }

    pub fn from_capped_string<const N: usize>(
        source: &'a CappedString<N>,
        title: &'a str,
        display_buf: &'a mut [u8; 20],
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = [0u8; 10];

            let elipsis_descr = strcat::concatenate(
                &[
                    "... ",
                    // numtoa_buf has to be at least 10 bytes for usize/u32(4 bytes) : ok
                    source.leftover().numtoa_str(10, &mut numtoa_buf),
                    " bytes",
                ],
                display_buf,
            )
            .unwrap(); // .unwrap() is ok because display_buf.len(): 20 >= 20
            ElipsisFields::Two([
                Field {
                    name: title,
                    value: source.as_str(),
                },
                Field {
                    name: title,
                    value: elipsis_descr,
                },
            ])
        } else {
            return ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }]);
        }
    }
}
