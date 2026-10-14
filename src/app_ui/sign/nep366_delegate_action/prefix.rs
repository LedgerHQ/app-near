#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use crate::app_ui::logo::NEAR_LOGO;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::{
    CenteredInfo, CenteredInfoStyle, Field, InfoButton, NbglGenericReview, NbglPageContent,
    NbglStatus, TagValueList, TuneIndex,
};
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};
use numtoa::NumToA;

use crate::{
    app_ui::{aliases::U32Buffer, fields_writer::FieldsWriter},
    parsing,
    utils::types::review_field::ReviewField,
};

struct FieldsContext {
    numtoa_buf: U32Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            numtoa_buf: U32Buffer::default(),
        }
    }
}

/// Sender Id (1) + Receiver Id (1) + Total subactions (1)
const MAX_FIELDS: usize = 3;

fn format<'b, 'a: 'b>(
    prefix: &'b mut parsing::types::nep366_delegate_action::prefix::Prefix,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let sender_id = ReviewField::from_capped_string(&mut prefix.sender_id, "Sender Id");
    writer.push_field(sender_id);

    let receiver_id = ReviewField::from_capped_string(&mut prefix.receiver_id, "Receiver Id");
    writer.push_field(receiver_id);

    let num_actions_str = prefix
        // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
        .number_of_actions
        .numtoa_str(10, &mut field_context.numtoa_buf);
    writer.push_field(ReviewField::new(Field {
        name: "Total subactions",
        value: num_actions_str,
    }));
}
pub fn ui_display(prefix: &mut parsing::types::nep366_delegate_action::prefix::Prefix) -> bool {
    let mut field_writer = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(prefix, &mut field_context, &mut field_writer);

    let msg_before = "View NEP366 prefix";
    let msg_after = "Proceed to subactions";

    #[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
    {
        let binding = [msg_before];

        let my_review = MultiFieldReview::new(
            field_writer.get_fields(),
            &binding,
            Some(&EYE),
            msg_after,
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        my_review.show()
    }

    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    {
        let centered_info = CenteredInfo::new(
            msg_before,
            "",
            "",
            Some(&NEAR_LOGO),
            false,
            CenteredInfoStyle::LargeCaseBoldInfo,
            0,
        );

        let info_button = InfoButton::new(
            msg_after,
            Some(&NEAR_LOGO),
            "Confirm NEP366 prefix",
            TuneIndex::Success,
        );

        let tag_values_list = TagValueList::new(field_writer.get_fields(), 2, false, false);

        let review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list))
            .add_content(NbglPageContent::InfoButton(info_button));

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text("NEP366 prefix confirmed").show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        res
    }
}
