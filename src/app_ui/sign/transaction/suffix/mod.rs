use crate::{app_ui::fields_writer::FieldsWriter, parsing};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    CenteredInfo, CenteredInfoStyle, InfoLongPress, NbglGenericReview, NbglGlyph, NbglPageContent,
    NbglStatus, TagValueList, TuneIndex,
};
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::MultiFieldReview,
};
mod tx_v1;

pub fn ui_display_transaction_v1(
    v1_suffix: &parsing::types::transaction::suffix::V1Suffix,
) -> bool {
    let mut writer = FieldsWriter::new();
    let mut field_context: tx_v1::FieldsContext = tx_v1::FieldsContext::new();

    tx_v1::format(v1_suffix, &mut field_context, &mut writer);

    ui_display_common(&mut writer)
}

pub fn ui_display_common<const N: usize>(writer: &mut FieldsWriter<'_, N>) -> bool {
    let msg_after = "Sign";

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let msg_before = ["View transaction", "suffix"];

        let my_review = MultiFieldReview::new(
            writer.get_fields(),
            &msg_before,
            Some(&EYE),
            msg_after,
            Some(&VALIDATE_14),
            "Reject",
            Some(&CROSSMARK),
        );

        my_review.show()
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        let msg_before = "View transaction suffix";

        const NEAR_LOGO: NbglGlyph =
            NbglGlyph::from_include(include_gif!("icons/app_near_64px.gif", NBGL));

        let centered_info = CenteredInfo::new(
            msg_before,
            "",
            "",
            Some(&NEAR_LOGO),
            false,
            CenteredInfoStyle::LargeCaseBoldInfo,
            0,
        );
        let tag_values_list = TagValueList::new(writer.get_fields(), 2, false, false);

        let info_longpress = InfoLongPress::new(
            msg_after,
            Some(&NEAR_LOGO),
            "Hold to sign",
            TuneIndex::Error,
        );

        let mut review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list));

        let last_screen: &str;

        review = review.add_content(NbglPageContent::InfoLongPress(info_longpress));
        last_screen = "Transaction signed";

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text(last_screen).show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        res
    }
}
