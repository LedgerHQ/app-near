use crate::parsing::types::UseGlobalContract;
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
    let mut use_global_contract =
        UseGlobalContract::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_use_global_contract(&mut use_global_contract, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
