use crate::{parsing::types::TxPublicKey, utils::types::capped_string::CappedString};
use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct Prefix {
    pub signer_id: CappedString<64>,
    pub receiver_id: CappedString<64>,
    pub public_key: TxPublicKey,
    pub number_of_actions: u32,
}

pub struct CryptoHash(pub [u8; 32]);

impl BorshDeserialize for CryptoHash {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self(BorshDeserialize::deserialize_reader(rd)?))
    }
}

impl Prefix {
    pub fn new() -> Self {
        Self {
            signer_id: CappedString::new(),
            receiver_id: CappedString::new(),
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
        let nonce: u64 = BorshDeserialize::deserialize_reader(reader)?;
        #[allow(dropping_copy_types)]
        drop(nonce);
        self.receiver_id.deserialize_reader_in_place(reader)?;

        let _crypto_hash: CryptoHash = BorshDeserialize::deserialize_reader(reader)?;

        let number_of_actions: u32 = BorshDeserialize::deserialize_reader(reader)?;
        self.number_of_actions = number_of_actions;

        Ok(())
    }
}
