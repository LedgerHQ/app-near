use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, types::CreateAccount, HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let create_account =
        CreateAccount::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_create_account(&create_account, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
