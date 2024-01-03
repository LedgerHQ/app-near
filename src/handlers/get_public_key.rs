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

use crate::app_ui::address::ui_display_pk;
use crate::utils::{bip32_derive, PathBip32, PublicKeyBe};
use crate::AppSW;
use ledger_device_sdk::ecc::{Ed25519, SeedDerive};
use ledger_device_sdk::io::Comm;

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

pub fn handler_get_public_key(comm: &mut Comm, display: bool) -> Result<(), AppSW> {
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    let path = PathBip32::parse(data).map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let pk = bip32_derive(&path.0)
        .public_key()
        .map_err(|_| AppSW::KeyDeriveFail)?;

    let pk = PublicKeyBe::from_little_endian(pk);

    #[cfg(feature = "speculos")]
    pk.debug_print()?;

    if display {
        if !ui_display_pk(&pk)? {
            return Err(AppSW::Deny);
        }
    }

    comm.append(&pk.0);

    Ok(())
}
