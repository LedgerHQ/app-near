use crate::{
    io::{Read, Result},
    parsing::{borsh::BorshDeserialize, types::TxPublicKey},
};

pub struct DeleteKey {
    /// A public key associated with the access_key to be deleted.
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for DeleteKey {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self {
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}
