use crate::{AppSW, parsing::{HashingStream, SingleTxStream, types::DeleteAccount}};
use crate::sign_ui;

pub fn handle(stream: &mut HashingStream<SingleTxStream<'_>>, ordinal_action: u32, total_actions: u32) -> Result<(), AppSW> {
    let mut delete_account = DeleteAccount::new();
    delete_account.deserialize_reader_in_place(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    delete_account.debug_print();
        

    if !sign_ui::action::ui_display_delete_account(&delete_account, ordinal_action + 1, total_actions) {
        return Err(AppSW::Deny);
    }
    Ok(())
    
}
