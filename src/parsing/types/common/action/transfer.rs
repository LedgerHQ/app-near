use near_token::NearToken;

use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct Transfer {
    pub deposit: NearToken,
}

impl BorshDeserialize for Transfer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let deposit: NearToken = BorshDeserialize::deserialize_reader(reader)?;
        Ok(Self { deposit })
    }
}
