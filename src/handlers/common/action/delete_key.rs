use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, types::DeleteKey, HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let delete_key = DeleteKey::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_delete_key(&delete_key, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
