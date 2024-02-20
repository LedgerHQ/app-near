use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
};

pub struct CreateAccount {}

impl BorshDeserialize for CreateAccount {
    fn deserialize_reader<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self {})
    }
}
