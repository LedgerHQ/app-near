use crate::{
    parsing::{self, types::ONE_NEAR},
    utils::types::elipsis_fields::ElipsisFields,
};
use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub method_name_display_buf: [u8; 20],
    pub gas_buf: [u8; 20],
    pub float_buffer: dtoa::Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            method_name_display_buf: [0u8; 20],
            gas_buf: [0u8; 20],
            float_buffer: dtoa::Buffer::new(),
        }
    }
}
pub fn format<'b, 'a: 'b, const N: usize>(
    func_call_common: &'a parsing::types::FunctionCallCommon,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, N>,
) {
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type",
            value: "Function Call",
        }))
        .unwrap();

    let method_name = func_call_common
        .method_name
        .ui_fields("Method Name", &mut field_context.method_name_display_buf);

    writer.push_fields(method_name).unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Gas",
            value: func_call_common
                .gas
                .numtoa_str(10, &mut field_context.gas_buf),
        }))
        .unwrap();

    let deposit_amount = (func_call_common.deposit as f64) / (ONE_NEAR as f64);
    let printed_amount = field_context.float_buffer.format(deposit_amount);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Deposit (NEAR)",
            value: printed_amount,
        }))
        .unwrap();
}
