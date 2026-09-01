use crate::app_ui::aliases::CappedAccountId;
use crate::sign_ui;
use crate::{
    AppSW,
    parsing::{HashingStream, SingleTxStream, types::DeleteAccount},
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
    account_id: &CappedAccountId,
) -> Result<(), AppSW> {
    let mut delete_account = DeleteAccount::new();
    delete_account
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_delete_account(
        &mut delete_account,
        &mut account_id.clone(),
        params,
    ) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
