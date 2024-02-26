use near_gas::NearGas;
use near_token::NearToken;

use crate::utils::types::capped_string::CappedString;
use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct FunctionCallCommon {
    pub method_name: CappedString<50>,
    pub gas: NearGas,
    pub deposit: NearToken,
}

impl FunctionCallCommon {
    pub fn deserialize_with_method_name<R: Read>(
        reader: &mut R,
        method_name: CappedString<50>,
    ) -> Result<Self> {
        let gas: NearGas = BorshDeserialize::deserialize_reader(reader)?;
        let deposit: NearToken = BorshDeserialize::deserialize_reader(reader)?;

        let r = Self {
            method_name,
            gas,
            deposit,
        };
        Ok(r)
    }
}
