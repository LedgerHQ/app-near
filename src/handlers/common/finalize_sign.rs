use crate::{
    parsing::{HashingStream, SingleTxStream},
    utils::crypto::PathBip32,
    AppSW,
};
use borsh::io::{ErrorKind, Read};
use ledger_device_sdk::ecc::Ed25519;

pub struct Signature(pub [u8; 64]);

pub fn end(
    mut stream: HashingStream<SingleTxStream<'_>>,
    path: &PathBip32,
) -> Result<Signature, AppSW> {
    // test no redundant bytes left in stream
    let mut buf = [0u8; 1];
    match stream.read_exact(&mut buf) {
        Err(f) if f.kind() == ErrorKind::UnexpectedEof => { // ok
        }
        _ => return Err(AppSW::TxParsingFail),
    }

    let digest = stream.finalize()?;

    let private_key = Ed25519::derive_from_path_slip10(&path.0);
    let (sig, _len) = private_key.sign(&digest.0).map_err(|_| AppSW::TxSignFail)?;

    Ok(Signature(sig))
}
