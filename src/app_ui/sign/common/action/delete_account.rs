use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub struct FieldsContext {
    pub beneficiary_display_buf: [u8; 20],
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            beneficiary_display_buf: [0u8; 20],
        }
    }
}
pub fn format<'b, 'a: 'b>(
    delete_account: &'a mut parsing::types::DeleteAccount,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 3>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Delete Account",
    }));

    let beneficiary_id = ElipsisFields::from_capped_string(
        &mut delete_account.beneficiary_id,
        "Beneficiary",
        &mut field_context.beneficiary_display_buf,
    );
    writer.push_fields(beneficiary_id);
}
