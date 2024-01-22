use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};

/// Balance is type for storing amounts of tokens.
pub type Balance = u128;

pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub mod create_account;
pub mod delete_account;
pub mod transfer;
pub mod delete_key;

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
            1 | 2 | 4 | 5 | 8 => unimplemented!("stub for other variants"),
            0 => Ok(Self::CreateAccount),
            3 => Ok(Self::Transfer),
            6 => Ok(Self::DeleteKey), 
            7 => Ok(Self::DeleteAccount),
            _ => {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }
    }
}
