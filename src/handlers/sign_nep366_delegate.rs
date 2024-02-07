use crate::{
    parsing::{
        self,
        borsh::BorshDeserialize,
        types::{common::message_discriminant::NEP_366_META_TRANSACTIONS, MessageDiscriminant},
        HashingStream, SingleTxStream,
    },
    sign_ui,
    utils::crypto,
    AppSW,
};

use super::common::{
    action::{handle_action, ActionParams},
    finalize_sign::{self, Signature},
};

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    #[cfg(feature = "speculos")]
    path.debug_print();

    let mut stream = HashingStream::new(stream)?;

    let msg_discriminant = MessageDiscriminant::new_on_chain(NEP_366_META_TRANSACTIONS).unwrap();

    let prefix_bytes = msg_discriminant.borsh_serialize();

    stream
        .feed_slice(&prefix_bytes)
        .map_err(|_err| AppSW::TxParsingFail)?;

    handle_delegate_action(&mut stream)?;

    finalize_sign::end(&mut stream, &path)
}

pub fn handle_delegate_action(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<(), AppSW> {
    let num_of_actions = handle_prefix(stream)?;

    for i in 0..num_of_actions {
        sign_ui::widgets::display_receiving();
        let params = ActionParams {
            ordinal_action: i + 1,
            total_actions: num_of_actions,
            is_nested_delegate: true,
        };
        handle_action(stream, params)?;
    }
    handle_suffix(stream)?;
    Ok(())
}

fn handle_prefix(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<u32, AppSW> {
    let mut delegate_action_prefix = parsing::types::nep366_delegate_action::prefix::Prefix::new();

    #[cfg(feature = "speculos")]
    delegate_action_prefix.debug_print();

    delegate_action_prefix
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::nep366_delegate_action::prefix::ui_display(&delegate_action_prefix) {
        return Err(AppSW::Deny);
    }
    Ok(delegate_action_prefix.number_of_actions)
}

fn handle_suffix(stream: &mut HashingStream<SingleTxStream<'_>>) -> Result<(), AppSW> {
    let delegate_action_suffix =
        parsing::types::nep366_delegate_action::suffix::Suffix::deserialize_reader(stream)
            .map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    delegate_action_suffix.debug_print();

    if !sign_ui::nep366_delegate_action::suffix::ui_display(&delegate_action_suffix) {
        return Err(AppSW::Deny);
    }
    Ok(())
}
