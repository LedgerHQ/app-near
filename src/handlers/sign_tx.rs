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
use crate::parsing;
use crate::parsing::borsh::BorshDeserialize;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::sign_ui;
use crate::utils::crypto;
use crate::AppSW;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use crate::handlers::common::action::{handle_action, ActionParams};

use super::common::finalize_sign::{Signature, self};

fn handle_transaction_prefix(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<u32, AppSW> {
    let mut tx_prefix = parsing::types::transaction::prefix::Prefix::new();

    tx_prefix
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::transaction::prefix::ui_display(&tx_prefix) {
        return Err(AppSW::Deny);
    }
    Ok(tx_prefix.number_of_actions)
}

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    let number_of_actions = handle_transaction_prefix(&mut stream)?;

    for i in 0..number_of_actions {
        sign_ui::widgets::display_receiving();
        let params = ActionParams {
            ordinal_action: i + 1,
            total_actions: number_of_actions,
            is_nested_delegate: false,
        };
        handle_action(&mut stream, params)?;
    }

    finalize_sign::end(&mut stream, &path)
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
