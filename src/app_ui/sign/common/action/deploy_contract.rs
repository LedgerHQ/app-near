use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1) + Contract SHA256 (1)
const MAX_FIELDS: usize = 2;

pub fn format<'b>(
    deploy_contract: &'b parsing::types::DeployContract,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Deploy Contract",
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Contract SHA256",
        value: deploy_contract.code_sha256.as_str(),
    }));
}
