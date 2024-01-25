use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, types::Transfer, HashingStream, SingleTxStream},
    AppSW,
};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
) -> Result<(), AppSW> {
    let transfer = Transfer::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    transfer.debug_print();

    if !sign_ui::action::ui_display_transfer(&transfer, ordinal_action + 1, total_actions) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
