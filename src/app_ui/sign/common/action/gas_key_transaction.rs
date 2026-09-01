#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};

pub struct FieldsContext {
    pub pub_key_context: tx_public_key_context::FieldsContext,
    pub balance_buffer: TokenBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            pub_key_context: tx_public_key_context::FieldsContext::new(),
            balance_buffer: TokenBuffer::new(),
        }
    }
}

const MAX_FIELDS: usize = 2;

pub fn format<'b, 'a: 'b>(
    gas_key_transaction: &'b parsing::types::GasKeyTransactionData,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
    gas_key_transaction_balance_action: &'b str,
) {
    field_context
        .pub_key_context
        .format_public_key(&gas_key_transaction.public_key);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Gas Key",
        value: field_context.pub_key_context.as_str(),
    }));

    gas_key_transaction
        .amount
        .display_as_buffer(&mut field_context.balance_buffer);
    writer.push_fields(ElipsisFields::one(Field {
        name: gas_key_transaction_balance_action,
        value: field_context.balance_buffer.as_str(),
    }));
}
