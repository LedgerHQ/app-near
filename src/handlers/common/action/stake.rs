use crate::sign_ui;
use crate::{
    parsing::{types::Stake, HashingStream, SingleTxStream},
    AppSW,
};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let stake = Stake::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_stake(&stake, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
