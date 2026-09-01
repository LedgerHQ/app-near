use crate::app_ui::aliases::CappedAccountId;
use crate::sign_ui;
use crate::{
    AppSW,
    parsing::{HashingStream, SingleTxStream, types::CreateAccount},
};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
    account_id: &CappedAccountId,
) -> Result<(), AppSW> {
    let create_account =
        CreateAccount::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_create_account(&create_account, &mut account_id.clone(), params)
    {
        return Err(AppSW::Deny);
    }
    Ok(())
}
