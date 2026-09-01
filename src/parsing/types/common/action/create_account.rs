use borsh::BorshDeserialize;
use borsh::io::{Read, Result};

pub struct CreateAccount {}

impl BorshDeserialize for CreateAccount {
    fn deserialize_reader<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self {})
    }
}
