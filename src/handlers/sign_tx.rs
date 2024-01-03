use crate::app_ui::sign::display_receiving;
use crate::borsh::BorshDeserialize;
use crate::io::{Read, ErrorKind};
use crate::tx_stream_reader::{SingleTxStream, HashingStream, Sha256Digest};
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
use crate::utils::{PathBip32, ALLOWED_PATH_LEN};
use crate::AppSW;
use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
use ledger_device_sdk::io::Comm;
use ledger_secure_sdk_sys::{
    cx_hash_no_throw, cx_hash_t, cx_keccak_init_no_throw, cx_sha3_t, CX_LAST, CX_OK,
};

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

pub fn handler_sign_tx(mut stream: SingleTxStream<'_>) -> Result<Sha256Digest, AppSW> {
    display_receiving();
    let path = <PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    let mut buff = [0u8; 50];
    loop {
        let n = stream
            .read(&mut buff)
            .map_err(|err| {
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
    #[cfg(feature = "speculos")]
    testing::debug_print("computed hash:\n");
    (&mut buff[0..32]).copy_from_slice(&digest.0);
    #[cfg(feature = "speculos")]
    debug_print_slice(&buff, 32);

    Ok(digest)
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

fn compute_signature_and_append(comm: &mut Comm, ctx: &mut TxContext) -> Result<(), AppSW> {
    let mut keccak256: cx_sha3_t = Default::default();
    let mut message_hash: [u8; 32] = [0u8; 32];

    unsafe {
        if cx_keccak_init_no_throw(&mut keccak256, 256) != CX_OK {
            return Err(AppSW::TxHashFail);
        }
        if cx_hash_no_throw(
            &mut keccak256.header as *mut cx_hash_t,
            CX_LAST,
            ctx.raw_tx.as_ptr(),
            ctx.raw_tx_len,
            message_hash.as_mut_ptr(),
            message_hash.len(),
        ) != CX_OK
        {
            return Err(AppSW::TxHashFail);
        }
    }

    let (sig, siglen, parity) = Secp256k1::derive_from_path(&ctx.path[..])
        .deterministic_sign(&message_hash)
        .map_err(|_| AppSW::TxSignFail)?;
    comm.append(&[siglen as u8]);
    comm.append(&sig[..siglen as usize]);
    comm.append(&[parity as u8]);
    Ok(())
}
