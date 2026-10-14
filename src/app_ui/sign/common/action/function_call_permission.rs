use crate::app_ui::aliases::U32Buffer;
use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::review_field::ReviewField,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_token::TokenBuffer;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: U32Buffer,
    pub allowance_buffer: TokenBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: U32Buffer::default(),
            allowance_buffer: TokenBuffer::new(),
        }
    }
}

/// action type (1) + Public Key (1) + Access Key Nonce (1) +
/// Access Permission (1) + FnCall Allowance (1)  +
/// FnCall Receiver (1) + Total FnCall Methods (1) +
/// Method Names (1)
const MAX_FIELDS: usize = 8;

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
    writer.push_field(ReviewField::new(Field {
        name: "FnCall Allowance",
        value: allowance,
    }));

    let recevier_id =
        ReviewField::from_capped_string(&mut function_call_perm.receiver_id, "FnCall Receiver");

    writer.push_field(recevier_id);

    // An empty list grants access to every method of the receiver.
    if function_call_perm.number_of_method_names == 0 {
        writer.push_field(ReviewField::new(Field {
            name: "Method Names",
            value: "Any method",
        }));
        return;
    }

    writer.push_field(ReviewField::new(Field {
        name: "Total FnCall Methods",
        value: function_call_perm
            .number_of_method_names
            // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
            .numtoa_str(10, &mut field_context.num_buf),
    }));

    let methods_names_fields =
        ReviewField::from_fmt_buffer(&mut function_call_perm.method_names, "Method Names");

    writer.push_field(methods_names_fields);
}
