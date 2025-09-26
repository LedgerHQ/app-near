#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use numtoa::NumToA;

use crate::app_ui::aliases::U32Buffer;

use super::{
    capped_string::CappedString,
    hex_display::HexDisplay,
    strcat::{self, concatenate},
};

use fmt_buffer::Buffer;

/// A buffer, large enough to contain string with number of leftover bytes: `... N bytes`
/// where N is u32
pub type EllipsisBuffer = [u8; 20];

pub enum ElipsisFields<'a> {
    One([Field<'a>; 1]),
    Two([Field<'a>; 2]),
}

impl<'a> ElipsisFields<'a> {
    pub fn one(field: Field<'a>) -> Self {
        ElipsisFields::One([field])
    }

    pub fn from_fmt_buffer<const N: usize>(
        source: &'a mut Buffer<N>,
        title: &'a str,
        display_buf: &'a mut EllipsisBuffer,
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = U32Buffer::default();

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
            ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }])
        }
    }

    pub fn from_hex_display<const N: usize>(
        source: &'a HexDisplay<N>,
        title: &'a str,
        display_buf: &'a mut EllipsisBuffer,
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = U32Buffer::default();

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
            ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }])
        }
    }

    pub fn from_capped_string<const N: usize>(
        source: &'a mut CappedString<N>,
        title: &'a str,
        display_buf: &'a mut EllipsisBuffer,
    ) -> Self {
        if source.truncated() {
            let mut numtoa_buf = U32Buffer::default();

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
            ElipsisFields::One([Field {
                name: title,
                value: source.as_str(),
            }])
        }
    }
}
