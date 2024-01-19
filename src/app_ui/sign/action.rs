use crate::{parsing::{self, types::action::ONE_NEAR}, utils::types::{fmt_buffer::FmtBuffer, capped_string::ElipsisFields}, app_ui::fields_writer::FieldsWriter};

use ledger_device_sdk::ui::{bitmaps::{EYE, VALIDATE_14, CROSSMARK}, gadgets::{MultiFieldReview, Field}};
use numtoa::NumToA;

pub fn ui_display(action: &parsing::types::Action, ordinal: u32, total_actions: u32) -> bool {

    #[cfg(feature = "speculos")]
    action.debug_print();
    let mut field_writer: FieldsWriter<'_, 5> = FieldsWriter::new();

    
    let mut ordinal_fmt_buf = FmtBuffer::<60>::new();
    let mut num_out = [0u8; 10];
    ordinal_fmt_buf.write_str("View action ");
    ordinal_fmt_buf.write_str(ordinal.numtoa_str(10, &mut num_out));
    ordinal_fmt_buf.write_str(" / ");
    ordinal_fmt_buf.write_str(total_actions.numtoa_str(10, &mut num_out));

    let ordinal_str = ordinal_fmt_buf.as_str();


    match field_writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: action._type(),
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }

    let mut float_buffer = dtoa::Buffer::new();
    match action {
        parsing::types::Action::Transfer(transfer) => {
            
            let deposit = (transfer.deposit as f64) / (ONE_NEAR as f64);
            let printed = float_buffer.format(deposit);
            match field_writer.push_fields(ElipsisFields::one(Field {
                name: "Amount (NEAR)",
                value: printed,
            })) {
                Ok(..) => {}
                Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
            }
            
        }
        _ => {
            unimplemented!("stub for other variants");
        }
    }
    let binding = [ordinal_str];

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
