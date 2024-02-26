use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct CreateAccount {}

impl BorshDeserialize for CreateAccount {
    fn deserialize_reader<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self {})
    }
}
