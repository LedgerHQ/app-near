use crate::app_ui::aliases::FnCallHexDisplay;
use crate::app_ui::fields_writer::FieldsWriter;
use crate::sign_ui::blind_signing::{BlindSigned, SizeBuffer};
use crate::utils::types::review_field::ReviewField;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

/// action type (1) + Method Name (1) +
/// Gas (1) + Deposit (1) + Args Binary (1) + Args size (0-1, signed blind)
const MAX_FIELDS: usize = 6;

pub fn format<'b>(
    args: &'b FnCallHexDisplay,
    blind: Option<&BlindSigned>,
    size_buf: &'b mut SizeBuffer,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    match blind {
        None => writer.push_field(ReviewField::from_hex_display(args, "Args Binary")),
        Some(blind) => {
            writer.push_field(ReviewField::from_hex_display_blind(
                args,
                "Args Binary",
                blind,
            ));
            writer.push_field(ReviewField::new(Field {
                name: "Args size",
                value: blind.size_description(args.shown_bytes(), size_buf),
            }));
        }
    }
}
