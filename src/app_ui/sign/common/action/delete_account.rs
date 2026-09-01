use crate::{
    app_ui::aliases::CappedAccountId,
    parsing,
    utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer},
};

use crate::app_ui::fields_writer::FieldsWriter;

/// Account to delete `EllipsisFields` (1-2) + Beneficiary `EllipsisFields` (1-2)
const MAX_FIELDS: usize = 4;

pub struct FieldsContext {
    pub account_id_to_delete_display_buf: EllipsisBuffer,
    pub beneficiary_display_buf: EllipsisBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            account_id_to_delete_display_buf: EllipsisBuffer::default(),
            beneficiary_display_buf: EllipsisBuffer::default(),
        }
    }
}
pub fn format<'b, 'a: 'b>(
    delete_account: &'a mut parsing::types::DeleteAccount,
    field_context: &'a mut FieldsContext,
    account_id: &'a mut CappedAccountId,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let account_id_to_delete = ElipsisFields::from_capped_string(
        account_id,
        "AccountId",
        &mut field_context.account_id_to_delete_display_buf,
    );
    let beneficiary_id = ElipsisFields::from_capped_string(
        &mut delete_account.beneficiary_id,
        "Beneficiary",
        &mut field_context.beneficiary_display_buf,
    );

    writer.push_fields(account_id_to_delete);
    writer.push_fields(beneficiary_id);
}
