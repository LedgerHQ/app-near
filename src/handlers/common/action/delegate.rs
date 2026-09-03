use crate::sign_ui;
use crate::{
    AppSW,
    parsing::{HashingStream, SingleTxStream},
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    _params: ActionParams,
) -> Result<(), AppSW> {
    stream.reader.comm.reply(AppSW::TxParsingFail);
    sign_ui::action::ui_display_delegate_error(stream.reader.comm);
    Err(AppSW::TxParsingFail)
}
