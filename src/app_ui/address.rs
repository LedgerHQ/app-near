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
use crate::utils::PublicKeyBe;
use crate::utils::fmt_buffer::TruncatingFmtBuffer;
use ledger_device_sdk::ui::bitmaps::{CROSSMARK, EYE, VALIDATE_14};
use ledger_device_sdk::ui::gadgets::{Field, MultiFieldReview};

pub fn ui_display_pk(public_key: &PublicKeyBe) -> Result<bool, AppSW> {
    let mut out_buf = TruncatingFmtBuffer::<60>::new();
    public_key.display_str(&mut out_buf)?;

    let my_field = [Field {
        name: "Address",
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
