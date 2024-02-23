use near_token::NearToken;

use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
};

pub struct Transfer {
    pub deposit: NearToken,
}

impl BorshDeserialize for Transfer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let balance = u128::deserialize_reader(reader)?;
        Ok(Self {
            deposit: NearToken::from_yoctonear(balance),
        })
    }
}
