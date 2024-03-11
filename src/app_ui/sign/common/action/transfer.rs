use crate::{
    parsing::{self},
    utils::types::elipsis_fields::ElipsisFields,
};
use fmt_buffer::Buffer;
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub amount_buffer: Buffer<30>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            amount_buffer: Buffer::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    transfer: &parsing::types::Transfer,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Transfer",
    }));

    transfer
        .deposit
        .display_as_buffer(&mut field_context.amount_buffer);

    writer.push_fields(ElipsisFields::one(Field {
        name: "Amount",
        value: field_context.amount_buffer.as_str(),
    }));
}
