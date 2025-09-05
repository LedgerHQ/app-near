use crate::app_ui::aliases::CappedAccountId;
use crate::parsing::types::TxPublicKey;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum TxVersion {
    V0,
    V1,
}

pub struct Prefix {
    pub tx_version: TxVersion,
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
            tx_version: TxVersion::V0,
            signer_id: CappedAccountId::new(),
            receiver_id: CappedAccountId::new(),
            public_key: TxPublicKey::ED25519([0u8; 32]),
            number_of_actions: 0,
        }
    }
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        // This is hackery that was made to enable backward-compatible parsing of
        // Transaction::V0.
        let u1: u8 = BorshDeserialize::deserialize_reader(reader)?;
        let u2: u8 = BorshDeserialize::deserialize_reader(reader)?;
        let u3: u8 = BorshDeserialize::deserialize_reader(reader)?;
        let u4: u8 = BorshDeserialize::deserialize_reader(reader)?;

        let read_signer_id = |buf: [u8; 4], reader: &mut R| -> Result<CappedAccountId> {
            let str_len = u32::from_le_bytes(buf);
            let mut account_id = CappedAccountId::new();
            account_id.deserialize_with_bytes_count(reader, str_len)?;
            Ok(account_id)
        };

        if u2 == 0 {
            self.tx_version = TxVersion::V0;
            self.signer_id = read_signer_id([u1, u2, u3, u4], reader)?;
        } else {
            self.tx_version = match u1 {
                1_u8 => TxVersion::V1,
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };

            let u5: u8 = BorshDeserialize::deserialize_reader(reader)?;
            self.signer_id = read_signer_id([u2, u3, u4, u5], reader)?;
        }

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
