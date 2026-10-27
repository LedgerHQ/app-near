//! Signing a value the review cannot display in full.
//!
//! Such a value is rejected unless the user enabled blind signing in the settings, and
//! then accepted a warning before the review that shows it truncated.

use fmt_buffer::Buffer;
use numtoa::NumToA;

use crate::app_ui::aliases::U32Buffer;
use crate::{settings, AppSW};

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use crate::app_ui::logo::WARNING_GLYPH;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{NbglChoice, NbglStatus};

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, VALIDATE_14, WARNING},
    gadgets::{clear_screen, MultiFieldReview, Page},
};

/// Issued only once the user, with blind signing enabled, accepted the warning that the
/// review that follows does not show a value in full. A review shows a truncated value
/// only against it.
pub struct BlindSigned {
    total_bytes: u32,
}

/// Large enough for `"<u32> bytes, first <u32> shown"`.
pub type SizeBuffer = Buffer<40>;

impl BlindSigned {
    /// Describes how much of the value the review shows, for a field next to it.
    pub fn size_description<'a>(&self, shown_bytes: usize, buf: &'a mut SizeBuffer) -> &'a str {
        describe_size(self.total_bytes, shown_bytes, buf)
    }
}

/// `"<total_bytes> bytes, first <shown_bytes> shown"`, for a review that shows several
/// truncated values against one approval.
pub fn describe_size(total_bytes: u32, shown_bytes: usize, buf: &mut SizeBuffer) -> &str {
    let mut num_out = U32Buffer::default();
    // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
    buf.write_str(total_bytes.numtoa_str(10, &mut num_out));
    buf.write_str(" bytes, first ");
    buf.write_str((shown_bytes as u32).numtoa_str(10, &mut num_out));
    buf.write_str(" shown");
    buf.as_str()
}

/// Asks to sign a value of `total_bytes` that the review cannot display in full.
///
/// Fails with [`AppSW::BlindSigningDisabled`] when blind signing is disabled in the
/// settings, and with [`AppSW::Deny`] when the user turns back at the warning.
pub fn approve(total_bytes: u32) -> Result<BlindSigned, AppSW> {
    if !settings::blind_signing_enabled() {
        ui_blind_signing_disabled();
        return Err(AppSW::BlindSigningDisabled);
    }
    if !ui_blind_signing_warning() {
        return Err(AppSW::Deny);
    }
    Ok(BlindSigned { total_bytes })
}

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
fn ui_blind_signing_disabled() {
    let go_to_settings = NbglChoice::new().glyph(&WARNING_GLYPH).show(
        "This transaction cannot be clear-signed",
        "Enable blind signing in the settings to sign this transaction.",
        "Go to settings",
        "Reject transaction",
    );
    if go_to_settings {
        crate::app_ui::menu::open_settings_next();
    } else {
        NbglStatus::new().text("Transaction rejected").show(false);
    }
}

#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
fn ui_blind_signing_warning() -> bool {
    let back_to_safety = NbglChoice::new().glyph(&WARNING_GLYPH).show(
        "Blind signing ahead",
        "This transaction's details are not fully verifiable. If you sign it, you could lose all your assets.",
        "Back to safety",
        "Continue anyway",
    );
    if back_to_safety {
        NbglStatus::new().text("Transaction rejected").show(false);
    }
    !back_to_safety
}

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
fn ui_blind_signing_disabled() {
    clear_screen();
    Page::from((["Blind signing must be", "enabled in Settings"], &CROSSMARK)).place_and_wait();
}

#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
fn ui_blind_signing_warning() -> bool {
    MultiFieldReview::new(
        &[],
        &["Blind signing", "ahead"],
        Some(&WARNING),
        "Continue",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    )
    .show()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn describes_the_shown_part_of_the_value() {
        let blind = BlindSigned { total_bytes: 612 };
        let mut buf = SizeBuffer::new();
        assert_eq!(
            blind.size_description(200, &mut buf),
            "612 bytes, first 200 shown"
        );
    }

    #[test_case]
    fn the_description_fits_the_largest_sizes() {
        let blind = BlindSigned {
            total_bytes: u32::MAX,
        };
        let mut buf = SizeBuffer::new();
        blind.size_description(u32::MAX as usize, &mut buf);
        assert!(!buf.truncated());
    }
}
