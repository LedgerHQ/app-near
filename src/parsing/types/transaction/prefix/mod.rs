use crate::app_ui::aliases::CappedAccountId;
use crate::parsing::types::TxPublicKey;
use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct Prefix {
    pub signer_id: CappedAccountId,
    pub receiver_id: CappedAccountId,
    pub public_key: TxPublicKey,
    pub number_of_actions: u32,
}

/// hash, which is `sha2::Sha256`
pub struct CryptoHash(pub [u8; 32]);

impl BorshDeserialize for CryptoHash {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self(BorshDeserialize::deserialize_reader(rd)?))
    }
}

impl Prefix {
    pub fn new() -> Self {
        Self {
            signer_id: CappedAccountId::new(),
            receiver_id: CappedAccountId::new(),
            public_key: TxPublicKey::ED25519([0u8; 32]),
            number_of_actions: 0,
        }
    }
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.signer_id.deserialize_reader_in_place(reader)?;
        let pk: TxPublicKey = BorshDeserialize::deserialize_reader(reader)?;

        self.public_key = pk;
        let _nonce: u64 = BorshDeserialize::deserialize_reader(reader)?;
        self.receiver_id.deserialize_reader_in_place(reader)?;

        let _crypto_hash: CryptoHash = BorshDeserialize::deserialize_reader(reader)?;

        let number_of_actions: u32 = BorshDeserialize::deserialize_reader(reader)?;
        self.number_of_actions = number_of_actions;

        Ok(())
    }
}
