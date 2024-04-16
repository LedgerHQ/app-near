use crate::{
    parsing::{self},
    utils::types::elipsis_fields::ElipsisFields,
};
use ledger_device_sdk::ui::gadgets::Field;
use near_token::TokenBuffer;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub amount_buffer: TokenBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            amount_buffer: TokenBuffer::new(),
        }
    }
}

/// action type (1) + Amount (1)
const MAX_FIELDS: usize = 2;

pub fn format<'b, 'a: 'b>(
    transfer: &parsing::types::Transfer,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
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
