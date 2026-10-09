use core::panic;

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::utils::types::review_field::ReviewField;

pub struct FieldsWriter<'a, const N: usize> {
    buffer: [Field<'a>; N],
    used: usize,
}

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

impl<'a, const N: usize> FieldsWriter<'a, N> {
    pub fn new() -> Self {
        let max_fields = [(); N].map(|_| Field {
            name: "",
            value: "",
        });

        Self {
            buffer: max_fields,
            used: 0,
        }
    }
    /// # Panics
    ///
    /// Panics if self.buffer capacity was chosen too small.
    /// Choosing self.buffer capacity depends on static scenarios (max number of fields)
    /// can be calculated per scenario)
    pub fn push_field(&mut self, val: ReviewField<'a>) {
        if self.used == self.buffer.len() {
            #[cfg(feature = "speculos")]
            testing::debug_print("FieldsWriter.push_field capacity overflow\n");
            panic!("FieldsWriter.push_field capacity overflow");
        }
        self.buffer[self.used] = val.into_field();
        self.used += 1;
    }

    pub fn get_fields(&self) -> &[Field<'a>] {
        &self.buffer[0..self.used]
    }
}
