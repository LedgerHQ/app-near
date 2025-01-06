use core::panic;

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::utils::types::elipsis_fields::ElipsisFields;

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
    pub fn push_fields(&mut self, val: ElipsisFields<'a>) {
        match val {
            ElipsisFields::One(array) => {
                for elem in array {
                    if self.used == self.buffer.len() {
                        #[cfg(feature = "speculos")]
                        testing::debug_print("FieldsWriter.push_fields capacity overflow\n");
                        panic!("FieldsWriter.push_fields capacity overflow");
                    }
                    self.buffer[self.used] = elem;
                    self.used += 1;
                }
            }
            ElipsisFields::Two(array) => {
                for elem in array {
                    if self.used == self.buffer.len() {
                        #[cfg(feature = "speculos")]
                        testing::debug_print("FieldsWriter.push_fields capacity overflow\n");
                        panic!("FieldsWriter.push_fields capacity overflow");
                    }
                    self.buffer[self.used] = elem;
                    self.used += 1;
                }
            }
        }
    }

    pub fn get_fields(&self) -> &[Field<'a>] {
        &self.buffer[0..self.used]
    }
}
