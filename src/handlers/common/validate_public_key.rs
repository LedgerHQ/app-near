use ledger_device_sdk::ecc::Ed25519;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE},
    gadgets::{Field, MultiFieldReview},
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview};
use crate::{
    utils::crypto::{public_key::NoSecpAllowed, PathBip32, PublicKeyBe},
    AppSW,
};
use fmt_buffer::Buffer;

pub fn validate(
    tx_public_key: Result<PublicKeyBe, NoSecpAllowed>,
    path: &PathBip32,
) -> Result<(), AppSW> {
    let matching_private_key = {
        let pk = Ed25519::derive_from_path_slip10(&path.0)
            .public_key()
            .map_err(|_| AppSW::KeyDeriveFail)?;
        PublicKeyBe::from_little_endian(pk)
    };
    let info = match tx_public_key {
        Ok(transaction_field) => {
            if transaction_field == matching_private_key {
                return Ok(());
            }
            KeyMismatchInfo::KeyMismatch {
                transaction_field,
                matching_private_key,
            }
        }
        Err(_err) => KeyMismatchInfo::NoSecpAllowed {
            matching_private_key,
        },
    };
    let _confirm = ui_display(&info)?;

    Err(AppSW::PublicKeyMismatch)
}

enum KeyMismatchInfo {
    NoSecpAllowed {
        matching_private_key: PublicKeyBe,
    },
    KeyMismatch {
        transaction_field: PublicKeyBe,
        matching_private_key: PublicKeyBe,
    },
}

fn ui_display(info: &KeyMismatchInfo) -> Result<bool, AppSW> {
    let mut key_buf1 = Buffer::<60>::new();
    let mut key_buf2 = Buffer::<60>::new();
    match info {
        KeyMismatchInfo::NoSecpAllowed {
            matching_private_key,
        } => {
            key_buf1.write_str("SECP256K1 curve was used");
            matching_private_key.display_str_base58(&mut key_buf2)?;
        }
        KeyMismatchInfo::KeyMismatch {
            transaction_field,
            matching_private_key,
        } => {
            transaction_field.display_str_base58(&mut key_buf1)?;
            matching_private_key.display_str_base58(&mut key_buf2)?;
        }
    }

    let my_fields = [
        Field {
            name: "Transaction Field",
            value: key_buf1.as_str(),
        },
        Field {
            name: "Requested BIP32",
            value: key_buf2.as_str(),
        },
    ];

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            &my_fields,
            &["Pub Key Mismatch"],
            Some(&EYE),
            "Error!",
            Some(&CROSSMARK),
            "Error!",
            Some(&CROSSMARK),
        );
    
        Ok(my_review.show())
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/app_near_16px.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let mut review: NbglReview = NbglReview::new()
            .titles(
                "Review transaction\nto send CRAB",
                "",
                "Sign transaction\nto send CRAB",
            )
            .glyph(&FERRIS);


        Ok(review.show(&my_fields))
    }
}
