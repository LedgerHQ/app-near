use crate::app_ui::aliases::FnCallCappedString;

use crate::app_ui::fields_writer::FieldsWriter;
use crate::sign_ui::blind_signing::{BlindSigned, SizeBuffer};
use crate::utils::types::review_field::ReviewField;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

/// action type (1) + Method Name (1) +
/// Gas (1) + Deposit (1) + Args String (1) + Args size (0-1, signed blind)
const MAX_FIELDS: usize = 6;

pub fn format<'b>(
    args: &'b mut FnCallCappedString,
    blind: Option<&BlindSigned>,
    size_buf: &'b mut SizeBuffer,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    match blind {
        None => writer.push_field(ReviewField::from_capped_string(args, "Args String")),
        Some(blind) => {
            let shown_bytes = args.shown_bytes();
            writer.push_field(ReviewField::from_capped_string_blind(
                args,
                "Args String",
                blind,
            ));
            writer.push_field(ReviewField::new(Field {
                name: "Args size",
                value: blind.size_description(shown_bytes, size_buf),
            }));
        }
    }
}
