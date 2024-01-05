use ledger_device_sdk::ui::gadgets::Field;

use crate::utils::types::capped_string::ElipsisFields;



pub struct FieldsWriter<'a, const N: usize> {
    buffer: [Field<'a>; N],
    used: usize,
}

pub struct FieldsOverflow;

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
    pub fn push_fields(&mut self, val: ElipsisFields<'a>) -> Result<(), FieldsOverflow> {
        match val {
            ElipsisFields::One(array) => {
                for elem in array {
                    if self.used == self.buffer.len() {
                        return Err(FieldsOverflow);
                    }
                    self.buffer[self.used] = elem;
                    self.used += 1;
                }
            }
            ElipsisFields::Two(array) => {
                for elem in array {
                    if self.used == self.buffer.len() {
                        return Err(FieldsOverflow);
                    }
                    self.buffer[self.used] = elem;
                    self.used += 1;
                }
            }
        };
        Ok(())
    }

    pub fn get_fields(&self) -> &[Field<'a>] {
        &self.buffer[0..self.used]
    }
}
