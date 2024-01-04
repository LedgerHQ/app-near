use ledger_device_sdk::ui::gadgets::Field;

use super::capped_string::CappedString;

pub enum ElipsisFields<'a> {
    One([Field<'a>; 1]),
    Two([Field<'a>; 2]),
}

impl<'a> ElipsisFields<'a> {
    pub fn one(field: Field<'a>) -> Self {
        ElipsisFields::One([field])
    }
    
}

pub fn capped_string_fields<'a, const N: usize>(
    string: &'a CappedString<N>,
    title: &'a str,
) -> ElipsisFields<'a> {
    if string.truncated() {
        ElipsisFields::Two([
            Field {
                name: title,
                value: string.as_str(),
            },
            Field {
                name: title,
                value: "...",
            },
        ])
    } else {
        return ElipsisFields::One([Field {
            name: title,
            value: string.as_str(),
        }]);
    }
}

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
