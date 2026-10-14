#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use crate::app_ui::logo::NEAR_LOGO;
use crate::{
    app_ui::fields_writer::FieldsWriter, parsing::types::nep413::payload::Payload,
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
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            nonce_buffer: [0u8; NONCE_HEX_LENGTH],
        }
    }
}

/// Message (1) + Nonce (1) + Recipient (1) + Callback Url (0-1)
const MAX_FIELDS: usize = 4;

fn format<'b, 'a: 'b>(
    payload: &'b mut Payload,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let message_fields = ReviewField::from_capped_string(&mut payload.message, "Message");
    writer.push_field(message_fields);

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
        let callback_url_fields = ReviewField::from_capped_string(callback_url, "Callback Url");
        writer.push_field(callback_url_fields);
    }
}
pub fn ui_display(payload: &mut Payload) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(payload, &mut field_context, &mut field_writer);

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
