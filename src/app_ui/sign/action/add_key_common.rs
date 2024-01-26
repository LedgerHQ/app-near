use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::capped_string::ElipsisFields,
};

use super::delete_key;
use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: [u8; 20],
    pub pub_key_context: delete_key::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: [0u8; 20],
            pub_key_context: delete_key::FieldsContext::new(),
        }
    }
}

pub fn format<'b, 'a: 'b, const N: usize>(
    add_key: &parsing::types::AddKey,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, N>,
    permission_value: &'b str,
) {
    field_context
        .pub_key_context
        .format_public_key(&add_key.public_key);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type:",
            value: "Add Key",
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Public Key:",
            value: field_context.pub_key_context.buffer.as_str(),
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Access Key Nonce:",
            value: add_key
                .access_key
                .nonce
                .numtoa_str(10, &mut field_context.num_buf),
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Access Permission:",
            value: permission_value,
        }))
        .unwrap();
}
