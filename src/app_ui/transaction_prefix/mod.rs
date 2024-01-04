use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{MultiFieldReview, Field},
};
use numtoa::NumToA;

use crate::{
    transaction_prefix::TransactionPrefix,
    utils
};

pub fn ui_display_tx_prefix(transaction_prefix: &TransactionPrefix) -> bool {

    let mut field_writer: utils::ui::FieldsWriter<'_, 5> = utils::ui::FieldsWriter::new();
    let signer_id = utils::ui::capped_string_fields(&transaction_prefix.signer_id, "Signer Id");
    match field_writer.push_fields(signer_id) {
        Ok(..) => {},
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
        
    }

    let receiver_id = utils::ui::capped_string_fields(&transaction_prefix.receiver_id, "Receiver Id");
    match field_writer.push_fields(receiver_id) {
        Ok(..) => {},
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }
    let mut numtoa_buf = [0u8; 10];

    let num_actions_str = transaction_prefix.number_of_actions.numtoa_str(10, &mut numtoa_buf);
    match field_writer.push_fields(utils::ui::ElipsisFields::one(Field {
        name: "Total actions:",
        value: num_actions_str,
        
    })) {
        Ok(..) => {},
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }


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
