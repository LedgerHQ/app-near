use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1)
const MAX_FIELDS: usize = 1;
pub fn format(
    _create_account: &parsing::types::CreateAccount,
    writer: &'_ mut FieldsWriter<'_, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Create Account",
    }));
}
