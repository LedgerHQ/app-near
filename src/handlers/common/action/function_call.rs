use crate::io::ErrorKind;
use crate::parsing::types::FunctionCallCommon;
use crate::sign_ui;
use crate::utils::types::capped_string::CappedString;
use crate::utils::types::hex_display::HexDisplay;
use crate::{
    parsing::{borsh::BorshDeserialize, HashingStream, SingleTxStream},
    AppSW,
};
#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut method_name: CappedString<50> = CappedString::new();

    method_name
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let args_bytes_count: u32 =
        u32::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    match stream
        .reader
        .peek_u8()
        .map_err(|_err| AppSW::TxParsingFail)?
    {
        // '{' char
        Some(123) => {
            let mut args_str: CappedString<500> = CappedString::new();
            match args_str.deserialize_with_bytes_count(stream, args_bytes_count) {
                Err(err) if err.kind() == ErrorKind::InvalidData => {
                    let args_str_mut_ref = &mut args_str;

                    let args_bin_mut_ref = unsafe {
                        core::mem::transmute::<&mut CappedString<500>, &mut HexDisplay<500>>(
                            args_str_mut_ref,
                        )
                    };
                    #[cfg(feature = "speculos")]
                    testing::debug_print(
                        "flow with assuming `args` as binary after parsing error\n",
                    );
                    handle_args_bin(args_bin_mut_ref, stream, method_name, params)
                }
                Ok(_) => {
                    let func_call_common =
                        FunctionCallCommon::deserialize_with_method_name(stream, method_name)
                            .map_err(|_err| AppSW::TxParsingFail)?;
                    #[cfg(feature = "speculos")]
                    debug_print(&args_str, &func_call_common);
                    if !sign_ui::action::ui_display_function_call_str(
                        &func_call_common,
                        &args_str,
                        params,
                    ) {
                        return Err(AppSW::Deny);
                    }
                    Ok(())
                }
                Err(_err) => {
                    return Err(AppSW::TxParsingFail);
                }
            }
        }
        Some(_first_byte) => {
            let mut args_bin: HexDisplay<500> = HexDisplay::new();
            args_bin
                .deserialize_with_bytes_count(stream, args_bytes_count)
                .map_err(|_err| AppSW::TxParsingFail)?;
            handle_args_bin(&mut args_bin, stream, method_name, params)
        }
        None => Err(AppSW::TxParsingFail),
    }
}

fn handle_args_bin(
    args_bin: &mut HexDisplay<500>,
    stream: &mut HashingStream<SingleTxStream<'_>>,
    method_name: CappedString<50>,
    params: ActionParams,
) -> Result<(), AppSW> {
    args_bin.reformat();
    let func_call_common = FunctionCallCommon::deserialize_with_method_name(stream, method_name)
        .map_err(|_err| AppSW::TxParsingFail)?;
    if !sign_ui::action::ui_display_function_call_bin(&func_call_common, &args_bin, params) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
#[cfg(feature = "speculos")]
pub fn debug_print(args_str: &CappedString<500>, func_call_common: &FunctionCallCommon) {
    func_call_common.debug_print();
    use numtoa::NumToA;

    let mut numtoa_buf = [0u8; 40];

    testing::debug_print("debug printing function call args str  action:\n");
    testing::debug_print("size of self: \n");
    testing::debug_print(core::mem::size_of_val(args_str).numtoa_str(10, &mut numtoa_buf));
    testing::debug_print("\n");
    testing::debug_print("debug printing function call args str  action finish:\n");
}
