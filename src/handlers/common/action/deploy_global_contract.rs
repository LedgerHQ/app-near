use crate::parsing::types::DeployGlobalContract;
use crate::sign_ui;
use crate::{
    parsing::{HashingStream, SingleTxStream},
    AppSW,
};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let deploy_contract =
        DeployGlobalContract::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_deploy_global_contract(&deploy_contract, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
