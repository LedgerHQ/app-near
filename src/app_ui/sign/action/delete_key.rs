use ledger_device_sdk::ui::gadgets::Field;

use crate::{parsing, utils::types::capped_string::ElipsisFields, app_ui::fields_writer::FieldsWriter};

pub struct FieldsContext {
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
        }
        
    }
}

pub fn format<'b, 'a: 'b>(
    delete_key: &parsing::types::DeleteKey,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: "Delete Key",
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }
}
