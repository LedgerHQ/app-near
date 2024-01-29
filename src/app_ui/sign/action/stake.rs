use crate::parsing::{self, types::action::ONE_NEAR};
use ledger_device_sdk::ui::gadgets::Field;

use crate::{app_ui::fields_writer::FieldsWriter, utils::types::capped_string::ElipsisFields};

use super::delete_key;

pub struct FieldsContext {
    pub float_buffer: dtoa::Buffer,
    pub pub_key_context: delete_key::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            float_buffer: dtoa::Buffer::new(),
            pub_key_context: delete_key::FieldsContext::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    stake: &parsing::types::Stake,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 3>,
) {
    let stake_amount = (stake.stake as f64) / (ONE_NEAR as f64);
    let printed_amount = field_context.float_buffer.format(stake_amount);
    field_context
        .pub_key_context
        .format_public_key(&stake.public_key);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type",
            value: "Stake",
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Stake (NEAR)",
            value: printed_amount,
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Public Key",
            value: field_context.pub_key_context.buffer.as_str(),
        }))
        .unwrap();
}
