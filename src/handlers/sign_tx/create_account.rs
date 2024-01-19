use crate::{AppSW, parsing::{HashingStream, SingleTxStream, types::CreateAccount, borsh::BorshDeserialize}};
use crate::sign_ui;

pub fn handle(stream: &mut HashingStream<SingleTxStream<'_>>, ordinal_action: u32, total_actions: u32) -> Result<(), AppSW> {
    let create_account = CreateAccount::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    create_account.debug_print();
        

    if !sign_ui::action::ui_display_create_account(&create_account, ordinal_action + 1, total_actions) {
        return Err(AppSW::Deny);
    }
    Ok(())
    
}
