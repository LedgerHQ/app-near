use crate::parsing::types::action::add_key::{AccessKeyPermission, FunctionCallPermission};
use crate::parsing::types::AddKey;
use crate::{parsing, sign_ui};
use crate::{
    parsing::{borsh::BorshDeserialize, HashingStream, SingleTxStream},
    AppSW,
};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
) -> Result<(), AppSW> {
    let add_key_common = AddKey::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    add_key_common.debug_print();

    match add_key_common.access_key.permission {
        AccessKeyPermission::FunctionCall => {
            handle_function_call(&add_key_common, stream, ordinal_action, total_actions)
        }
        AccessKeyPermission::FullAccess => {
            if !sign_ui::action::ui_display_add_key_fullaccess(
                &add_key_common,
                ordinal_action + 1,
                total_actions,
            ) {
                return Err(AppSW::Deny);
            }
            Ok(())
        }
    }
}

pub fn handle_function_call(
    add_key_common: &parsing::types::AddKey,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
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
        ordinal_action + 1,
        total_actions,
    ) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
