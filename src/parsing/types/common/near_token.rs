use crate::utils::types::fmt_buffer::FmtBuffer;

use super::action::Balance;

const ONE_MILLINEAR: u128 = 10_u128.pow(21);
use numtoa::NumToA;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NearToken(pub Balance);

impl NearToken {
    /// `from_yoctonear` is a function that takes value by a number of yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!( NearToken::from_yoctonear(10u128.pow(21)), NearToken::from_millinear(1))
    /// ```
    pub const fn from_yoctonear(inner: u128) -> Self {
        Self(inner)
    }

    /// `from_millinear` is a function that takes value by a number of mili-near and converts it to an equivalent to the yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_millinear(1), NearToken::from_yoctonear(10u128.pow(21)))
    /// ```
    pub const fn from_millinear(inner: u128) -> Self {
        Self(inner * ONE_MILLINEAR)
    }

    /// `as_yoctonear` is a function that shows a number of yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10).as_yoctonear(), 10)
    /// ```
    pub const fn as_yoctonear(&self) -> u128 {
        self.0
    }

    pub fn display_as_buffer(&self, result: &mut FmtBuffer<30>) {
        if *self == NearToken::from_yoctonear(0) {
            result.write_str("0 NEAR");
        } else if *self == NearToken::from_yoctonear(1) {
            result.write_str("1 yoctoNEAR");
        } else if *self < NearToken::from_millinear(1) {
            result.write_str("less than 0.001 NEAR");
        } else if *self <= NearToken::from_millinear(999) {
            let millinear_rounded_up =
                self.as_yoctonear().saturating_add(ONE_MILLINEAR - 1) / ONE_MILLINEAR;

            let mut millis_str_buf = [0u8; 10];

            result.write_str("0.");
            let millis_str = millinear_rounded_up.numtoa_str(10, &mut millis_str_buf);
            let leading_zeros = 3 - millis_str.len();
            for _ in 0..leading_zeros {
                result.write_str("0");
            }
            result.write_str(millis_str);
            result.write_str(" NEAR");
        } else {
            let near_rounded_up =
                self.as_yoctonear().saturating_add(10 * ONE_MILLINEAR - 1) / ONE_MILLINEAR / 10;
            let mut str_buf = [0u8; 20];

            result.write_str((near_rounded_up / 100).numtoa_str(10, &mut str_buf));
            result.write_str(".");
            let hundreds_str = (near_rounded_up % 100).numtoa_str(10, &mut str_buf);
            let leading_zeros = 2 - hundreds_str.len();
            for _ in 0..leading_zeros {
                result.write_str("0");
            }
            result.write_str(hundreds_str);
            result.write_str(" NEAR");
        }
    }
}
