use crate::app_ui::sign::display_receiving;
use crate::borsh::BorshDeserialize;
use crate::io::{ErrorKind, Read};
use crate::tx_stream_reader::{HashingStream, SingleTxStream};
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
use crate::utils::{bip32_derive, PathBip32, ALLOWED_PATH_LEN};
use crate::AppSW;
use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
use ledger_device_sdk::io::Comm;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;
use numtoa::NumToA;

const MAX_TRANSACTION_LEN: usize = 534;

// #[derive(Deserialize)]
// pub struct Tx<'a> {
//     #[allow(dead_code)]
//     nonce: u64,
//     pub coin: &'a str,
//     pub value: u64,
//     #[serde(with = "hex::serde")] // Allows JSON deserialization from hex string
//     pub to: [u8; 20],
//     pub memo: &'a str,
// }

pub struct TxContext {
    raw_tx: [u8; MAX_TRANSACTION_LEN], // raw transaction serialized
    raw_tx_len: usize,                 // length of raw transaction
    path: [u32; ALLOWED_PATH_LEN],     // BIP32 path for key derivation
}

// Implement constructor for TxInfo with default values
impl TxContext {
    pub fn new() -> TxContext {
        TxContext {
            raw_tx: [0u8; MAX_TRANSACTION_LEN],
            raw_tx_len: 0,
            path: [0u32; ALLOWED_PATH_LEN],
        }
    }
    // Implement reset for TxInfo
    fn reset(&mut self) {
        self.raw_tx = [0u8; MAX_TRANSACTION_LEN];
        self.raw_tx_len = 0;
        self.path = [0u32; ALLOWED_PATH_LEN];
    }
}

pub struct Signature(pub [u8; 64]);

pub fn handler_sign_tx(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    display_receiving();
    let path = <PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

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
