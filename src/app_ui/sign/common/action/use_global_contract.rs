use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1) + Contract SHA256 or AccountId (1)
const MAX_FIELDS: usize = 2;

pub fn format<'b>(
    use_global_contract: &'b mut parsing::types::UseGlobalContract,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Use Global Contract",
    }));

    writer.push_fields(ElipsisFields::one(match use_global_contract {
        parsing::types::UseGlobalContract::CodeHash(code_hash) => Field {
            name: "Global Contract SHA256",
            value: code_hash.as_str(),
        },

        parsing::types::UseGlobalContract::AccountId(account_id) => Field {
            name: "Global Contract AccountId",
            value: account_id.as_str(),
        },
    }));
}
