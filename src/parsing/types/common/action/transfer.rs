use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
};

use super::Balance;

pub struct Transfer {
    pub deposit: Balance,
}

impl BorshDeserialize for Transfer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let balance = u128::deserialize_reader(reader)?;
        Ok(Self { deposit: balance })
    }
}
