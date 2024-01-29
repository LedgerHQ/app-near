use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub fn format(
    _create_account: &parsing::types::CreateAccount,
    writer: &'_ mut FieldsWriter<'_, 1>,
) {
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type",
            value: "Create Account",
        }))
        .unwrap();
}
