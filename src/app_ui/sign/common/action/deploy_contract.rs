use crate::{parsing, utils::types::elipsis_fields::ElipsisFields};
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

pub fn format<'b>(
    deploy_contract: &'b parsing::types::DeployContract,
    writer: &'_ mut FieldsWriter<'b, 2>,
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
