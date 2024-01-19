use crate::parsing;
use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter,
    utils::types::capped_string::ElipsisFields,
};

pub fn format<'b, 'a: 'b>(
    delete_account: &'a parsing::types::DeleteAccount,
    writer: &'_ mut FieldsWriter<'b, 3>,
) {
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: "Delete Account",
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }

    let beneficiary_id = delete_account.beneficiary_id.ui_fields("Beneficiary");
    match writer.push_fields(beneficiary_id) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }
}
