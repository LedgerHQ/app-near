use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};

/// Balance is type for storing amounts of tokens.
pub type Balance = u128;

pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

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
            0 | 1 | 2 | 4 | 5 | 6 | 7 | 8 => unimplemented!("stub for other variants"),
            3 => {
                Ok(Self::Transfer)
            }
            _ => {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }
    }
}
