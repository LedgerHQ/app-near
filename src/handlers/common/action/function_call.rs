use crate::parsing::types::FunctionCallCommon;
use crate::sign_ui;
use crate::utils::types::capped_string::CappedString;
use crate::utils::types::hex_display::HexDisplay;
use crate::{
    parsing::{HashingStream, SingleTxStream},
    AppSW,
};
use borsh::io::ErrorKind;
use borsh::BorshDeserialize;

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

    let representation = match stream
        .reader
        .peek_u8()
        .map_err(|_err| AppSW::TxParsingFail)?
    {
        // '{' char
        Some(123) => {
            let mut args_str: CappedString<200> = CappedString::new();
            match args_str.deserialize_with_bytes_count(stream, args_bytes_count) {
                Err(err) if err.kind() == ErrorKind::InvalidData => {
                    let mut args_bin: HexDisplay<200> = unsafe {
                        core::mem::transmute::<CappedString<200>, HexDisplay<200>>(args_str)
                    };
                    args_bin.reformat();
                    ArgsRepr::BinHex(args_bin)
                }
                Ok(_) => ArgsRepr::String(args_str),
                Err(_err) => {
                    return Err(AppSW::TxParsingFail);
                }
            }
        }
        Some(_first_byte) => {
            let mut args_bin: HexDisplay<200> = HexDisplay::new();
            args_bin
                .deserialize_with_bytes_count(stream, args_bytes_count)
                .map_err(|_err| AppSW::TxParsingFail)?;
            args_bin.reformat();

            ArgsRepr::BinHex(args_bin)
        }
        None => {
            return Err(AppSW::TxParsingFail);
        }
    };
    handle_common(stream, method_name, params, &representation)
}
fn handle_common(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    method_name: CappedString<50>,
    params: ActionParams,
    representation: &ArgsRepr,
) -> Result<(), AppSW> {
    let func_call_common = FunctionCallCommon::deserialize_with_method_name(stream, method_name)
        .map_err(|_err| AppSW::TxParsingFail)?;
    match representation {
        ArgsRepr::BinHex(args_bin) => {
            if !sign_ui::action::ui_display_function_call_bin(&func_call_common, args_bin, params) {
                return Err(AppSW::Deny);
            }
        }
        ArgsRepr::String(args_str) => {
            if !sign_ui::action::ui_display_function_call_str(&func_call_common, args_str, params) {
                return Err(AppSW::Deny);
            }
        }
    }
    Ok(())
}

enum ArgsRepr {
    String(CappedString<200>),
    BinHex(HexDisplay<200>),
}
