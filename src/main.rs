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

#![no_std]
#![no_main]

mod utils;
mod app_ui {
    pub mod address;
    pub mod menu;
    pub mod sign;
}
mod handlers {
    pub mod get_public_key;
    pub mod get_version;
    pub mod sign_tx;
}

mod borsh;
mod io;
mod tx_stream_reader;

use app_ui::menu::ui_menu_main;
use handlers::{
    get_public_key::handler_get_public_key,
    get_version::handler_get_version,
    sign_tx::{handler_sign_tx, TxContext},
};
use ledger_device_sdk::io::{ApduHeader, Comm, Event, Reply, StatusWords};
#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;
use tx_stream_reader::SingleTxStream;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

// CLA (APDU class byte) for all APDUs.
const CLA: u8 = 0x80;
const INS_GET_VERSION: u8 = 6; // Instruction code to get app version from the Ledger
const INS_GET_PUBLIC_KEY: u8 = 4; // Instruction code to get public key
const INS_SIGN_TRANSACTION: u8 = 2; // Instruction code to sign a transaction on the Ledger

const P1_SIGN_NORMAL: u8 = 0;
const P1_SIGN_NORMAL_LAST_CHUNK: u8 = 0x80;

// Application status words.
#[repr(u16)]
pub enum AppSW {
    Deny = 0x6985,
    WrongP1P2 = 0x6A86,
    InsNotSupported = 0x6D00,
    ClaNotSupported = 0x6E00,
    TxDisplayFail = 0xB001,
    AddrDisplayFail = 0xB002,
    TxWrongLength = 0xB004,
    TxParsingFail = 0xB005,
    TxHashFail = 0xB006,
    TxSignFail = 0xB008,
    KeyDeriveFail = 0xB009,
    VersionParsingFail = 0xB00A,
    WrongApduLength = StatusWords::BadLen as u16,
}

impl From<AppSW> for Reply {
    fn from(sw: AppSW) -> Reply {
        Reply(sw as u16)
    }
}

/// Possible input commands received through APDUs.
pub enum Instruction {
    GetVersion,
    // GetAppName,
    GetPubkey,
    SignTx { is_last_chunk: bool },
}

/// APDU parsing logic.
///
/// Parses CLA, INS, P1 and P2 bytes to build an [`Ins`]. P1 and P2 are translated to strongly
/// typed variables depending on the APDU instruction code. Invalid CLA, INS, P1 or P2 values
/// result in errors with a status word, which are automatically sent to the host by the SDK.
///
/// This design allows a clear separation of the APDU parsing logic and commands handling.
impl TryFrom<ApduHeader> for Instruction {
    type Error = AppSW;

    fn try_from(value: ApduHeader) -> Result<Self, Self::Error> {
        match (value.cla, value.ins, value.p1, value.p2) {
            (CLA, INS_GET_VERSION, 0, 0) => Ok(Instruction::GetVersion),
            (CLA, INS_GET_PUBLIC_KEY, 0, _) => Ok(Instruction::GetPubkey),
            (CLA, INS_SIGN_TRANSACTION, P1_SIGN_NORMAL | P1_SIGN_NORMAL_LAST_CHUNK, _) => {
                Ok(Instruction::SignTx {
                    is_last_chunk: value.p1 == P1_SIGN_NORMAL_LAST_CHUNK,
                })
            }
            // (CLA, 4, 0, 0) => Ok(Instruction::GetAppName),
            // (CLA, 6, P1_SIGN_TX_START, P2_SIGN_TX_MORE)
            // | (CLA, 6, 1..=P1_SIGN_TX_MAX, P2_SIGN_TX_LAST | P2_SIGN_TX_MORE) => {
            //     Ok(Instruction::SignTx {
            //         chunk: value.p1,
            //         more: value.p2 == P2_SIGN_TX_MORE,
            //     })
            // }
            // (CLA, 3..=6, _, _) => Err(AppSW::WrongP1P2),
            (CLA, _, _, _) => Err(AppSW::InsNotSupported),
            (_, _, _, _) => Err(AppSW::ClaNotSupported),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    #[cfg(feature = "speculos")]
    testing::debug_print("enter `sample_main` fn\n\n");
    let mut comm = Comm::new();

    loop {
        // Wait for either a specific button push to exit the app
        // or an APDU command
        if let Event::Command(ins) = ui_menu_main(&mut comm) {
            match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            }
        }
    }
}

fn handle_apdu(comm: &mut Comm, ins: Instruction) -> Result<(), AppSW> {
    match ins {
        Instruction::GetVersion => handler_get_version(comm),
        Instruction::GetPubkey => handler_get_public_key(comm, true),
        Instruction::SignTx { is_last_chunk } => {
            let stream = SingleTxStream::new(comm, is_last_chunk);
            handler_sign_tx(stream)
        }
    }
}
