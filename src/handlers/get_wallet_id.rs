use crate::app_ui::address;
use crate::utils::crypto;
use crate::AppSW;
use ledger_device_sdk::io::Comm;

pub fn handler(comm: &mut Comm) -> Result<(), AppSW> {
    let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    let path = crypto::PathBip32::parse(data).map_err(|_| AppSW::Bip32PathParsingFail)?;

    let pk = crypto::bip32_derive(&path.0)
        .public_key()
        .map_err(|_| AppSW::KeyDeriveFail)?;

    let pk = crypto::PublicKeyBe::from_little_endian(pk);

    if !address::ui_display_hex(&pk)? {
        return Err(AppSW::Deny);
    }

    comm.append(&pk.0);

    Ok(())
}
