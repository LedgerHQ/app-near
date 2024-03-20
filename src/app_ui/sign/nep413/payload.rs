use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing::types::nep413::payload::Payload,
    utils::types::elipsis_fields::ElipsisFields,
};

struct FieldsContext {
    msg_display_buf: [u8; 20],
    nonce_buffer: [u8; 64],
    recipient_display_buf: [u8; 20],
    callback_url_display_buf: [u8; 20],
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            msg_display_buf: [0u8; 20],
            nonce_buffer: [0u8; 64],
            recipient_display_buf: [0u8; 20],
            callback_url_display_buf: [0u8; 20],
        }
    }
}

fn format<'b, 'a: 'b>(
    payload: &'b mut Payload,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 7>,
) {
    // 2
    let message_fields = ElipsisFields::from_capped_string(
        &mut payload.message,
        "Message",
        &mut field_context.msg_display_buf,
    );
    writer.push_fields(message_fields);

    // 3
    // .unwrap() is ok, as `64 == 32 * 2` holds true
    hex::encode_to_slice(payload.nonce, &mut field_context.nonce_buffer).unwrap();
    writer.push_fields(ElipsisFields::one(Field {
        name: "Nonce",
        // .unwrap() is ok, as buffer contains only bytes, encoding hex chars
        value: core::str::from_utf8(&field_context.nonce_buffer).unwrap(),
    }));

    // 5
    let recipient_fields = ElipsisFields::from_capped_string(
        &mut payload.recipient,
        "Recipient",
        &mut field_context.recipient_display_buf,
    );
    writer.push_fields(recipient_fields);

    // 7
    if let Some(callback_url) = payload.callback_url.as_mut() {
        let callback_url_fields = ElipsisFields::from_capped_string(
            callback_url,
            "Callback Url",
            &mut field_context.callback_url_display_buf,
        );
        writer.push_fields(callback_url_fields);
    }
}
pub fn ui_display(payload: &mut Payload) -> bool {
    let mut field_writer: FieldsWriter<'_, 7> = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(payload, &mut field_context, &mut field_writer);

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
