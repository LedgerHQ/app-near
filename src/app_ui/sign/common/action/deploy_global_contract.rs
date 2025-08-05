use crate::{
    parsing::{self, types::common::action::deploy_global_contract::GlobalContractDeployMode},
    utils::types::elipsis_fields::ElipsisFields,
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::Field;

use crate::app_ui::fields_writer::FieldsWriter;

/// action type (1) + Contract SHA256 (1) + deploy type (1)
const MAX_FIELDS: usize = 3;

pub fn format<'b>(
    deploy_global_contract: &'b parsing::types::DeployGlobalContract,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Deploy Global Contract",
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Contract SHA256",
        value: deploy_global_contract.code_sha256.as_str(),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Can be used",
        value: match deploy_global_contract.deploy_mode {
            GlobalContractDeployMode::CodeHash => "by Code Hash",
            GlobalContractDeployMode::AccountId => {
                "by AccountId (auto-upgrade everywhere it is used)"
            }
        },
    }))
}
