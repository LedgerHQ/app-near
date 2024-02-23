#![no_std]
use fmt_buffer::Buffer;
use numtoa::NumToA;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct NearGas(u64);

/// Gas is a type for storing amount of gas.
pub type Gas = u64;

const ONE_GIGA_GAS: u64 = 10u64.pow(9);
const ONE_TERA_GAS: u64 = 10u64.pow(12);

impl NearGas {
    /// Returns the total number of a whole part of tera Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// let neargas = NearGas::from_gas(1 * 1_000_000_000_000);
    /// assert_eq!(neargas.as_tgas(), 1);
    /// ```
    pub const fn as_tgas(self) -> u64 {
        self.0 / ONE_TERA_GAS
    }
    /// Creates a new `NearGas` from the specified number of whole giga Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    ///
    /// let giga_gas = NearGas::from_ggas(5);
    ///
    /// assert_eq!(giga_gas.as_gas(), 5 * 1_000_000_000);
    /// ```
    pub const fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self(inner)
    }

    /// Creates a new `NearGas` from the specified number of whole Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    ///
    /// let gas = NearGas::from_gas(5 * 1_000_000_000_000);
    ///
    /// assert_eq!(gas.as_tgas(), 5);
    /// ```
    pub const fn from_gas(inner: u64) -> Self {
        Self(inner)
    }

    /// Returns the total number of whole Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// let neargas = NearGas::from_gas(12345);
    /// assert_eq!(neargas.as_gas(), 12345);
    /// ```
    pub const fn as_gas(self) -> u64 {
        self.0
    }

    pub fn display_as_buffer(&self, result: &mut Buffer<30>) {
        if *self == NearGas::from_gas(0) {
            result.write_str("0 Tgas");
        } else if *self < NearGas::from_ggas(1) {
            result.write_str("less than 0.001 Tgas");
        } else if *self <= NearGas::from_ggas(999) {
            let gigagas_rounded_up: u32 =
                (self.as_gas().saturating_add(ONE_GIGA_GAS - 1) / ONE_GIGA_GAS) as u32;
            let mut millis_str_buf = [0u8; 10];

            result.write_str("0.");
            let millis_str = gigagas_rounded_up.numtoa_str(10, &mut millis_str_buf);
            let leading_zeros = 3 - millis_str.len();

            for _ in 0..leading_zeros {
                result.write_str("0");
            }

            result.write_str(millis_str);
            result.write_str(" Tgas");
        } else {
            let terragas_rounded_up: u64 =
                self.as_gas().saturating_add(100 * ONE_GIGA_GAS - 1) / ONE_GIGA_GAS / 100;
            let mut str_buf = [0u8; 20];

            result.write_str((terragas_rounded_up / 10).numtoa_str(10, &mut str_buf));
            result.write_str(".");
            result.write_str((terragas_rounded_up % 10).numtoa_str(10, &mut str_buf));

            result.write_str(" Tgas");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::NearGas;
    use fmt_buffer::Buffer;

    #[test]
    fn test_display() {
        for (near_gas, expected_display) in [
            (NearGas::from_gas(0), "0 Tgas"),
            (NearGas::from_gas(1), "less than 0.001 Tgas"),
            (NearGas::from_gas(999_999_999), "less than 0.001 Tgas"),
            (NearGas::from_gas(1_000_000_000), "0.001 Tgas"),
            (NearGas::from_gas(1_000_000_001), "0.002 Tgas"),
            (NearGas::from_gas(2_000_000_000), "0.002 Tgas"),
            (NearGas::from_gas(200_000_000_000), "0.200 Tgas"),
            (NearGas::from_gas(999_000_000_000), "0.999 Tgas"),
            (NearGas::from_gas(999_000_000_001), "1.0 Tgas"),
            (NearGas::from_gas(999_999_999_999), "1.0 Tgas"),
            (NearGas::from_gas(1_000_000_000_000), "1.0 Tgas"),
            (NearGas::from_gas(1_000_000_000_001), "1.1 Tgas"),
            (NearGas::from_gas(1_234_567_000_000), "1.3 Tgas"),
            (NearGas::from_gas(1_500_000_000_000), "1.5 Tgas"),
            (NearGas::from_gas(10_000_000_000_000), "10.0 Tgas"),
            (NearGas::from_gas(10_500_000_000_000), "10.5 Tgas"),
            (NearGas::from_gas(99_999_999_999_999), "100.0 Tgas"),
            (NearGas::from_gas(100_000_000_000_000), "100.0 Tgas"),
            (NearGas::from_gas(100_500_000_000_000), "100.5 Tgas"),
            (NearGas::from_gas(1_000_500_000_000_000), "1000.5 Tgas"),
            (
                NearGas::from_gas(1_000_000_500_000_000_000),
                "1000000.5 Tgas",
            ),
        ] {
            let mut buffer: Buffer<30> = Buffer::new();
            near_gas.display_as_buffer(&mut buffer);

            assert_eq!(buffer.as_str(), expected_display);
            assert_eq!(buffer.truncated(), false);
        }
    }
}
