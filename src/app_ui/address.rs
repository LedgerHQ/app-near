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

use crate::AppSW;
use crate::utils::crypto;
use crate::utils::types::fmt_buffer::FmtBuffer;
use ledger_device_sdk::ui::bitmaps::{CROSSMARK, EYE, VALIDATE_14};
use ledger_device_sdk::ui::gadgets::{Field, MultiFieldReview};

pub fn ui_display_pk_base58(public_key: &crypto::PublicKeyBe) -> Result<bool, AppSW> {
    let mut out_buf = FmtBuffer::<60>::new();
    public_key.display_str_base58(&mut out_buf)?;

    let my_field = [Field {
        name: "Public Key",
        value: out_buf.as_str(),
    }];

    let my_review = MultiFieldReview::new(
        &my_field,
        &["Confirm Address"],
        Some(&EYE),
        "Approve",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    Ok(my_review.show())
}

pub fn ui_display_hex(public_key: &crypto::PublicKeyBe) -> Result<bool, AppSW> {
    let mut out_buf = [0u8; 64];

    let my_field = [Field {
        name: "Wallet ID",
        value: public_key.display_str_hex(&mut out_buf),
    }];

    let my_review = MultiFieldReview::new(
        &my_field,
        &["Confirm Address"],
        Some(&EYE),
        "Approve",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    Ok(my_review.show())
}
