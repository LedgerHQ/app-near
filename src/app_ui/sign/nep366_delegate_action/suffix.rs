use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

use crate::{
    app_ui::{aliases::U64Buffer, fields_writer::FieldsWriter},
    parsing,
    sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};

struct FieldsContext {
    pub num_buf1: U64Buffer,
    pub num_buf2: U64Buffer,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            num_buf1: U64Buffer::default(),
            num_buf2: U64Buffer::default(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
        }
    }
}

/// Nonce (1) + Max Block Height (1) + Public Key (1)
const MAX_FIELDS: usize = 3;

fn format<'b, 'a: 'b>(
    suffix: &'b parsing::types::nep366_delegate_action::suffix::Suffix,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Nonce",
        // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
        value: suffix.nonce.numtoa_str(10, &mut field_context.num_buf1),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Max Block Height",
        value: suffix
            .max_block_height
            // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
            .numtoa_str(10, &mut field_context.num_buf2),
    }));

    field_context
        .pub_key_context
        .format_public_key(&suffix.public_key);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));
}

pub fn ui_display(suffix: &parsing::types::nep366_delegate_action::suffix::Suffix) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(suffix, &mut field_context, &mut field_writer);

    let my_review = MultiFieldReview::new(
        field_writer.get_fields(),
        &["View NEP366 suffix"],
        Some(&EYE),
        "Sign",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}
