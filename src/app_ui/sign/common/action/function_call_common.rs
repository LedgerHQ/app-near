use crate::{
    parsing::{self},
    utils::types::elipsis_fields::ElipsisFields,
};
use fmt_buffer::Buffer;
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub method_name_display_buf: [u8; 20],
    pub gas_buf: Buffer<30>,
    pub deposit_buffer: Buffer<30>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            method_name_display_buf: [0u8; 20],
            gas_buf: Buffer::new(),
            deposit_buffer: Buffer::new(),
        }
    }
}
pub fn format<'b, 'a: 'b, const N: usize>(
    func_call_common: &'a parsing::types::FunctionCallCommon,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, N>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Function Call",
    }));

    let method_name = ElipsisFields::from_capped_string(
        &func_call_common.method_name,
        "Method Name",
        &mut field_context.method_name_display_buf,
    );

    writer.push_fields(method_name);

    func_call_common
        .gas
        .display_as_buffer(&mut field_context.gas_buf);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Gas",
        value: field_context.gas_buf.as_str(),
    }));

    func_call_common
        .deposit
        .display_as_buffer(&mut field_context.deposit_buffer);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Deposit",
        value: field_context.deposit_buffer.as_str(),
    }));
}
