use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, types::Stake, HashingStream, SingleTxStream},
    AppSW,
};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
) -> Result<(), AppSW> {
    let stake = Stake::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    stake.debug_print();

    if !sign_ui::action::ui_display_stake(&stake, ordinal_action + 1, total_actions) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
