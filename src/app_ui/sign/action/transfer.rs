use crate::parsing::{self, types::action::ONE_NEAR};
use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter,
    utils::types::capped_string::ElipsisFields,
};

pub struct FieldsContext {
    pub float_buffer: dtoa::Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            float_buffer: dtoa::Buffer::new(),
        }
        
    }
}

pub fn format<'b, 'a: 'b>(
    transfer: &parsing::types::Transfer,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: "Transfer",
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }

    let deposit = (transfer.deposit as f64) / (ONE_NEAR as f64);
    let printed = field_context.float_buffer.format(deposit);
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Amount (NEAR)",
        value: printed,
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }
}
