use crate::app_ui::aliases::{FnCallCappedString, FnCallHexDisplay};
use crate::parsing::types::FunctionCallCommon;
use crate::sign_ui;
use crate::utils::types::capped_string::CappedString;
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

    handle_from_method_name(stream, method_name, params)
}

/// Continue FunctionCall handling after the method_name has already been consumed
/// from the stream. Used by sign_tx.rs when the method name was pre-read for
/// staking-method detection.
pub fn handle_from_method_name(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    method_name: CappedString<50>,
    params: ActionParams,
) -> Result<(), AppSW> {
    // The method name is never signed unless it can be displayed in full,
    // with each byte shown as its own glyph.
    if method_name.truncated() || !method_name.is_printable_ascii() {
        return Err(AppSW::TxParsingFail);
    }

    let args_bytes_count: u32 =
        u32::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    let mut representation = match stream
        .reader
        .peek_u8()
        .map_err(|_err| AppSW::TxParsingFail)?
    {
        // '{' char
        Some(123) => {
            let mut args_str: FnCallCappedString = FnCallCappedString::new();
            match args_str.deserialize_with_bytes_count(stream, args_bytes_count) {
                Err(err) if err.kind() == ErrorKind::InvalidData => ArgsRepr::hex(args_str),
                Ok(_) => ArgsRepr::from_valid_utf8(args_str),
                Err(_err) => {
                    return Err(AppSW::TxParsingFail);
                }
            }
        }
        Some(_first_byte) => {
            let mut args_bin: FnCallHexDisplay = FnCallHexDisplay::new();
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
    // The args are never signed unless they can be displayed in full.
    if representation.truncated() {
        return Err(AppSW::TxParsingFail);
    }
    handle_common(stream, method_name, params, &mut representation)
}
fn handle_common(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    method_name: CappedString<50>,
    params: ActionParams,
    representation: &mut ArgsRepr,
) -> Result<(), AppSW> {
    let mut func_call_common =
        FunctionCallCommon::deserialize_with_method_name(stream, method_name)
            .map_err(|_err| AppSW::TxParsingFail)?;
    match representation {
        ArgsRepr::BinHex(args_bin) => {
            if !sign_ui::action::ui_display_function_call_bin(
                &mut func_call_common,
                args_bin,
                params,
            ) {
                return Err(AppSW::Deny);
            }
        }
        ArgsRepr::String(args_str) => {
            if !sign_ui::action::ui_display_function_call_str(
                &mut func_call_common,
                args_str,
                params,
            ) {
                return Err(AppSW::Deny);
            }
        }
    }
    Ok(())
}

enum ArgsRepr {
    String(FnCallCappedString),
    BinHex(FnCallHexDisplay),
}

impl ArgsRepr {
    fn hex(args_str: FnCallCappedString) -> Self {
        let mut args_bin: FnCallHexDisplay = args_str.into();
        args_bin.reformat();
        ArgsRepr::BinHex(args_bin)
    }

    /// Valid UTF-8 args are shown as a string only when they are printable ASCII;
    /// otherwise in hex, as the device draws every other byte as the same square glyph.
    fn from_valid_utf8(args_str: FnCallCappedString) -> Self {
        if args_str.is_printable_ascii() {
            ArgsRepr::String(args_str)
        } else {
            ArgsRepr::hex(args_str)
        }
    }

    fn truncated(&self) -> bool {
        match self {
            ArgsRepr::String(args_str) => args_str.truncated(),
            ArgsRepr::BinHex(args_bin) => args_bin.truncated(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArgsRepr;
    use crate::app_ui::aliases::{FnCallCappedString, FnCallHexDisplay};

    fn string_args(len: usize) -> ArgsRepr {
        let bytes = [b'x'; 201];
        let mut args = FnCallCappedString::new();
        assert!(args
            .deserialize_with_bytes_count(&mut &bytes[..len], len as u32)
            .is_ok());
        ArgsRepr::String(args)
    }

    fn binary_args(len: usize) -> ArgsRepr {
        let bytes = [0x80u8; 101];
        let mut args = FnCallHexDisplay::new();
        assert!(args
            .deserialize_with_bytes_count(&mut &bytes[..len], len as u32)
            .is_ok());
        args.reformat();
        ArgsRepr::BinHex(args)
    }

    #[test_case]
    fn string_args_at_display_limit_are_not_truncated() {
        assert!(!string_args(200).truncated());
    }

    #[test_case]
    fn string_args_one_over_display_limit_are_truncated() {
        assert!(string_args(201).truncated());
    }

    #[test_case]
    fn binary_args_at_display_limit_are_not_truncated() {
        assert!(!binary_args(100).truncated());
    }

    #[test_case]
    fn binary_args_one_over_display_limit_are_truncated() {
        assert!(binary_args(101).truncated());
    }

    fn valid_utf8_args(bytes: &[u8]) -> ArgsRepr {
        let mut args = FnCallCappedString::new();
        assert!(args
            .deserialize_with_bytes_count(&mut &bytes[..], bytes.len() as u32)
            .is_ok());
        ArgsRepr::from_valid_utf8(args)
    }

    #[test_case]
    fn printable_ascii_args_are_shown_as_string() {
        assert!(matches!(
            valid_utf8_args(br#"{"a":"b c~"}"#),
            ArgsRepr::String(_)
        ));
    }

    #[test_case]
    fn non_printable_args_are_shown_as_the_signed_bytes_in_hex() {
        for bytes in [
            "{\"a\":\"\u{e9}\"}".as_bytes(),
            b"{\"a\":\"\n\"}",
            b"{\"a\":\"\x7f\"}",
        ] {
            match valid_utf8_args(bytes) {
                ArgsRepr::BinHex(args_bin) => {
                    let mut expected = [0u8; 24];
                    let expected = &mut expected[..bytes.len() * 2];
                    hex::encode_to_slice(bytes, expected).unwrap();
                    assert_eq!(args_bin.as_str().as_bytes(), &expected[..]);
                }
                ArgsRepr::String(_) => panic!("non-printable args shown as a string"),
            }
        }
    }

    #[test_case]
    fn non_printable_args_follow_the_binary_display_limit() {
        // two-byte UTF-8 characters: 100 bytes fit the hex review, 102 do not
        let mut bytes = [0u8; 102];
        for pair in bytes.chunks_mut(2) {
            pair.copy_from_slice("\u{e9}".as_bytes());
        }
        assert!(!valid_utf8_args(&bytes[..100]).truncated());
        assert!(valid_utf8_args(&bytes[..102]).truncated());
    }
}
