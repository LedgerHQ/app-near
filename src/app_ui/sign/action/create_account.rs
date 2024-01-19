use crate::parsing;
use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter,
    utils::types::capped_string::ElipsisFields,
};

pub fn format(
    _create_account: &parsing::types::CreateAccount,
    writer: &'_ mut FieldsWriter<'_, 1>,
) {
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: "Create Account",
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }
}
