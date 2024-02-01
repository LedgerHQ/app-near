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
// use crate::app_ui::sign::ui_display_tx;
use crate::io::{ErrorKind, Read};
use crate::parsing;
use crate::parsing::borsh::BorshDeserialize;
use crate::parsing::types::Action;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::sign_ui;
use crate::utils::crypto;
use crate::utils::types::capped_string::CappedString;
use crate::AppSW;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

pub struct Signature(pub [u8; 64]);

pub mod add_key;
pub mod create_account;
pub mod delegate;
pub mod delete_account;
pub mod delete_key;
pub mod deploy_contract;
pub mod function_call;
pub mod stake;
pub mod transfer;

fn popup_transaction_prefix(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<u32, AppSW> {
    let mut tx_prefix = parsing::types::TransactionPrefix {
        signer_id: CappedString::new(),
        receiver_id: CappedString::new(),
        number_of_actions: 0,
    };

    tx_prefix
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::transaction_prefix::ui_display(&tx_prefix) {
        return Err(AppSW::Deny);
    }
    Ok(tx_prefix.number_of_actions)
}

fn popup_action(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
) -> Result<(), AppSW> {
    let action = Action::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    match action {
        Action::Transfer => transfer::handle(stream, ordinal_action, total_actions),
        Action::CreateAccount => create_account::handle(stream, ordinal_action, total_actions),
        Action::DeleteAccount => delete_account::handle(stream, ordinal_action, total_actions),
        Action::DeleteKey => delete_key::handle(stream, ordinal_action, total_actions),
        Action::Stake => stake::handle(stream, ordinal_action, total_actions),
        Action::AddKey => add_key::handle(stream, ordinal_action, total_actions),
        Action::DeployContract => deploy_contract::handle(stream, ordinal_action, total_actions),
        Action::FunctionCall => function_call::handle(stream, ordinal_action, total_actions),
        Action::Delegate => delegate::handle(stream, ordinal_action, total_actions),
    }
}

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    let number_of_actions = popup_transaction_prefix(&mut stream)?;

    for i in 0..number_of_actions {
        sign_ui::widgets::display_receiving();
        popup_action(&mut stream, i, number_of_actions)?;
    }

    // test no redundant bytes left in stream
    let mut buf = [0u8; 1];
    match stream.read_exact(&mut buf) {
        Err(f) if f.kind() == ErrorKind::UnexpectedEof => { // ok
        }
        _ => return Err(AppSW::TxParsingFail),
    }

    let digest = stream.finalize()?;

    let private_key = crypto::bip32_derive(&path.0);
    let (sig, _len) = private_key.sign(&digest.0).map_err(|_| AppSW::TxSignFail)?;

    Ok(Signature(sig))
}

#[cfg(feature = "speculos")]
#[allow(unused)]
pub fn debug_print_slice(slice: &[u8; 50], n: usize) {
    testing::debug_print("debug printing slice hex:\n");

    let mut to_str = [0u8; 100];
    hex::encode_to_slice(&slice[0..n], &mut to_str[..2 * n]).unwrap();

    testing::debug_print(core::str::from_utf8(&to_str[0..2 * n]).unwrap());
    testing::debug_print("\n");
    testing::debug_print("debug printing slice hex finish:\n\n");
}
