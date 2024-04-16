use crate::app_ui::aliases::FnCallHexDisplay;
use crate::app_ui::fields_writer::FieldsWriter;
use crate::utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer};

pub struct FieldsContext {
    pub args_display_buf: EllipsisBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            args_display_buf: EllipsisBuffer::default(),
        }
    }
}

/// action type (1) + Method Name `ElipsisFields` (1-2) +
/// Gas (1) + Deposit (1) + Args Binary `ElipsisFields` (1-2)
const MAX_FIELDS: usize = 7;

pub fn format<'b, 'a: 'b>(
    args: &'b FnCallHexDisplay,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let args_fields =
        ElipsisFields::from_hex_display(args, "Args Binary", &mut field_context.args_display_buf);

    writer.push_fields(args_fields);
}
