#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use super::{capped_string::CappedString, hex_display::HexDisplay};
use crate::sign_ui::blind_signing::BlindSigned;

use fmt_buffer::Buffer;

/// A field pushed to a review.
///
/// A signed value is displayed in full: a value that does not fit its buffer is
/// rejected while parsing, before any review screen, unless the user approved signing
/// it blind. The constructors below panic on a truncated source instead of showing part
/// of it, so a display site that misses the parsing check exits the app rather than
/// asking to sign a value the user could not see. Only the `_blind` constructors show a
/// truncated source, and they take the [`BlindSigned`] approval as proof.
pub struct ReviewField<'a>(Field<'a>);

/// # Panics
///
/// Panics if `truncated`: the parser rejects a value that does not fit its buffer,
/// so a truncated value reaching the review is a bug.
fn fail_if_truncated(truncated: bool) {
    if truncated {
        #[cfg(feature = "speculos")]
        testing::debug_print("ReviewField: truncated value reached the review\n");
        panic!("ReviewField: truncated value reached the review");
    }
}

impl<'a> ReviewField<'a> {
    pub fn new(field: Field<'a>) -> Self {
        ReviewField(field)
    }

    pub fn into_field(self) -> Field<'a> {
        self.0
    }

    /// # Panics
    ///
    /// Panics if `source` is truncated.
    pub fn from_fmt_buffer<const N: usize>(source: &'a mut Buffer<N>, title: &'a str) -> Self {
        fail_if_truncated(source.truncated());
        Self::new(Field {
            name: title,
            value: source.as_str(),
        })
    }

    /// # Panics
    ///
    /// Panics if `source` is truncated.
    pub fn from_hex_display<const N: usize>(source: &'a HexDisplay<N>, title: &'a str) -> Self {
        fail_if_truncated(source.truncated());
        Self::new(Field {
            name: title,
            value: source.as_str(),
        })
    }

    /// # Panics
    ///
    /// Panics if `source` is truncated.
    pub fn from_capped_string<const N: usize>(
        source: &'a mut CappedString<N>,
        title: &'a str,
    ) -> Self {
        fail_if_truncated(source.truncated());
        Self::new(Field {
            name: title,
            value: source.as_str(),
        })
    }

    /// Shows the part of `source` that fits, which may be all of it, for a value the
    /// user approved signing blind.
    pub fn from_capped_string_blind<const N: usize>(
        source: &'a mut CappedString<N>,
        title: &'a str,
        _approved: &BlindSigned,
    ) -> Self {
        Self::new(Field {
            name: title,
            value: source.as_str(),
        })
    }

    /// Shows the part of `source` that fits, which may be all of it, for a value the
    /// user approved signing blind.
    pub fn from_hex_display_blind<const N: usize>(
        source: &'a HexDisplay<N>,
        title: &'a str,
        _approved: &BlindSigned,
    ) -> Self {
        Self::new(Field {
            name: title,
            value: source.as_str(),
        })
    }
}
