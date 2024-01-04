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
use crate::app_ui::sign::display_receiving;
use crate::app_ui::transaction_prefix::ui_display_tx_prefix;
use crate::borsh::BorshDeserialize;
use crate::io::{ErrorKind, Read};
use crate::transaction_prefix::TransactionPrefix;
use crate::tx_stream_reader::{HashingStream, SingleTxStream};
use crate::utils::capped_string::CappedString;
use crate::utils::{bip32_derive, PathBip32};
use crate::AppSW;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

#[allow(unused)]
const MAX_TRANSACTION_LEN: usize = 534;

pub struct Signature(pub [u8; 64]);

fn popup_transaction_prefix(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<(), AppSW> {
    
    let mut tx_prefix: TransactionPrefix = TransactionPrefix { 
        signer_id:  CappedString::new(false),  
        receiver_id: CappedString::new(false),  
        number_of_actions: 0,
    };

    tx_prefix.deserialize_reader_in_place(stream).map_err(|_err| AppSW::TxParsingFail)?;

    if !ui_display_tx_prefix(&tx_prefix) {
        return Err(AppSW::Deny);
    }
    Ok(())
}

pub fn handler_sign_tx(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    display_receiving();
    let path = <PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    popup_transaction_prefix(&mut stream)?;

    let mut buff = [0u8; 50];
    loop {
        let n = stream.read(&mut buff).map_err(|err| {
            if err.kind() == ErrorKind::OutOfMemory {
                return AppSW::TxHashFail;
            }
            AppSW::TxParsingFail
        })?;

        #[cfg(feature = "speculos")]
        debug_print_slice(&buff, n);

        if n == 0 {
            break;
        }
    }
    let digest = stream.finalize()?;

    let private_key = bip32_derive(&path.0);
    let (sig, _len) = private_key.sign(&digest.0).map_err(|_| AppSW::TxSignFail)?;

    Ok(Signature(sig))
}

#[cfg(feature = "speculos")]
pub fn debug_print_slice(slice: &[u8; 50], n: usize) {
    testing::debug_print("debug printing slice hex:\n");

    let mut to_str = [0u8; 100];
    hex::encode_to_slice(&slice[0..n], &mut to_str[..2 * n]).unwrap();

    testing::debug_print(core::str::from_utf8(&to_str[0..2 * n]).unwrap());
    testing::debug_print("\n");
    testing::debug_print("debug printing slice hex finish:\n\n");
}
