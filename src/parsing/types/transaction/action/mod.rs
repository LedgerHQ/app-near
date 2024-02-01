use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};

/// Balance is type for storing amounts of tokens.
pub type Balance = u128;

/// Gas is a type for storing amount of gas.
pub type Gas = u64;

/// Nonce for transactions.
pub type Nonce = u64;

pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub mod add_key;
pub mod create_account;
pub mod delete_account;
pub mod delete_key;
pub mod deploy_contract;
pub mod function_call;
pub mod stake;
pub mod transfer;

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
            _ => {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }
    }
}
