#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::elipsis_fields::ElipsisFields,
};

use near_gas::GasBuffer;

/// priority_fee buffer(1)
const MAX_FIELDS: usize = 1;

pub struct FieldsContext {
    priority_fee_buf: GasBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            priority_fee_buf: GasBuffer::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    v1_suffix: &'b parsing::types::transaction::suffix::V1Suffix,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    v1_suffix
        .priority_fee
        .display_as_buffer(&mut field_context.priority_fee_buf);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Gas Priority Fee",
        value: field_context.priority_fee_buf.as_str(),
    }));
}
