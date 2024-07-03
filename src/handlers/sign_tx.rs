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
use crate::parsing;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::sign_ui;
use crate::utils::crypto::public_key::NoSecpAllowed;
use crate::utils::crypto::{self, PublicKeyBe};
use crate::AppSW;
use borsh::BorshDeserialize;

use crate::handlers::common::action::{handle_action, ActionParams};

use super::common::finalize_sign::{self, Signature};
use super::common::validate_public_key;

struct PrefixResult {
    number_of_actions: u32,
    tx_public_key_prevalidation: Result<PublicKeyBe, NoSecpAllowed>,
}

fn handle_transaction_prefix(
    stream: &mut HashingStream<SingleTxStream<'_>>,
) -> Result<PrefixResult, AppSW> {
    let mut tx_prefix = parsing::types::transaction::prefix::Prefix::new();

    tx_prefix
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::transaction::prefix::ui_display(&mut tx_prefix) {
        return Err(AppSW::Deny);
    }
    let tx_public_key = PublicKeyBe::try_from(tx_prefix.public_key);

    Ok(PrefixResult {
        number_of_actions: tx_prefix.number_of_actions,
        tx_public_key_prevalidation: tx_public_key,
    })
}

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    let mut stream = HashingStream::new(stream)?;

    let PrefixResult {
        number_of_actions,
        tx_public_key_prevalidation,
    } = handle_transaction_prefix(&mut stream)?;
    validate_public_key::validate(tx_public_key_prevalidation, &path)?;

    for i in 0..number_of_actions {
        #[cfg(not(any(target_os = "stax", target_os = "flex")))]
        sign_ui::widgets::display_receiving();
        let params = ActionParams {
            ordinal_action: i + 1,
            total_actions: number_of_actions,
            is_nested_delegate: false,
        };
        handle_action(&mut stream, params)?;
    }

    finalize_sign::end(stream, &path)
}
