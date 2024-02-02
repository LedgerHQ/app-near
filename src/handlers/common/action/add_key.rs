use crate::parsing::types::AddKey;
use crate::parsing::types::{AccessKeyPermission, FunctionCallPermission};
use crate::{parsing, sign_ui};
use crate::{
    parsing::{borsh::BorshDeserialize, HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let add_key_common = AddKey::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    add_key_common.debug_print();

    match add_key_common.access_key.permission {
        AccessKeyPermission::FunctionCall => handle_function_call(&add_key_common, stream, params),
        AccessKeyPermission::FullAccess => {
            if !sign_ui::action::ui_display_add_key_fullaccess(&add_key_common, params) {
                return Err(AppSW::Deny);
            }
            Ok(())
        }
    }
}

pub fn handle_function_call(
    add_key_common: &parsing::types::AddKey,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut function_call_perm = FunctionCallPermission::new();

    #[cfg(feature = "speculos")]
    function_call_perm.debug_print();

    function_call_perm
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;
    if !sign_ui::action::ui_display_add_key_functioncall(
        add_key_common,
        &function_call_perm,
        params,
    ) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
