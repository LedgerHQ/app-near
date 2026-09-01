use near_token::NearToken;

use crate::parsing::types::TxPublicKey;

use borsh::{
    BorshDeserialize,
    io::{Read, Result},
};

pub struct GasKeyTransactionData {
    /// GasKey to make transaction to
    pub public_key: TxPublicKey,
    /// Amount to deposit to or withdraw from this gas key
    pub amount: NearToken,
}

#[repr(u8)]
pub enum GasKeyTransactionType {
    Transfer,
    Withdraw,
}

impl BorshDeserialize for GasKeyTransactionData {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            public_key: TxPublicKey::deserialize_reader(reader)?,
            amount: NearToken::deserialize_reader(reader)?,
        })
    }
}
