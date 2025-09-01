use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

/// Nonce for transactions.
pub type Nonce = u64;

/// Height of the block.
pub type BlockHeight = u64;

pub mod add_key;
pub mod create_account;
pub mod delete_account;
pub mod delete_key;
pub mod deploy_contract;
pub mod deploy_global_contract;
pub mod function_call;
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
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}
