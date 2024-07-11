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

use include_gif::include_gif;
use ledger_device_sdk::io::{Comm, Event};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::bitmaps::{Glyph, BACK, CERTIFICATE, DASHBOARD_X};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::gadgets::{EventOrPageIndex, MultiPageMenu, Page};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{NbglGlyph, NbglHomeAndSettings};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use crate::settings::Settings;

use crate::Instruction;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
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

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
pub fn ui_menu_main(comm: &mut Comm) -> Event<Instruction> {
    #[cfg(target_os = "nanos")]
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("icons/app_near_16px.gif"));
    #[cfg(target_os = "nanosplus")]
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("icons/app_near_14px.gif"));
    #[cfg(target_os = "nanox")]
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("icons/app_near_14px.gif"));
    let pages = [
        // The from trait allows to create different styles of pages
        // without having to use the new() function.
        &Page::from((["Near app", "is ready"], &APP_ICON)),
        &Page::from((["Version", env!("CARGO_PKG_VERSION")], true)),
        &Page::from(("About", &CERTIFICATE)),
        &Page::from(("Quit", &DASHBOARD_X)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(2) => return ui_about_menu(comm),
            EventOrPageIndex::Index(3) => ledger_device_sdk::exit_app(0),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub fn ui_menu_main(_: &mut Comm) -> Event<Instruction> {
    const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

    let settings_strings = [["Display Memo", "Allow display of transaction memo."]];
    let mut settings: Settings = Default::default();

    NbglHomeAndSettings::new()
        .glyph(&FERRIS)
        .settings(settings.get_mut_ref(), &settings_strings)
        .infos(
            "Near",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .show()
}