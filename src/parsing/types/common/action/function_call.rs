use crate::{
    io::{Read, Result},
    parsing::{borsh::BorshDeserialize, types::common::near_token::NearToken},
    utils::types::capped_string::CappedString,
};

use super::{Balance, Gas};

pub struct FunctionCallCommon {
    pub method_name: CappedString<50>,
    pub gas: Gas,
    pub deposit: NearToken,
}

impl FunctionCallCommon {
    pub fn deserialize_with_method_name<R: Read>(
        reader: &mut R,
        method_name: CappedString<50>,
    ) -> Result<Self> {
        let gas: Gas = BorshDeserialize::deserialize_reader(reader)?;
        let deposit: Balance = BorshDeserialize::deserialize_reader(reader)?;

        let r = Self {
            method_name,
            gas,
            deposit: NearToken::from_yoctonear(deposit),
        };
        Ok(r)
    }
}
