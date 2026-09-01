use crate::app_ui::aliases::U64Buffer;
use crate::parsing::types::AccessKeyPermission;
use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
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

/// Public Key (1) + Access Key Type (1) +
/// Access Key Nonce (1) + Access Permission (1) +
/// GasKey Balance (1) + GasKey Number of Nonces (1)
/// FnCall Allowance (1) +
/// FnCall Receiver `ElipsisFields` (1-2) + Total FnCall Methods (1) +
/// Method Name `ElipsisFields` (1-2)
pub const MAX_FIELDS_ADD_KEY: usize = 12;

pub fn format<'b, 'a: 'b, const N: usize>(
    add_key: &parsing::types::AddKey,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, N>,
    permission_value: &'b str,
) {
    field_context
        .pub_key_context
        .format_public_key(&add_key.public_key);

    let key_type_str: &str = match add_key.access_key.permission {
        AccessKeyPermission::FullAccess | AccessKeyPermission::FunctionCall => "Standard Key",
        AccessKeyPermission::GasKeyFullAccess | AccessKeyPermission::GasKeyFunctionCall => {
            "Gas Key"
        }
    };

    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Access Key Type",
        value: key_type_str,
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
