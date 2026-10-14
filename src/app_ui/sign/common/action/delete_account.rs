use crate::{parsing, utils::types::review_field::ReviewField};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1) + Beneficiary (1)
const MAX_FIELDS: usize = 2;

pub fn format<'b, 'a: 'b>(
    delete_account: &'a mut parsing::types::DeleteAccount,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_field(ReviewField::new(Field {
        name: "Action type",
        value: "Delete Account",
    }));

    let beneficiary_id =
        ReviewField::from_capped_string(&mut delete_account.beneficiary_id, "Beneficiary");
    writer.push_field(beneficiary_id);
}
