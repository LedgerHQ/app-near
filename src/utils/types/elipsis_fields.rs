use ledger_device_sdk::ui::gadgets::Field;

pub enum ElipsisFields<'a> {
    One([Field<'a>; 1]),
    Two([Field<'a>; 2]),
}

impl<'a> ElipsisFields<'a> {
    pub fn one(field: Field<'a>) -> Self {
        ElipsisFields::One([field])
    }
}
