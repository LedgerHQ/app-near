use ledger_device_sdk::buttons::ButtonEvent;
use ledger_device_sdk::io::Event;

use crate::{
    parsing::{HashingStream, SingleTxStream},
    AppSW,
};
use crate::{sign_ui, Instruction};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    _params: ActionParams,
) -> Result<(), AppSW> {
    stream.reader.comm.reply(AppSW::TxParsingFail);
    sign_ui::widgets::delegate_error_screen();
    loop {
        if let Event::Button(ButtonEvent::BothButtonsRelease) =
            stream.reader.comm.next_event::<Instruction>()
        {
            return Err(AppSW::TxParsingFail);
        };
    }
}
