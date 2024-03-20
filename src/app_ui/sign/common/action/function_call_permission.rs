use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::elipsis_fields::ElipsisFields,
};
use fmt_buffer::Buffer;

use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: [u8; 10],
    pub receiver_display_buf: [u8; 20],
    pub method_names_display_buf: [u8; 20],
    pub allowance_buffer: Buffer<30>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: [0u8; 10],
            receiver_display_buf: [0u8; 20],
            method_names_display_buf: [0u8; 20],
            allowance_buffer: Buffer::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    function_call_perm: &'a mut parsing::types::FunctionCallPermission,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 10>,
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
