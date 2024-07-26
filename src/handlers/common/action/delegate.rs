use crate::sign_ui;
use crate::{
    parsing::{HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    _params: ActionParams,
) -> Result<(), AppSW> {
    stream.reader.comm.reply(AppSW::TxParsingFail);
    sign_ui::action::ui_display_delegate_error(&mut stream.reader.comm);
    Err(AppSW::TxParsingFail)
}
