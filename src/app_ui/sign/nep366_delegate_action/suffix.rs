#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{Field, NbglGlyph, NbglReview};
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

    let msg_before = "View NEP366 suffix";
    let binding = [msg_before];
    
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            field_writer.get_fields(),
            &binding,
            Some(&EYE),
            "Sign",
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );
    
        my_review.show()
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let mut review: NbglReview = NbglReview::new()
            .titles(
                msg_before,
                "",
                "Sign transaction",
            )
            .glyph(&FERRIS);

        review.show(field_writer.get_fields())
    }
}
