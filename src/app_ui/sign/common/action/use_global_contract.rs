use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// Contract SHA256 or AccountId (1)
const MAX_FIELDS: usize = 1;

pub fn format<'b>(
    use_global_contract: &'b mut parsing::types::GlobalContractIdentifier,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(match use_global_contract {
        parsing::types::GlobalContractIdentifier::CodeHash(code_hash) => Field {
            name: "Contract SHA256",
            value: code_hash.as_str(),
        },

        parsing::types::GlobalContractIdentifier::AccountId(account_id) => Field {
            name: "Contract AccountId",
            value: account_id.as_str(),
        },
    }));
}
