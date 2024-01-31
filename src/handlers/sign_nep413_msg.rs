use crate::sign_ui;
use crate::{
    io::{ErrorKind, Read},
    parsing::{
        borsh::BorshDeserialize, types::message_discriminant::MessageDiscriminant, HashingStream,
        SingleTxStream,
    },
    utils::crypto,
    AppSW,
};

use super::sign_tx::Signature;

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    let msg_discriminant = MessageDiscriminant::new_off_chain(413).unwrap();

    let prefix_bytes = msg_discriminant.borsh_serialize();

    stream
        .feed_slice(&prefix_bytes)
        .map_err(|_err| AppSW::TxParsingFail)?;

    read_till_end(&mut stream)?;
    // test no redundant bytes left in stream
    let mut buf = [0u8; 1];
    match stream.read_exact(&mut buf) {
        Err(f) if f.kind() == ErrorKind::UnexpectedEof => { // ok
        }
        _ => return Err(AppSW::TxParsingFail),
    }

    let digest = stream.finalize()?;

    let private_key = crypto::bip32_derive(&path.0);
    let (sig, _len) = private_key.sign(&digest.0).map_err(|_| AppSW::TxSignFail)?;

    Ok(Signature(sig))
}

pub fn read_till_end<R: Read>(stream: &mut R) -> Result<(), AppSW> {
    let mut buff = [0u8; 50];
    loop {
        let n = stream.read(&mut buff).map_err(|err| {
            if err.kind() == ErrorKind::OutOfMemory {
                return AppSW::TxHashFail;
            }
            AppSW::TxParsingFail
        })?;

        #[cfg(feature = "speculos")]
        debug_print_slice(&buff, n);

        if n == 0 {
            break;
        }
    }
    Ok(())
}
