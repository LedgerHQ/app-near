#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use crate::app_ui::logo::NEAR_LOGO;
use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing::types::nep413::payload::Payload,
    sign_ui::blind_signing::{describe_size, BlindSigned, SizeBuffer},
    utils::types::review_field::ReviewField,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{Field, NbglReview, NbglReviewStatus, StatusType};
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};

/// length, twice as long as [crate::parsing::types::nep413::payload::NonceBuffer],
/// sufficient to store its representation as hexadecimal string.
/// NOTE: arrays only implement [Default] up to 32 in size
const NONCE_HEX_LENGTH: usize = 64;

struct FieldsContext {
    nonce_buffer: [u8; NONCE_HEX_LENGTH],
    message_size_buffer: SizeBuffer,
    callback_url_size_buffer: SizeBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            nonce_buffer: [0u8; NONCE_HEX_LENGTH],
            message_size_buffer: SizeBuffer::new(),
            callback_url_size_buffer: SizeBuffer::new(),
        }
    }
}

/// Message (1) + Message size (0-1, signed blind) + Nonce (1) + Recipient (1) +
/// Callback Url (0-1) + Callback Url size (0-1, signed blind)
const MAX_FIELDS: usize = 6;

fn format<'b, 'a: 'b>(
    payload: &'b mut Payload,
    blind: Option<&BlindSigned>,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    // A truncated field reaches the review only against a blind-signing approval, which
    // the handler obtains before calling `ui_display`.
    match blind {
        Some(blind) if payload.message.truncated() => {
            let shown_bytes = payload.message.shown_bytes();
            writer.push_field(ReviewField::from_capped_string_blind(
                &mut payload.message,
                "Message",
                blind,
            ));
            writer.push_field(ReviewField::new(Field {
                name: "Message size",
                value: describe_size(
                    payload.message_len,
                    shown_bytes,
                    &mut field_context.message_size_buffer,
                ),
            }));
        }
        _ => {
            writer.push_field(ReviewField::from_capped_string(
                &mut payload.message,
                "Message",
            ));
        }
    }

    // .unwrap() is ok, as `64 == 32 * 2` holds true
    hex::encode_to_slice(payload.nonce, &mut field_context.nonce_buffer).unwrap();
    writer.push_field(ReviewField::new(Field {
        name: "Nonce",
        // .unwrap() is ok, as buffer contains only bytes, encoding hex chars
        value: core::str::from_utf8(&field_context.nonce_buffer).unwrap(),
    }));

    let recipient_fields = ReviewField::from_capped_string(&mut payload.recipient, "Recipient");
    writer.push_field(recipient_fields);

    if let Some(callback_url) = payload.callback_url.as_mut() {
        match blind {
            Some(blind) if callback_url.truncated() => {
                let shown_bytes = callback_url.shown_bytes();
                writer.push_field(ReviewField::from_capped_string_blind(
                    callback_url,
                    "Callback Url",
                    blind,
                ));
                writer.push_field(ReviewField::new(Field {
                    name: "Callback Url size",
                    value: describe_size(
                        payload.callback_url_len,
                        shown_bytes,
                        &mut field_context.callback_url_size_buffer,
                    ),
                }));
            }
            _ => {
                writer.push_field(ReviewField::from_capped_string(
                    callback_url,
                    "Callback Url",
                ));
            }
        }
    }
}

/// `blind` is the user's approval to sign fields the review cannot show in full; it must be
/// `Some` whenever a field is truncated.
pub fn ui_display(payload: &mut Payload, blind: Option<&BlindSigned>) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(payload, blind, &mut field_context, &mut field_writer);

    #[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
    {
        let my_review = MultiFieldReview::new(
            field_writer.get_fields(),
            &["View NEP413 msg sign"],
            Some(&EYE),
            "Sign",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        my_review.show()
    }

    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    {
        let review: NbglReview = NbglReview::new()
            .titles("Review NEP413 msg sign", "", "Sign message")
            .glyph(&NEAR_LOGO);

        let res = review.show(field_writer.get_fields());

        NbglReviewStatus::new()
            .status_type(StatusType::Message)
            .show(res);

        res
    }
}
