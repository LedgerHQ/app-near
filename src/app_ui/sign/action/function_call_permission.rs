use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing::{self, types::action::ONE_NEAR},
    utils::types::capped_string::ElipsisFields,
};

use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

pub struct FieldsContext {
    pub num_buf: [u8; 10],
    pub float_buffer: dtoa::Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf: [0u8; 10],
            float_buffer: dtoa::Buffer::new(),
        }
    }
}

pub fn format<'b, 'a: 'b>(
    function_call_perm: &'a parsing::types::FunctionCallPermission,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 10>,
) {
    let allowance = match function_call_perm.allowance {
        Some(allowance) => {
            let allowance = (allowance as f64) / (ONE_NEAR as f64);
            field_context.float_buffer.format(allowance)
        }
        None => "Unlimited",
    };
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "FuncCall Allowance:",
            value: allowance,
        }))
        .unwrap();

    let recevier_id = function_call_perm.receiver_id.ui_fields("FnCall Receiver:");

    writer.push_fields(recevier_id).unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Number of methods:",
            value: function_call_perm
                .number_of_method_names
                .numtoa_str(10, &mut field_context.num_buf),
        }))
        .unwrap();

    let methods_names_fields = function_call_perm.method_names.ui_fields("FnCall Methods:");

    writer.push_fields(methods_names_fields).unwrap();
}
