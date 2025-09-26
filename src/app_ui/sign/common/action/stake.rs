use crate::{
    parsing::{self},
    sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub stake_buffer: TokenBuffer,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            stake_buffer: TokenBuffer::new(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
        }
    }
}
/// action type (1) + Stake (1) + Public Key (1)
const MAX_FIELDS: usize = 3;

pub fn format<'b, 'a: 'b>(
    stake: &parsing::types::Stake,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    field_context
        .pub_key_context
        .format_public_key(&stake.public_key);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Stake",
    }));

    stake
        .stake
        .display_as_buffer(&mut field_context.stake_buffer);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Stake",
        value: field_context.stake_buffer.as_str(),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));
}
