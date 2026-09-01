use crate::parsing::types::GlobalContractIdentifier;
use crate::sign_ui;
use crate::{
    AppSW,
    parsing::{HashingStream, SingleTxStream},
};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut global_contract = GlobalContractIdentifier::deserialize_reader(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_use_global_contract(&mut global_contract, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
