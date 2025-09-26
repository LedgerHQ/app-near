use crate::app_ui::aliases::U32Buffer;
use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing,
    utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer},
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: U32Buffer,
    pub receiver_display_buf: EllipsisBuffer,
    pub method_names_display_buf: EllipsisBuffer,
    pub allowance_buffer: TokenBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: U32Buffer::default(),
            receiver_display_buf: EllipsisBuffer::default(),
            method_names_display_buf: EllipsisBuffer::default(),
            allowance_buffer: TokenBuffer::new(),
        }
    }
}

/// action type (1) + Public Key (1) + Access Key Nonce (1) +
/// Access Permission (1) + FnCall Allowance (1)  +
/// FnCall Receiver `ElipsisFields` (1-2) + Total FnCall Methods (1) +
/// Method Names `ElipsisFields` (1-2)
const MAX_FIELDS: usize = 10;

pub fn format<'b, 'a: 'b>(
    function_call_perm: &'a mut parsing::types::FunctionCallPermission,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let allowance = match function_call_perm.allowance {
        Some(allowance) => {
            allowance.display_as_buffer(&mut field_context.allowance_buffer);
            field_context.allowance_buffer.as_str()
        }
        None => "Unlimited NEAR",
    };
    writer.push_fields(ElipsisFields::one(Field {
        name: "FnCall Allowance",
        value: allowance,
    }));

    let recevier_id = ElipsisFields::from_capped_string(
        &mut function_call_perm.receiver_id,
        "FnCall Receiver",
        &mut field_context.receiver_display_buf,
    );

    writer.push_fields(recevier_id);

    writer.push_fields(ElipsisFields::one(Field {
        name: "Total FnCall Methods",
        value: function_call_perm
            .number_of_method_names
            // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
            .numtoa_str(10, &mut field_context.num_buf),
    }));

    let methods_names_fields = ElipsisFields::from_fmt_buffer(
        &mut function_call_perm.method_names,
        "Method Names",
        &mut field_context.method_names_display_buf,
    );

    writer.push_fields(methods_names_fields);
}
