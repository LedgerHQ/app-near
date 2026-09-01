use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::elipsis_fields::ElipsisFields,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;

pub struct PostfixFieldsContext {
    pub deposit_buf: TokenBuffer,
}

impl PostfixFieldsContext {
    pub fn new() -> Self {
        Self {
            deposit_buf: TokenBuffer::new(),
        }
    }
}

/// State Init Version (1) + Global Contract Identifier (1) + state size (1) + num of state entries (1) +
/// deposit
const MAX_FIELDS: usize = 5;

pub fn format<'b, const N: usize>(
    state_init_version_str: &'b str,
    writer: &'_ mut FieldsWriter<'b, N>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "State Init Version",
        value: state_init_version_str,
    }));
}

pub fn format_postfix<'b, 'a: 'b>(
    args: &'b parsing::types::DeterministicAccountStateInitPostfix,
    field_context: &'a mut PostfixFieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    args.deposit
        .display_as_buffer(&mut field_context.deposit_buf);

    writer.push_fields(ElipsisFields::one(Field {
        name: "Deposit",
        value: field_context.deposit_buf.as_str(),
    }));
}
