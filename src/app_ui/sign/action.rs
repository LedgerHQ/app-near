use crate::{
    app_ui::{fields_context::FieldsContext, fields_writer::FieldsWriter},
    parsing::{self, types::action::ONE_NEAR},
    utils::types::{capped_string::ElipsisFields, fmt_buffer::FmtBuffer},
};

use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

pub fn ui_display(action: &parsing::types::Action, ordinal: u32, total_actions: u32) -> bool {
    #[cfg(feature = "speculos")]
    action.debug_print();
    let mut field_context: FieldsContext = FieldsContext::new();
    let mut writer: FieldsWriter<'_, 5> = FieldsWriter::new();

    format_action(action, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

fn format_action<'b, 'a: 'b>(
    action: &parsing::types::Action,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 5>,
) {
    match writer.push_fields(ElipsisFields::one(Field {
        name: "Action type:",
        value: action._type(),
    })) {
        Ok(..) => {}
        Err(_err) => panic!("wrong total fields in tx prefix FieldsWriter"),
    }

    match action {
        parsing::types::Action::Transfer(transfer) => {
            let deposit = (transfer.deposit as f64) / (ONE_NEAR as f64);
            let printed = field_context.float_buffer.format(deposit);
            match writer.push_fields(ElipsisFields::one(Field {
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
}

pub fn ui_display_common(
    writer: &mut FieldsWriter<'_, 5>,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut ordinal_fmt_buf = FmtBuffer::<25>::new();
    let is_last = ordinal_string(&mut ordinal_fmt_buf, ordinal, total_actions);

    let ordinal_str = ordinal_fmt_buf.as_str();

    let binding = [ordinal_str];

    let my_review = MultiFieldReview::new(
        writer.get_fields(),
        &binding,
        Some(&EYE),
        if is_last { "Sign" } else { "Next Action" },
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}

fn ordinal_string(fmt_buf: &mut FmtBuffer<25>, ordinal: u32, total_actions: u32) -> bool {
    let mut num_out = [0u8; 10];
    fmt_buf.write_str("View action ");
    fmt_buf.write_str(ordinal.numtoa_str(10, &mut num_out));
    fmt_buf.write_str(" / ");
    fmt_buf.write_str(total_actions.numtoa_str(10, &mut num_out));

    ordinal == total_actions
}
