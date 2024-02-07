use ledger_device_sdk::io::Event;
use ledger_secure_sdk_sys::buttons::ButtonEvent;

use crate::{
    parsing::{HashingStream, SingleTxStream},
    sign_ui,
    utils::crypto::{self, public_key::NoSecpAllowed, PathBip32, PublicKeyBe},
    AppSW, Instruction,
};

pub fn validate(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    tx_public_key: Result<PublicKeyBe, NoSecpAllowed>,
    path: &PathBip32,
) -> Result<(), AppSW> {
    match tx_public_key {
        Ok(tx_public_key) => {
            let matching_private_key = {
                let pk = crypto::bip32_derive(&path.0)
                    .public_key()
                    .map_err(|_| AppSW::KeyDeriveFail)?;
                PublicKeyBe::from_little_endian(pk)
            };
            if tx_public_key == matching_private_key {
                return Ok(());
            }
        }
        Err(_err) => {}
    }
    stream.reader.comm.reply(AppSW::PublicKeyMismatch);
    sign_ui::widgets::public_key_mismatch();

    loop {
        match stream.reader.comm.next_event::<Instruction>() {
            Event::Button(button) => match button {
                ButtonEvent::BothButtonsRelease => {
                    return Err(AppSW::PublicKeyMismatch);
                }
                _ => {
                    // ignore all other button presses
                }
            },
            _ => (),
        };
    }
}
