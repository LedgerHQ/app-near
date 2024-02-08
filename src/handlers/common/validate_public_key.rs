use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE},
    gadgets::{Field, MultiFieldReview},
};

use crate::{
    utils::{
        crypto::{self, public_key::NoSecpAllowed, PathBip32, PublicKeyBe},
        types::fmt_buffer::FmtBuffer,
    },
    AppSW,
};

pub fn validate(
    tx_public_key: Result<PublicKeyBe, NoSecpAllowed>,
    path: &PathBip32,
) -> Result<(), AppSW> {
    let matching_private_key = {
        let pk = crypto::bip32_derive(&path.0)
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
    let mut key_buf1 = FmtBuffer::<60>::new();
    let mut key_buf2 = FmtBuffer::<60>::new();
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
