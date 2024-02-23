use crate::{
    parsing::{self},
    sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};
use fmt_buffer::Buffer;
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub stake_buffer: Buffer<30>,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            stake_buffer: Buffer::new(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    stake: &parsing::types::Stake,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 3>,
) {
    field_context
        .pub_key_context
        .format_public_key(&stake.public_key);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type",
            value: "Stake",
        }))
        .unwrap();

    stake
        .stake
        .display_as_buffer(&mut field_context.stake_buffer);
    writer
        .push_fields(ElipsisFields::One([Field {
            name: "Stake",
            value: field_context.stake_buffer.as_str(),
        }]))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Public Key",
            value: field_context.pub_key_context.buffer.as_str(),
        }))
        .unwrap();
}
