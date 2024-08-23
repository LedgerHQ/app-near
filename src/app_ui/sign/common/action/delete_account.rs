use crate::{
    parsing,
    utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer},
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1) + Beneficiary `EllipsisFields` (1-2)
const MAX_FIELDS: usize = 3;

pub struct FieldsContext {
    pub beneficiary_display_buf: EllipsisBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            beneficiary_display_buf: EllipsisBuffer::default(),
        }
    }
}
pub fn format<'b, 'a: 'b>(
    delete_account: &'a mut parsing::types::DeleteAccount,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
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
