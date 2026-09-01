use crate::parsing::types::{AccessKeyPermission, AddKey, FunctionCallPermission, GasKeyInfo};
use crate::{
    AppSW,
    parsing::{HashingStream, SingleTxStream},
};
use crate::{parsing, sign_ui};
use borsh::BorshDeserialize;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let add_key_common = AddKey::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    match add_key_common.access_key.permission {
        AccessKeyPermission::FunctionCall => handle_function_call(&add_key_common, stream, params),
        AccessKeyPermission::FullAccess => {
            if !sign_ui::action::ui_display_add_key_fullaccess(&add_key_common, params) {
                return Err(AppSW::Deny);
            }
            Ok(())
        }
        AccessKeyPermission::GasKeyFunctionCall => {
            handle_gas_key_function_call(&add_key_common, stream, params)
        }
        AccessKeyPermission::GasKeyFullAccess => {
            handle_gas_key_fullaccess(&add_key_common, stream, params)
        }
    }
}

pub fn handle_function_call(
    add_key_common: &parsing::types::AddKey,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut function_call_perm = FunctionCallPermission::new();

    function_call_perm
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;
    if !sign_ui::action::ui_display_add_key_functioncall(
        add_key_common,
        &mut function_call_perm,
        params,
    ) {
        return Err(AppSW::Deny);
    }
    Ok(())
}

pub fn handle_gas_key_function_call(
    add_key_common: &parsing::types::AddKey,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut gas_key_info = GasKeyInfo::new();
    let mut function_call_perm = FunctionCallPermission::new();

    gas_key_info
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;
    function_call_perm
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_add_gas_key_functioncall(
        add_key_common,
        &mut gas_key_info,
        &mut function_call_perm,
        params,
    ) {
        return Err(AppSW::Deny);
    }

    Ok(())
}

pub fn handle_gas_key_fullaccess(
    add_key_common: &parsing::types::AddKey,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut gas_key_info = GasKeyInfo::new();

    gas_key_info
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_add_gas_key_fullaccess(
        add_key_common,
        &mut gas_key_info,
        params,
    ) {
        return Err(AppSW::Deny);
    }

    Ok(())
}
