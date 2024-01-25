use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, utils::types::capped_string::ElipsisFields,
};

pub fn ui_display(transaction_prefix: &parsing::types::TransactionPrefix) -> bool {
    #[cfg(feature = "speculos")]
    transaction_prefix.debug_print();

    let mut field_writer: FieldsWriter<'_, 5> = FieldsWriter::new();
    let signer_id = transaction_prefix.signer_id.ui_fields("Signer Id");
    field_writer.push_fields(signer_id).unwrap();

    let receiver_id = transaction_prefix.receiver_id.ui_fields("Receiver Id");
    field_writer.push_fields(receiver_id).unwrap();
    let mut numtoa_buf = [0u8; 10];

    let num_actions_str = transaction_prefix
        .number_of_actions
        .numtoa_str(10, &mut numtoa_buf);
    field_writer
        .push_fields(ElipsisFields::one(Field {
            name: "Total actions:",
            value: num_actions_str,
        }))
        .unwrap();

    let my_review = MultiFieldReview::new(
        field_writer.get_fields(),
        &["View header"],
        Some(&EYE),
        "Continue to actions",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}
