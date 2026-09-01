use near_token::NearToken;

use crate::parsing::types::TxPublicKey;
use borsh::BorshDeserialize;
use borsh::io::{Read, Result};

pub struct Stake {
    /// Amount of tokens to stake.
    pub stake: NearToken,
    /// Validator key which will be used to sign transactions on behalf of signer_id
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for Stake {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        let stake: NearToken = BorshDeserialize::deserialize_reader(rd)?;
        Ok(Self {
            stake,
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}
