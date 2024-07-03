use crate::sign_ui;
use crate::{
    parsing::{types::Transfer, HashingStream, SingleTxStream},
    AppSW,
};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let transfer = Transfer::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    sign_ui::action::ui_display_my_tx()

    // if !sign_ui::action::ui_display_transfer(&transfer, params) {
    //     return Err(AppSW::Deny);
    // }
    // Ok(())
}
