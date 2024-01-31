use ledger_device_sdk::io::Event;
use ledger_secure_sdk_sys::buttons::ButtonEvent;

use crate::{
    parsing::{HashingStream, SingleTxStream},
    AppSW,
};
use crate::{sign_ui, Instruction};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    _ordinal_action: u32,
    _total_actions: u32,
) -> Result<(), AppSW> {
    stream.reader.comm.reply(AppSW::TxParsingFail);
    sign_ui::widgets::delegate_error_screen();
    loop {
        match stream.reader.comm.next_event::<Instruction>() {
            Event::Button(button) => match button {
                ButtonEvent::BothButtonsRelease => {
                    return Err(AppSW::TxParsingFail);
                }
                _ => {
                    // ignore all other button presses
                }
            },
            _ => (),
        };
    }
}
