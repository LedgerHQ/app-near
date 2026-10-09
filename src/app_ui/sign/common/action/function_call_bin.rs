use crate::app_ui::aliases::FnCallHexDisplay;
use crate::app_ui::fields_writer::FieldsWriter;
use crate::utils::types::review_field::ReviewField;

/// action type (1) + Method Name (1) +
/// Gas (1) + Deposit (1) + Args Binary (1)
const MAX_FIELDS: usize = 5;

pub fn format<'b>(args: &'b FnCallHexDisplay, writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>) {
    let args_fields = ReviewField::from_hex_display(args, "Args Binary");

    writer.push_field(args_fields);
}
