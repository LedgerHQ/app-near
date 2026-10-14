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

use crate::app_ui::logo::NEAR_LOGO;
use crate::settings;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use core::sync::atomic::{AtomicBool, Ordering};
use ledger_device_sdk::io::{Comm, Event};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{NbglHomeAndSettings, PageIndex};
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::bitmaps::{BACK, CERTIFICATE, COGGLE, DASHBOARD_X};
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::{EventOrPageIndex, MultiPageMenu, Page};

use crate::Instruction;

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
fn ui_about_menu(comm: &mut Comm) -> Event<Instruction> {
    let pages = [
        &Page::from((["NEAR", "(c) 2024 Ledger"], true)),
        &Page::from(("Back", &BACK)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(1) => return ui_menu_main(comm),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
fn ui_settings_menu(comm: &mut Comm) -> Event<Instruction> {
    loop {
        let blind_signing = if settings::blind_signing_enabled() {
            "Enabled"
        } else {
            "Disabled"
        };
        let pages = [
            &Page::from((["Blind signing", blind_signing], true)),
            &Page::from(("Back", &BACK)),
        ];
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(0) => settings::toggle_blind_signing(),
            EventOrPageIndex::Index(1) => return ui_menu_main(comm),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
pub fn ui_menu_main(comm: &mut Comm) -> Event<Instruction> {
    let pages = [
        // The from trait allows to create different styles of pages
        // without having to use the new() function.
        &Page::from((["Near app", "is ready"], &NEAR_LOGO)),
        &Page::from((["Version", env!("CARGO_PKG_VERSION")], true)),
        &Page::from(("Settings", &COGGLE)),
        &Page::from(("About", &CERTIFICATE)),
        &Page::from(("Quit", &DASHBOARD_X)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(2) => return ui_settings_menu(comm),
            EventOrPageIndex::Index(3) => return ui_about_menu(comm),
            EventOrPageIndex::Index(4) => ledger_device_sdk::exit_app(0),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

// `show` is deprecated since SDK 1.37 in favour of `show_and_return`; migrating changes when the
// home screen is redrawn after each flow, so it is kept out of this change.
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
#[allow(deprecated)]
pub fn ui_menu_main(_: &mut Comm) -> Event<Instruction> {
    let mut home = NbglHomeAndSettings::new()
        .glyph(&NEAR_LOGO)
        .infos("NEAR", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"))
        .settings(
            settings::storage(),
            &[[
                "Blind signing",
                "Enable signing function calls whose arguments cannot be fully displayed.",
            ]],
        );
    if OPEN_SETTINGS.swap(false, Ordering::Relaxed) {
        home.set_start_page(PageIndex::Settings(0));
    }
    home.show()
}

/// Set when the user asks to go to the settings from a transaction that needs them.
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
static OPEN_SETTINGS: AtomicBool = AtomicBool::new(false);

/// Opens the settings instead of the home page the next time the menu is shown.
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
pub fn open_settings_next() {
    OPEN_SETTINGS.store(true, Ordering::Relaxed);
}
