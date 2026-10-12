//! Settings the user changes on the device, kept in NVM.

use ledger_device_sdk::nvm::{AtomicStorage, SingleStorage};
use ledger_device_sdk::NVMData;

/// One byte per setting, the layout `NbglHomeAndSettings::settings` reads and toggles.
pub const SETTINGS_SIZE: usize = 1;

/// Index of the blind-signing setting.
const BLIND_SIGNING: usize = 0;

/// Values of a setting byte, as NBGL's `OFF_STATE` / `ON_STATE`.
const DISABLED: u8 = 0;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
const ENABLED: u8 = 1;

const DEFAULTS: [u8; SETTINGS_SIZE] = [DISABLED; SETTINGS_SIZE];

#[link_section = ".nvm_data"]
static mut SETTINGS: NVMData<AtomicStorage<[u8; SETTINGS_SIZE]>> =
    NVMData::new(AtomicStorage::new(&DEFAULTS));

pub fn storage() -> &'static mut AtomicStorage<[u8; SETTINGS_SIZE]> {
    // SAFETY: the app is single-threaded, and no reference returned here is kept
    // across a call that takes another one.
    unsafe { (*core::ptr::addr_of_mut!(SETTINGS)).get_mut() }
}

/// Stores the defaults when the storage holds no value yet (first launch, or the zeroed
/// NVM Speculos loads), so that the reads below find one.
pub fn init() {
    storage().get_or_init(&DEFAULTS);
}

/// `true` when the user allowed signing values that cannot be displayed in full.
/// Disabled by default.
pub fn blind_signing_enabled() -> bool {
    storage().get_ref()[BLIND_SIGNING] != DISABLED
}

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
pub fn toggle_blind_signing() {
    let mut values = *storage().get_ref();
    values[BLIND_SIGNING] = if values[BLIND_SIGNING] == DISABLED {
        ENABLED
    } else {
        DISABLED
    };
    storage().update(&values);
}
