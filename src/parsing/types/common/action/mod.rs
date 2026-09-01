use borsh::BorshDeserialize;
use borsh::io::{Error, ErrorKind, Read, Result};

/// Nonce for transactions.
pub type Nonce = u64;

/// Nonce index for GasKeys
pub type NonceIndex = u16;

/// Height of the block.
pub type BlockHeight = u64;

pub mod add_key;
pub mod create_account;
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

#[derive(PartialEq)]
pub enum Action {
    CreateAccount,
    DeployContract,
    FunctionCall,
    Transfer,
    Stake,
    AddKey,
    DeleteKey,
    DeleteAccount,
    Delegate,
    DeployGlobalContract,
    UseGlobalContract,
    DeterministicStateInit,
    TransferToGasKey,
    WithdrawFromGasKey,
}

impl BorshDeserialize for Action {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let variant_tag = u8::deserialize_reader(reader)?;
        match variant_tag {
            0 => Ok(Self::CreateAccount),
            1 => Ok(Self::DeployContract),
            2 => Ok(Self::FunctionCall),
            3 => Ok(Self::Transfer),
            4 => Ok(Self::Stake),
            5 => Ok(Self::AddKey),
            6 => Ok(Self::DeleteKey),
            7 => Ok(Self::DeleteAccount),
            8 => Ok(Self::Delegate),
            9 => Ok(Self::DeployGlobalContract),
            10 => Ok(Self::UseGlobalContract),
            11 => Ok(Self::DeterministicStateInit),
            12 => Ok(Self::TransferToGasKey),
            13 => Ok(Self::WithdrawFromGasKey),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}
