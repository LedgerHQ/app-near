use crate::app_ui::aliases::CappedAccountId;
use crate::parsing::types::Action;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::{AppSW, parsing};
use borsh::BorshDeserialize;

pub mod add_key;
pub mod create_account;
pub mod delegate;
pub mod delete_account;
pub mod delete_key;
pub mod deploy_contract;
pub mod deploy_global_contract;
pub mod deterministic_state_init;
pub mod function_call;
pub mod gas_key_transaction;
pub mod stake;
pub mod transfer;
pub mod use_global_contract;

#[derive(Clone, Copy)]
pub struct ActionParams {
    pub ordinal_action: u32,
    pub total_actions: u32,
    pub action_str: &'static str,
    pub is_nested_delegate: bool,
}

pub fn handle_action(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    mut params: ActionParams,
    receiver_id: &CappedAccountId,
) -> Result<(), AppSW> {
    let action = Action::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;
    params.action_str = action.get_action_str();
    dispatch_action(stream, action, params, receiver_id)
}

/// Dispatch an already-deserialized `Action` — used when the action tag byte
/// has been consumed upstream (e.g. in sign_tx.rs single-action special path).
pub fn dispatch_action(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    action: Action,
    params: ActionParams,
    receiver_id: &CappedAccountId,
) -> Result<(), AppSW> {
    match action {
        Action::Transfer => transfer::handle(stream, params),
        Action::CreateAccount => create_account::handle(stream, params, receiver_id),
        Action::DeleteAccount => delete_account::handle(stream, params, receiver_id),
        Action::DeleteKey => delete_key::handle(stream, params),
        Action::Stake => stake::handle(stream, params),
        Action::AddKey => add_key::handle(stream, params),
        Action::DeployContract => deploy_contract::handle(stream, params),
        Action::FunctionCall => function_call::handle(stream, params),
        Action::Delegate => delegate::handle(stream, params),
        Action::DeployGlobalContract => deploy_global_contract::handle(stream, params),
        Action::UseGlobalContract => use_global_contract::handle(stream, params),
        Action::DeterministicStateInit => {
            deterministic_state_init::handle(stream, params, receiver_id)
        }
        Action::TransferToGasKey => gas_key_transaction::handle(
            stream,
            params,
            parsing::types::common::action::gas_key_transaction::GasKeyTransactionType::Transfer,
        ),
        Action::WithdrawFromGasKey => gas_key_transaction::handle(
            stream,
            params,
            parsing::types::common::action::gas_key_transaction::GasKeyTransactionType::Withdraw,
        ),
    }
}
