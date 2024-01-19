use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing,
    utils::types::fmt_buffer::FmtBuffer,
};

use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::MultiFieldReview,
};
use numtoa::NumToA;

mod transfer;
mod create_account;

pub fn ui_display_transfer(
    transfer: &parsing::types::Transfer,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut field_context: transfer::FieldsContext = transfer::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 5> = FieldsWriter::new();

    transfer::format(transfer, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_create_account(
    create_account: &parsing::types::CreateAccount,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 5> = FieldsWriter::new();

    create_account::format(create_account, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
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
