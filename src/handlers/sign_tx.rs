/*****************************************************************************
 *   Ledger App Near Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/
use crate::app_ui::aliases::CappedAccountId;
use crate::parsing;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::sign_ui;
use crate::utils::crypto::{self, PublicKeyBe};
use crate::utils::types::capped_string::CappedString;
use crate::AppSW;
use borsh::BorshDeserialize;

use crate::handlers::common::action::{handle_action, ActionParams};
use crate::parsing::types::{Action, Stake};

use super::common::finalize_sign::{self, Signature};
use super::common::validate_public_key;

use ledger_device_sdk::libcall::swap::CreateTxParams;
use ledger_device_sdk::log;

enum CombinedStakeFlow {
    NativeStake,
    StakingFunctionCall,
}

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    let mut stream = HashingStream::new(stream)?;

    // Parse prefix (always needed for key validation).
    let mut tx_prefix = parsing::types::transaction::prefix::Prefix::new();
    tx_prefix
        .deserialize_reader_in_place(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let tx_public_key_prevalidation = PublicKeyBe::try_from(tx_prefix.public_key);
    validate_public_key::validate(tx_public_key_prevalidation, &path)?;
    let number_of_actions = tx_prefix.number_of_actions;

    // Only skip the generic prefix screen when the current APDU already proves
    // this is a stake/unstake single-action flow. Otherwise keep the original
    // prefix-first behavior so APDU timing for legacy tests stays unchanged.
    // `None` means this single action is not a staking flow; keep legacy behavior.
    let combined_flow = if number_of_actions == 1 {
        detect_combined_stake_flow(&mut stream)?
    } else {
        None
    };

    if let Some(combined_flow) = combined_flow {
        sign_ui::widgets::display_receiving();

        match combined_flow {
            CombinedStakeFlow::NativeStake => {
                let action =
                    Action::deserialize_reader(&mut stream).map_err(|_err| AppSW::TxParsingFail)?;
                if action != Action::Stake {
                    return Err(AppSW::TxParsingFail);
                }
                let stake =
                    Stake::deserialize_reader(&mut stream).map_err(|_err| AppSW::TxParsingFail)?;
                if !sign_ui::action::ui_display_stake_combined(&mut tx_prefix, &stake) {
                    return Err(AppSW::Deny);
                }
            }
            CombinedStakeFlow::StakingFunctionCall => {
                let action =
                    Action::deserialize_reader(&mut stream).map_err(|_err| AppSW::TxParsingFail)?;
                if action != Action::FunctionCall {
                    return Err(AppSW::TxParsingFail);
                }
                let mut method_name: CappedString<50> = CappedString::new();
                method_name
                    .deserialize_reader_in_place(&mut stream)
                    .map_err(|_err| AppSW::TxParsingFail)?;

                if !sign_ui::action::is_staking_method(method_name.as_str()) {
                    return Err(AppSW::TxParsingFail);
                }
                handle_fn_call_stake_with_prefix(
                    &mut stream,
                    &mut tx_prefix,
                    method_name.as_str(),
                )?;
            }
        }
    } else {
        if !sign_ui::transaction::prefix::ui_display(&mut tx_prefix) {
            return Err(AppSW::Deny);
        }
        for i in 0..number_of_actions {
            sign_ui::widgets::display_receiving();
            let params = ActionParams {
                ordinal_action: i + 1,
                total_actions: number_of_actions,
                is_nested_delegate: false,
            };
            handle_action(&mut stream, params)?;
        }
    }

    finalize_sign::end(stream, &path)
}

fn detect_combined_stake_flow(
    stream: &mut HashingStream<SingleTxStream<'_>>,
) -> Result<Option<CombinedStakeFlow>, AppSW> {
    let bytes = stream
        .reader
        .remaining_in_current_chunk()
        .map_err(|_err| AppSW::TxParsingFail)?;

    let Some((&tag, rest)) = bytes.split_first() else {
        return Ok(None);
    };

    match tag {
        4 => Ok(Some(CombinedStakeFlow::NativeStake)),
        2 => {
            if rest.len() < 4 {
                return Ok(None);
            }

            let method_len = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
            let Some(method_end) = 4usize.checked_add(method_len) else {
                return Err(AppSW::TxParsingFail);
            };
            let Some(method_bytes) = rest.get(4..method_end) else {
                return Ok(None);
            };

            let method_name = core::str::from_utf8(method_bytes).ok();
            if let Some(method_name) = method_name {
                if sign_ui::action::is_staking_method(method_name) {
                    return Ok(Some(CombinedStakeFlow::StakingFunctionCall));
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Parse the remaining fields of a staking FunctionCall (args, gas, deposit)
/// and display the combined prefix + action review screen.
fn handle_fn_call_stake_with_prefix(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    tx_prefix: &mut parsing::types::transaction::prefix::Prefix,
    method_name: &str,
) -> Result<(), AppSW> {
    use crate::app_ui::aliases::FnCallCappedString;
    use borsh::io::ErrorKind;

    // Read args bytes count then the args themselves.
    let args_bytes_count: u32 =
        u32::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    let mut args_str = FnCallCappedString::new();
    let args_valid = match args_str.deserialize_with_bytes_count(stream, args_bytes_count) {
        Ok(_) => true,
        // InvalidData means bytes were consumed but not valid UTF-8 — carry on.
        Err(ref e) if e.kind() == ErrorKind::InvalidData => false,
        Err(_) => return Err(AppSW::TxParsingFail),
    };

    log::debug!(
        "sign_tx.rs: handle_fn_call_stake_with_prefix - arg_str:{}\n",
        args_str.as_str()
    );

    let args = sign_ui::action::decode_stake_args(method_name, args_str, args_valid);
    // The args are never signed unless they can be displayed in full.
    if args.truncated() {
        return Err(AppSW::TxParsingFail);
    }

    // Read gas and deposit.
    let gas: near_gas::NearGas =
        BorshDeserialize::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;
    let deposit: near_token::NearToken =
        BorshDeserialize::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_fn_call_stake(tx_prefix, method_name, args, gas, deposit) {
        return Err(AppSW::Deny);
    }
    Ok(())
}

/// Sign handler for the swap transaction
/// This handler is called when the user wants to sign a swap transaction
/// The swap transaction is a transfer transaction with a specific amount and destination address
/// The handler checks the transaction parameters and signs the transaction
pub fn swap_handler(
    mut stream: SingleTxStream<'_>,
    tx_params: &CreateTxParams,
) -> Result<Signature, AppSW> {
    log::debug!("sign_tx.rs: swap_handler()\n");

    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    log::debug!("sign_tx.rs: path computed\n");

    // Get the public key from the transaction
    let mut stream = HashingStream::new(stream)?;
    let mut tx_prefix = parsing::types::transaction::prefix::Prefix::new();
    tx_prefix
        .deserialize_reader_in_place(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;
    let tx_public_key = match PublicKeyBe::try_from(tx_prefix.public_key) {
        Ok(tx_public_key) => tx_public_key,
        Err(_) => return Err(AppSW::PublicKeyMismatch),
    };

    // Derive the public key from the path and compare it with the transaction public key
    let dpath_public_key = {
        let pk = ledger_device_sdk::ecc::Ed25519::derive_from_path_slip10(&path.0)
            .public_key()
            .map_err(|_| AppSW::KeyDeriveFail)?;
        PublicKeyBe::from_little_endian(pk)
    };

    if tx_public_key != dpath_public_key {
        return Err(AppSW::PublicKeyMismatch);
    }

    if !is_implicit_account_of(&mut tx_prefix.signer_id, &dpath_public_key) {
        log::debug!("sign_tx.rs: signer_id does not match the swap source account\n");
        return Err(AppSW::TxSignFail);
    }

    // Check nb of actions (shall be == 1 == Transfer in swap context)
    if tx_prefix.number_of_actions != 1 {
        return Err(AppSW::TxSignFail);
    }
    let action = crate::parsing::types::Action::deserialize_reader(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;
    if action != crate::parsing::types::Action::Transfer {
        return Err(AppSW::TxSignFail);
    }

    // Check the tx parameters match with the ones previously validated in Exchange app (tx_params)
    let transfer = crate::parsing::types::Transfer::deserialize_reader(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let amount_match = near_token::NearToken::from_yoctonear(u128::from_be_bytes(tx_params.amount))
        == transfer.deposit;
    if !amount_match {
        log::debug!("sign_tx.rs: amounts do not not match\n");
        return Err(AppSW::TxSignFail);
    }

    let dest_address_match = tx_prefix.receiver_id.as_str()
        == core::str::from_utf8(tx_params.dest_address[..tx_params.dest_address_len].as_ref())
            .unwrap();
    if !dest_address_match {
        log::debug!("sign_tx.rs: receiver_id does not match with dest_address\n",);
        return Err(AppSW::TxSignFail);
    }

    finalize_sign::end(stream, &path)
}

/// The swap source checked by `swap::check_address` is the implicit account of the
/// derived key: a swap transaction must be sent from that exact account, whose id is
/// the 64 lowercase hex characters of the public key.
fn is_implicit_account_of(signer_id: &mut CappedAccountId, public_key: &PublicKeyBe) -> bool {
    let mut implicit_account = [0u8; 64];
    let implicit_account = public_key.display_str_hex(&mut implicit_account);
    !signer_id.truncated() && signer_id.as_str() == implicit_account
}

#[cfg(test)]
mod tests {
    use super::is_implicit_account_of;
    use crate::app_ui::aliases::CappedAccountId;
    use crate::utils::crypto::PublicKeyBe;

    // the key of the implicit account the app-exchange swap fixtures sign from
    const PUBLIC_KEY: [u8; 32] = [
        0xc4, 0xf5, 0x94, 0x1e, 0x81, 0xe0, 0x71, 0xc2, 0xfd, 0x1d, 0xae, 0x2e, 0x71, 0xfd, 0x3d,
        0x85, 0x9d, 0x46, 0x24, 0x84, 0x39, 0x1d, 0x9a, 0x90, 0xbf, 0x21, 0x92, 0x11, 0xdc, 0xbb,
        0x32, 0x0f,
    ];
    const IMPLICIT_ACCOUNT: &str =
        "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f";

    fn account(id: &str) -> CappedAccountId {
        let mut account = CappedAccountId::new();
        assert!(account
            .deserialize_with_bytes_count(&mut id.as_bytes(), id.len() as u32)
            .is_ok());
        account
    }

    #[test_case]
    fn accepts_the_implicit_account_of_the_key() {
        let key = PublicKeyBe(PUBLIC_KEY);
        assert!(is_implicit_account_of(&mut account(IMPLICIT_ACCOUNT), &key));
    }

    #[test_case]
    fn rejects_a_named_account() {
        let key = PublicKeyBe(PUBLIC_KEY);
        assert!(!is_implicit_account_of(
            &mut account("blablatest.testnet"),
            &key
        ));
    }

    #[test_case]
    fn rejects_the_implicit_account_of_another_key() {
        let mut other_key = PUBLIC_KEY;
        other_key[31] ^= 1;
        let key = PublicKeyBe(other_key);
        assert!(!is_implicit_account_of(
            &mut account(IMPLICIT_ACCOUNT),
            &key
        ));
    }

    #[test_case]
    fn rejects_the_implicit_account_in_uppercase() {
        let key = PublicKeyBe(PUBLIC_KEY);
        let mut upper = [0u8; 64];
        upper.copy_from_slice(IMPLICIT_ACCOUNT.as_bytes());
        upper.make_ascii_uppercase();
        let upper = core::str::from_utf8(&upper).unwrap();
        assert!(!is_implicit_account_of(&mut account(upper), &key));
    }

    #[test_case]
    fn rejects_a_prefix_or_extension_of_the_implicit_account() {
        let key = PublicKeyBe(PUBLIC_KEY);
        assert!(!is_implicit_account_of(
            &mut account(&IMPLICIT_ACCOUNT[..63]),
            &key
        ));

        // 65 bytes do not fit the account id buffer: kept as a truncated 64-byte prefix
        let mut longer = CappedAccountId::new();
        let mut bytes = [b'0'; 65];
        bytes[..64].copy_from_slice(IMPLICIT_ACCOUNT.as_bytes());
        assert!(longer
            .deserialize_with_bytes_count(&mut &bytes[..], 65)
            .is_ok());
        assert!(!is_implicit_account_of(&mut longer, &key));
    }
}
