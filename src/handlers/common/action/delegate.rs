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
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    sign_ui::widgets::delegate_error_screen();
    loop {
        #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        if let Event::Button(ButtonEvent::BothButtonsRelease) =
            stream.reader.comm.next_event::<Instruction>()
        {
            return Err(AppSW::TxParsingFail);
        };
    }
    }
}
