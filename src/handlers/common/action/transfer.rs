use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, types::Transfer, HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let transfer = Transfer::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    transfer.debug_print();

    if !sign_ui::action::ui_display_transfer(&transfer, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
