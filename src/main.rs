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

mod utils {
    pub mod crypto {
        pub mod path;
        pub mod public_key;

        pub use path::PathBip32;
        pub use public_key::{bip32_derive, PublicKeyBe};
    }
    pub mod types {
        pub mod capped_string;
        pub mod fmt_buffer;
    }
}
mod app_ui {
    pub mod address;
    pub mod fields_writer;
    pub mod fields_context;
    pub mod menu;
    pub mod sign {
        pub mod action;
        pub mod transaction_prefix;
        pub mod widgets;
    }
}
pub use app_ui::sign as sign_ui;

mod handlers {
    pub mod get_public_key;
    pub mod get_version;
    pub mod get_wallet_id;
    pub mod sign_tx;
}

mod io;
pub mod parsing {
    pub mod borsh;
    pub mod transaction_stream_reader;
    pub mod types {
        pub mod action; 
        pub mod transaction_prefix;

        pub use action::{Action, transfer::Transfer};
        pub use transaction_prefix::TransactionPrefix;
    }

    pub use transaction_stream_reader::{HashingStream, SingleTxStream};
}

use app_ui::menu::ui_menu_main;
use handlers::{get_public_key, get_version, get_wallet_id, sign_tx};
use ledger_device_sdk::io::{ApduHeader, Comm, Event, Reply, StatusWords};
#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;
use parsing::SingleTxStream;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

// CLA (APDU class byte) for all APDUs.
const CLA: u8 = 0x80;
const INS_GET_VERSION: u8 = 6; // Instruction code to get app version from the Ledger
const INS_GET_PUBLIC_KEY: u8 = 4; // Instruction code to get public key
const INS_GET_WALLET_ID: u8 = 0x05; // Get Wallet ID
const INS_SIGN_TRANSACTION: u8 = 2; // Instruction code to sign a transaction on the Ledger

const P1_SIGN_NORMAL: u8 = 0;
const P1_SIGN_LAST_CHUNK: u8 = 0x80;

const P1_GET_PUB_DISPLAY: u8 = 0;
const P1_GET_PUB_SILENT: u8 = 1;

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
    Bip32PathParsingFail = 0xB00B,
    TxHashFinalizeFail = 0xB00C,
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
    GetWalletID,
    GetPubkey { display: bool },
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
            (CLA, INS_GET_VERSION, _, _) => Ok(Instruction::GetVersion),
            (CLA, INS_GET_WALLET_ID, _, _) => Ok(Instruction::GetWalletID),
            (CLA, INS_GET_PUBLIC_KEY, P1_GET_PUB_DISPLAY | P1_GET_PUB_SILENT, _) => {
                Ok(Instruction::GetPubkey {
                    display: value.p1 == P1_GET_PUB_DISPLAY,
                })
            }
            (CLA, INS_SIGN_TRANSACTION, P1_SIGN_NORMAL | P1_SIGN_LAST_CHUNK, _) => {
                Ok(Instruction::SignTx {
                    is_last_chunk: value.p1 == P1_SIGN_LAST_CHUNK,
                })
            }
            (CLA, INS_GET_PUBLIC_KEY | INS_SIGN_TRANSACTION, _, _) => Err(AppSW::WrongP1P2),
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
        Instruction::GetVersion => get_version::handler(comm),
        Instruction::GetWalletID => get_wallet_id::handler(comm),
        Instruction::GetPubkey { display } => get_public_key::handler(comm, display),
        Instruction::SignTx { is_last_chunk } => {
            let stream = SingleTxStream::new(comm, is_last_chunk);
            let signature = sign_tx::handler(stream)?;
            comm.append(&signature.0);
            Ok(())
        }
    }
}
