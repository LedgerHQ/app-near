use near_token::NearToken;

use borsh::BorshDeserialize;
use borsh::io::{Read, Result};

pub struct Transfer {
    pub deposit: NearToken,
}

impl BorshDeserialize for Transfer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let deposit: NearToken = BorshDeserialize::deserialize_reader(reader)?;
        Ok(Self { deposit })
    }
}
