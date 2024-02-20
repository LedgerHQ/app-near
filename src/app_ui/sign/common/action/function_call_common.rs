use crate::{
    parsing::{self},
    utils::types::{elipsis_fields::ElipsisFields, fmt_buffer::FmtBuffer},
};
use ledger_device_sdk::ui::gadgets::Field;
use numtoa::NumToA;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub method_name_display_buf: [u8; 20],
    pub gas_buf: [u8; 20],
    pub deposit_buffer: FmtBuffer<30>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            method_name_display_buf: [0u8; 20],
            gas_buf: [0u8; 20],
            deposit_buffer: FmtBuffer::new(),
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

    func_call_common
        .deposit
        .display_as_buffer(&mut field_context.deposit_buffer);
    writer
        .push_fields(field_context.deposit_buffer.ui_field("Deposit"))
        .unwrap();
}
