use borsh::BorshDeserialize;

use crate::{
    AppSW,
    handlers::common::action::ActionParams,
    parsing::{
        HashingStream, SingleTxStream,
        types::common::action::gas_key_transaction::{
            GasKeyTransactionData, GasKeyTransactionType,
        },
    },
    sign_ui::action::{ui_display_gas_key_transfer, ui_display_gas_key_withdraw},
};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
    transaction_type: GasKeyTransactionType,
) -> Result<(), AppSW> {
    let gas_key_data =
        GasKeyTransactionData::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    let ui_result_success = match transaction_type {
        GasKeyTransactionType::Transfer => ui_display_gas_key_transfer(&gas_key_data, params),
        GasKeyTransactionType::Withdraw => ui_display_gas_key_withdraw(&gas_key_data, params),
    };

    if !ui_result_success {
        return Err(AppSW::Deny);
    }

    Ok(())
}
