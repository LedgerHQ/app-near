use crate::{
    app_ui::aliases::CappedAccountId, parsing, utils::types::elipsis_fields::ElipsisFields,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// AccountId to create (1)
const MAX_FIELDS: usize = 1;

pub fn format<'b, 'a: 'b>(
    _create_account: &'a parsing::types::CreateAccount,
    account_id: &'a mut CappedAccountId,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    // TODO: use buffer instead of mutable capped account_id
    writer.push_fields(ElipsisFields::one(Field {
        name: "AccountId",
        value: account_id.as_str(),
    }));
}
