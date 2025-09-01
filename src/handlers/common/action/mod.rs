use crate::parsing::types::Action;
use crate::parsing::{HashingStream, SingleTxStream};
use crate::AppSW;
use borsh::BorshDeserialize;

pub mod add_key;
pub mod create_account;
pub mod delegate;
pub mod delete_account;
pub mod delete_key;
pub mod deploy_contract;
pub mod deploy_global_contract;
pub mod function_call;
pub mod stake;
pub mod transfer;
pub mod use_global_contract;

#[derive(Clone, Copy)]
pub struct ActionParams {
    pub ordinal_action: u32,
    pub total_actions: u32,
    pub is_nested_delegate: bool,
}

pub fn handle_action(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let action = Action::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    match action {
        Action::Transfer => transfer::handle(stream, params),
        Action::CreateAccount => create_account::handle(stream, params),
        Action::DeleteAccount => delete_account::handle(stream, params),
        Action::DeleteKey => delete_key::handle(stream, params),
        Action::Stake => stake::handle(stream, params),
        Action::AddKey => add_key::handle(stream, params),
        Action::DeployContract => deploy_contract::handle(stream, params),
        Action::FunctionCall => function_call::handle(stream, params),
        Action::Delegate => delegate::handle(stream, params),
        Action::DeployGlobalContract => deploy_global_contract::handle(stream, params),
        Action::UseGlobalContract => use_global_contract::handle(stream, params),
    }
}
