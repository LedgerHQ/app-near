use crate::app_ui::aliases::U64Buffer;
use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};

use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: U64Buffer,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: U64Buffer::default(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
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
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Add Key",
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Access Key Nonce",
        value: add_key
            .access_key
            .nonce
            // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
            .numtoa_str(10, &mut field_context.num_buf),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Access Permission",
        value: permission_value,
    }));
}
