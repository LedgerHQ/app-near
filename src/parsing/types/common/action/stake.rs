use crate::{
    io::{Read, Result},
    parsing::{
        borsh::BorshDeserialize,
        types::{common::near_token::NearToken, TxPublicKey},
    },
};

pub struct Stake {
    /// Amount of tokens to stake.
    pub stake: NearToken,
    /// Validator key which will be used to sign transactions on behalf of signer_id
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for Stake {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        let stake = u128::deserialize_reader(rd)?;
        Ok(Self {
            stake: NearToken::from_yoctonear(stake),
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}
