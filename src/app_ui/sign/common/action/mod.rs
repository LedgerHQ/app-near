use crate::app_ui::aliases::{FnCallCappedString, FnCallHexDisplay, U32Buffer};
use crate::{app_ui::fields_writer::FieldsWriter, handlers::common::action::ActionParams, parsing};
use fmt_buffer::Buffer;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::MultiFieldReview,
};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{Field, NbglReview, NbglGlyph};

#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
use crate::AppSW;
use crate::settings;
use numtoa::NumToA;

use super::tx_public_key_context;

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

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub fn ui_display_my_tx() -> Result<(), AppSW> {
    // Define transaction review fields
    let my_fields = [
        Field {
            name: "Amount",
            value: "500",
        },
        Field {
            name: "Destination",
            value: "my_destination",
        },
        Field {
            name: "Memo",
            value: "exmem",
        },
    ];

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));
        // Create NBGL review. Maximum number of fields and string buffer length can be customised
        // with constant generic parameters of NbglReview. Default values are 32 and 1024 respectively.
        let mut review: NbglReview = NbglReview::new()
            .titles(
                "Review transaction\nto send CRAB",
                "",
                "Sign transaction\nto send CRAB",
            )
            .glyph(&FERRIS);

        // If first setting switch is disabled do not display the transaction memo
        let settings: settings::Settings = Default::default();
        if settings.get_element(0) == 0 {
            review.show(&my_fields[0..2]);
        } else {
            review.show(&my_fields);
        }
        Ok(())
    }
}



pub fn ui_display_transfer(transfer: &parsing::types::Transfer, params: ActionParams) -> bool {
    let mut field_context: transfer::FieldsContext = transfer::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    transfer::format(transfer, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_create_account(
    create_account: &parsing::types::CreateAccount,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();

    create_account::format(create_account, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_delete_account(
    delete_account: &mut parsing::types::DeleteAccount,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut field_context: delete_account::FieldsContext = delete_account::FieldsContext::new();

    delete_account::format(delete_account, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_delete_key(delete_key: &parsing::types::DeleteKey, params: ActionParams) -> bool {
    let mut field_context: tx_public_key_context::FieldsContext =
        tx_public_key_context::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    delete_key::format(delete_key, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_stake(stake: &parsing::types::Stake, params: ActionParams) -> bool {
    let mut field_context: stake::FieldsContext = stake::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    stake::format(stake, &mut field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

/// action type (1) + Public Key (1) + Access Key Nonce (1) +
/// Access Permission (1)
const ADD_KEY_FULL_ACCESS_MAX_FIELDS: usize = 4;

pub fn ui_display_add_key_fullaccess(
    add_key: &parsing::types::AddKey,
    params: ActionParams,
) -> bool {
    let mut field_context: add_key_common::FieldsContext = add_key_common::FieldsContext::new();
    let mut writer: FieldsWriter<'_, ADD_KEY_FULL_ACCESS_MAX_FIELDS> = FieldsWriter::new();

    add_key_common::format(add_key, &mut field_context, &mut writer, "Full Access");

    ui_display_common(&mut writer, params)
}

pub fn ui_display_add_key_functioncall(
    add_key: &parsing::types::AddKey,
    function_call_per: &mut parsing::types::FunctionCallPermission,
    params: ActionParams,
) -> bool {
    let mut common_field_context: add_key_common::FieldsContext =
        add_key_common::FieldsContext::new();
    let mut func_call_field_context: function_call_permission::FieldsContext =
        function_call_permission::FieldsContext::new();
    let mut writer = FieldsWriter::new();

    add_key_common::format(
        add_key,
        &mut common_field_context,
        &mut writer,
        "Function Call",
    );
    function_call_permission::format(function_call_per, &mut func_call_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_deploy_contract(
    deploy_contract: &parsing::types::DeployContract,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();

    deploy_contract::format(deploy_contract, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_function_call_str(
    func_call_common: &mut parsing::types::FunctionCallCommon,
    args: &mut FnCallCappedString,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_str::FieldsContext =
        function_call_str::FieldsContext::new();
    function_call_str::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}

pub fn ui_display_function_call_bin(
    func_call_common: &mut parsing::types::FunctionCallCommon,
    args: &FnCallHexDisplay,
    params: ActionParams,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut common_field_context: function_call_common::FieldsContext =
        function_call_common::FieldsContext::new();

    function_call_common::format(func_call_common, &mut common_field_context, &mut writer);
    let mut args_field_context: function_call_bin::FieldsContext =
        function_call_bin::FieldsContext::new();
    function_call_bin::format(args, &mut args_field_context, &mut writer);

    ui_display_common(&mut writer, params)
}
pub fn ui_display_common<const N: usize>(
    writer: &mut FieldsWriter<'_, N>,
    params: ActionParams,
) -> bool {
    let mut ordinal_fmt_buf = OrdinalStringBuffer::new();
    let is_last = ordinal_string(&mut ordinal_fmt_buf, params);

    let ordinal_str = ordinal_fmt_buf.as_str();

    let binding = [ordinal_str];

    let next_msg = if params.is_nested_delegate {
        "Next Subaction"
    } else {
        "Next Action"
    };

    let last_msg = if params.is_nested_delegate {
        "To NEP366 suffix"
    } else {
        "Sign"
    };

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let my_review = MultiFieldReview::new(
            writer.get_fields(),
            &binding,
            Some(&EYE),
            if is_last { last_msg } else { next_msg },
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
                "Review transaction\nto send CRAB",
                "",
                "Sign transaction\nto send CRAB",
            )
            .glyph(&FERRIS);

        review.show(writer.get_fields())
    }
}

/// a buffer, large enough to fit description string and
/// 2 u32 numbers as strings
type OrdinalStringBuffer = Buffer<40>;

fn ordinal_string(fmt_buf: &mut OrdinalStringBuffer, params: ActionParams) -> bool {
    let mut num_out = U32Buffer::default();
    let header = if params.is_nested_delegate {
        "View subaction "
    } else {
        "View action "
    };
    fmt_buf.write_str(header);
    // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
    fmt_buf.write_str(params.ordinal_action.numtoa_str(10, &mut num_out));
    fmt_buf.write_str(" / ");
    // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
    fmt_buf.write_str(params.total_actions.numtoa_str(10, &mut num_out));

    params.ordinal_action == params.total_actions
}
