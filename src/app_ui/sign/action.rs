use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing,
    utils::types::{capped_string::CappedString, fmt_buffer::FmtBuffer, hex_display::HexDisplay},
};

use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::MultiFieldReview,
};
use numtoa::NumToA;

mod add_key_common;
mod create_account;
mod delete_account;
mod delete_key;
mod deploy_contract;
mod function_call_bin;
mod function_call_common;
mod function_call_permission;
mod function_call_str;
mod stake;
mod transfer;

pub fn ui_display_transfer(
    transfer: &parsing::types::Transfer,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut field_context: transfer::FieldsContext = transfer::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 2> = FieldsWriter::new();

    transfer::format(transfer, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_create_account(
    create_account: &parsing::types::CreateAccount,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 1> = FieldsWriter::new();

    create_account::format(create_account, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_delete_account(
    delete_account: &parsing::types::DeleteAccount,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 3> = FieldsWriter::new();
    let mut field_context: delete_account::FieldsContext = delete_account::FieldsContext::new();

    delete_account::format(delete_account, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_delete_key(
    delete_key: &parsing::types::DeleteKey,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut field_context: delete_key::FieldsContext = delete_key::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 2> = FieldsWriter::new();

    delete_key::format(delete_key, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_stake(stake: &parsing::types::Stake, ordinal: u32, total_actions: u32) -> bool {
    let mut field_context: stake::FieldsContext = stake::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 3> = FieldsWriter::new();

    stake::format(stake, &mut field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_add_key_fullaccess(
    add_key: &parsing::types::AddKey,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut field_context: add_key_common::FieldsContext = add_key_common::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 4> = FieldsWriter::new();

    add_key_common::format(add_key, &mut field_context, &mut writer, "Full Access");

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_add_key_functioncall(
    add_key: &parsing::types::AddKey,
    function_call_per: &parsing::types::FunctionCallPermission,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut common_field_context: add_key_common::FieldsContext =
        add_key_common::FieldsContext::new();
    let mut func_call_field_context: function_call_permission::FieldsContext =
        function_call_permission::FieldsContext::new();
    let mut writer: FieldsWriter<'_, 10> = FieldsWriter::new();

    add_key_common::format(
        add_key,
        &mut common_field_context,
        &mut writer,
        "Function Call",
    );
    function_call_permission::format(function_call_per, &mut func_call_field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_deploy_contract(
    deploy_contract: &parsing::types::DeployContract,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 2> = FieldsWriter::new();

    deploy_contract::format(deploy_contract, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_function_call_str(
    func_call_common: &parsing::types::FunctionCallCommon,
    args: &CappedString<500>,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 7> = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_str::FieldsContext =
        function_call_str::FieldsContext::new();
    function_call_str::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}

pub fn ui_display_function_call_bin(
    func_call_common: &parsing::types::FunctionCallCommon,
    args: &HexDisplay<500>,
    ordinal: u32,
    total_actions: u32,
) -> bool {
    let mut writer: FieldsWriter<'_, 7> = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_bin::FieldsContext =
        function_call_bin::FieldsContext::new();
    function_call_bin::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, ordinal, total_actions)
}
pub fn ui_display_common<const N: usize>(
    writer: &mut FieldsWriter<'_, N>,
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
