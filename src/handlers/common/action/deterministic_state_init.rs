use crate::{
    AppSW,
    app_ui::aliases::CappedAccountId,
    handlers::common::action::ActionParams,
    parsing::{
        HashingStream, SingleTxStream,
        types::common::action::deterministic_state_init::{
            DeterministicAccountStateInit, DeterministicAccountStateInitPostfix,
        },
    },
    sign_ui::action::ui_display_deterministic_state_init_v1,
};

use borsh::BorshDeserialize;
use ledger_device_sdk::hash::{HashInit, sha3::Keccak256};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
    receiver_account_id: &CappedAccountId,
) -> Result<(), AppSW> {
    let mut keccak256 = Keccak256::new();
    let state_init = DeterministicAccountStateInit::deserialize_and_hash(stream, &mut keccak256)
        .map_err(|err| match err.kind() {
            borsh::io::ErrorKind::OutOfMemory => AppSW::TxHashFail,
            _ => AppSW::TxParsingFail,
        })?;

    let deposit = DeterministicAccountStateInitPostfix::deserialize_reader(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    verify_derived_account(receiver_account_id, &mut keccak256)?;

    let ui_result_success: bool = match state_init {
        DeterministicAccountStateInit::V1(mut state_init_v1) => {
            ui_display_deterministic_state_init_v1(&mut state_init_v1, &deposit, params)
        }
    };

    if !ui_result_success {
        return Err(AppSW::Deny);
    }
    Ok(())
}

const ACCOUNT_ID_PREFIX_LEN: usize = 2;
const STATE_INIT_ACCOUNT_ID_PREFIX: [u8; ACCOUNT_ID_PREFIX_LEN] = *b"0s";
/// 40 bytes ("hex::encode(keccak256.finalize(...)[12..32])")
const STATE_INIT_ACCOUNT_ID_V1_LEN_NO_PREFIX: usize = 40;
/// 2 prefix bytes ("0s") + 40 bytes ("hex::encode(keccak256.finalize(...)[12..32])")
const STATE_INIT_ACCOUNT_ID_V1_LEN: usize =
    STATE_INIT_ACCOUNT_ID_V1_LEN_NO_PREFIX + ACCOUNT_ID_PREFIX_LEN;

fn verify_derived_account(
    receiver_account_id: &CappedAccountId,
    hasher: &mut Keccak256,
) -> Result<(), AppSW> {
    let mut buf = [0u8; 32];
    let mut hex_buf: [u8; STATE_INIT_ACCOUNT_ID_V1_LEN] = {
        let mut arr = [0u8; STATE_INIT_ACCOUNT_ID_V1_LEN];
        arr.as_mut_slice()[..ACCOUNT_ID_PREFIX_LEN].copy_from_slice(&STATE_INIT_ACCOUNT_ID_PREFIX);
        arr
    };

    hasher
        .finalize(&mut buf)
        .map_err(|_err| AppSW::TxHashFinalizeFail)?;

    // .unwrap() is fine as 40 = 20 * 2, which fits hex_buf
    hex::encode_to_slice(&buf[12..32], &mut hex_buf[ACCOUNT_ID_PREFIX_LEN..]).unwrap();

    let acc = receiver_account_id.as_bytes();

    if acc.len() != STATE_INIT_ACCOUNT_ID_V1_LEN || *acc != hex_buf {
        let _confirm = verify_mismatch_display(
            receiver_account_id.clone().as_str(),
            // .unwrap() is ok as we base our display on hex
            core::str::from_utf8(&hex_buf).unwrap(),
        );
        return Err(AppSW::DerivedAccountIdMismatch);
    }

    Ok(())
}

fn verify_mismatch_display(receiver_account_id: &str, derived_account_id: &str) -> bool {
    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    use ledger_device_sdk::nbgl::Field;
    #[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
    use ledger_device_sdk::ui::gadgets::Field;

    let my_fields = [
        Field {
            name: "Receiver Account Id",
            value: receiver_account_id,
        },
        Field {
            name: "Derived Account Id",
            value: derived_account_id,
        },
    ];

    let msg_before = "Derived Account Id";
    let msg_before_2 = "Mismatch";
    let msg_after = "Error!";

    #[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
    {
        use ledger_device_sdk::ui::{
            bitmaps::{CROSSMARK, EYE},
            gadgets::MultiFieldReview,
        };

        let binding = [msg_before, msg_before_2];

        let my_review = MultiFieldReview::new(
            &my_fields,
            &binding,
            Some(&EYE),
            msg_after,
            Some(&CROSSMARK),
            msg_after,
            Some(&CROSSMARK),
        );

        my_review.show()
    }
    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    {
        use crate::app_ui::logo::NEAR_LOGO;
        use ledger_device_sdk::nbgl::{
            CenteredInfo, CenteredInfoStyle, InfoButton, NbglGenericReview, NbglPageContent,
            NbglStatus, TagValueList, TuneIndex,
        };

        let centered_info = CenteredInfo::new(
            msg_before,
            msg_before_2,
            "",
            Some(&NEAR_LOGO),
            false,
            CenteredInfoStyle::LargeCaseBoldInfo,
            0,
        );

        let info_button =
            InfoButton::new(msg_after, Some(&NEAR_LOGO), "Confirm", TuneIndex::Success);

        let tag_values_list = TagValueList::new(&my_fields, 2, false, false);

        let review: NbglGenericReview = NbglGenericReview::new()
            .add_content(NbglPageContent::CenteredInfo(centered_info))
            .add_content(NbglPageContent::TagValueList(tag_values_list))
            .add_content(NbglPageContent::InfoButton(info_button));

        let res = review.show("Reject");
        let status: NbglStatus = NbglStatus::new();
        match res {
            true => {
                status.text("Confirmed").show(true);
            }
            false => {
                status.text("Transaction rejected").show(false);
            }
        }
        res
    }
}
