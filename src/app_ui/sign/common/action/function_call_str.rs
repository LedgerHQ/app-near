use crate::app_ui::aliases::FnCallCappedString;

use crate::app_ui::fields_writer::FieldsWriter;
use crate::utils::types::review_field::ReviewField;

/// action type (1) + Method Name (1) +
/// Gas (1) + Deposit (1) + Args String (1)
const MAX_FIELDS: usize = 5;

pub fn format<'b>(args: &'b mut FnCallCappedString, writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>) {
    let args_fields = ReviewField::from_capped_string(args, "Args String");

    writer.push_field(args_fields);
}
