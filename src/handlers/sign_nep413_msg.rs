use crate::parsing::types::common::message_discriminant::NEP_413_SIGN_MESSAGE;
use crate::parsing::types::nep413::payload::Payload;
use crate::sign_ui;
use crate::utils::types::capped_string::CappedString;
use crate::{
    parsing::{types::MessageDiscriminant, HashingStream, SingleTxStream},
    utils::crypto,
    AppSW,
};
use borsh::BorshDeserialize;

use super::common::finalize_sign::{self, Signature};

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    let mut stream = HashingStream::new(stream)?;

    let msg_discriminant = MessageDiscriminant::new(NEP_413_SIGN_MESSAGE);

    let prefix_bytes = msg_discriminant.borsh_serialize();

    stream
        .feed_slice(&prefix_bytes)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let mut payload: Payload = Payload::new();

    payload
        .deserialize_reader_in_place(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    // A message is clear-signed only when every field is displayed in full, with
    // each byte shown as its own glyph. Otherwise it is signed blind, like
    // function-call arguments: the user must have enabled blind signing in the
    // settings and accept the warning, and the review shows what fits. None of
    // these fields is validated on chain, the recipient included.
    let callback_url_displayable = payload.callback_url.as_ref().is_none_or(displayable);
    let blind = if !displayable(&payload.message)
        || !displayable(&payload.recipient)
        || !callback_url_displayable
    {
        Some(sign_ui::blind_signing::approve(payload.message_len)?)
    } else {
        None
    };

    if !sign_ui::nep413::payload::ui_display(&mut payload, blind.as_ref()) {
        return Err(AppSW::Deny);
    }

    finalize_sign::end(stream, &path)
}

/// `true` when the field fits the review and every byte is shown as its own glyph.
fn displayable<const N: usize>(field: &CappedString<N>) -> bool {
    !field.truncated() && field.is_printable_ascii()
}
