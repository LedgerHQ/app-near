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
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::bitmaps::WARNING;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::clear_screen;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::layout::{Layout, Location, StringPlace};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::screen_util::screen_update;

/// the constants and their special meaning were copied from
/// [ledger_device_sdk::ui::gadgets::display_pending_review]
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
pub fn display_receiving() {
    clear_screen();

    // Add icon and text to match the C SDK equivalent.
    if cfg!(target_os = "nanos") {
        "Receiving".place(Location::Custom(2), Layout::Centered, true);
        "Transaction...".place(Location::Custom(14), Layout::Centered, true);
    } else {
        WARNING.draw(57, 10);
        "Receiving".place(Location::Custom(28), Layout::Centered, true);
        "Transaction...".place(Location::Custom(42), Layout::Centered, true);
    }

    screen_update();
}

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub fn display_receiving() {
    // TODO: implement loader indicator for stax and flex
}
