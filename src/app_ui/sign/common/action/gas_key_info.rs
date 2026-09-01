#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;

use crate::{
    app_ui::{aliases::U16Buffer, fields_writer::FieldsWriter},
    parsing,
    utils::types::elipsis_fields::ElipsisFields,
};
use numtoa::NumToA;

pub struct FieldsContext {
    pub balance_buffer: TokenBuffer,
    pub num_nonces_buffer: U16Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            balance_buffer: TokenBuffer::new(),
            num_nonces_buffer: U16Buffer::default(),
        }
    }
}

const MAX_FIELDS: usize = super::add_key_common::MAX_FIELDS_ADD_KEY;

pub fn format<'b, 'a: 'b>(
    gas_key_info: &'a mut parsing::types::GasKeyInfo,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    gas_key_info
        .balance
        .display_as_buffer(&mut field_context.balance_buffer);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Gas Key Balance",
        value: field_context.balance_buffer.as_str(),
    }));

    let num_nonces_str = gas_key_info
        .num_nonces
        .numtoa_str(10, &mut field_context.num_nonces_buffer);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Gas Key Nonces",
        value: num_nonces_str,
    }));
}
