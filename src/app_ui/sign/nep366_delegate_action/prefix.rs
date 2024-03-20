use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::elipsis_fields::ElipsisFields,
};

struct FieldsContext {
    display_buf1: [u8; 20],
    display_buf2: [u8; 20],
    numtoa_buf: [u8; 10],
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            display_buf1: [0u8; 20],
            display_buf2: [0u8; 20],
            numtoa_buf: [0u8; 10],
        }
    }
}

fn format<'b, 'a: 'b>(
    prefix: &'b mut parsing::types::nep366_delegate_action::prefix::Prefix,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 5>,
) {
    let sender_id = ElipsisFields::from_capped_string(
        &mut prefix.sender_id,
        "Sender Id",
        &mut field_context.display_buf1,
    );
    writer.push_fields(sender_id);

    let receiver_id = ElipsisFields::from_capped_string(
        &mut prefix.receiver_id,
        "Receiver Id",
        &mut field_context.display_buf2,
    );
    writer.push_fields(receiver_id);

    let num_actions_str = prefix
        // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
        .number_of_actions
        .numtoa_str(10, &mut field_context.numtoa_buf);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Total subactions",
        value: num_actions_str,
    }));
}
pub fn ui_display(prefix: &mut parsing::types::nep366_delegate_action::prefix::Prefix) -> bool {
    let mut field_writer: FieldsWriter<'_, 5> = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(prefix, &mut field_context, &mut field_writer);

    let my_review = MultiFieldReview::new(
        field_writer.get_fields(),
        &["View NEP366 prefix"],
        Some(&EYE),
        "Proceed to subactions",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}
