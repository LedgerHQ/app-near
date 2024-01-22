use crate::{AppSW, parsing::{HashingStream, SingleTxStream, types::DeleteKey, borsh::BorshDeserialize}};
use crate::sign_ui;

pub fn handle(stream: &mut HashingStream<SingleTxStream<'_>>, ordinal_action: u32, total_actions: u32) -> Result<(), AppSW> {
    let delete_key = DeleteKey::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    delete_key.debug_print();
        

    if !sign_ui::action::ui_display_delete_key(&delete_key, ordinal_action + 1, total_actions) {
        return Err(AppSW::Deny);
    }
    Ok(())
    
}
