#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use crate::{
    borsh::BorshDeserialize,
    io::{ErrorKind, Read, Result},
    utils::capped_string::CappedString,
};

// NOTE: works in speculos
// signer_id: CappedString<270>,
// receiver_id: CappedString<270>,
// NOTE: works on nanos, but crashes in speculos
// signer_id: CappedString<300>,
// receiver_id: CappedString<300>,
pub struct TransactionPrefix {
    signer_id: CappedString<64>,
    receiver_id: CappedString<64>,
    number_of_actions: u32,
}

pub enum KeyType {
    ED25519 = 0,
    SECP256K1 = 1,
}

pub enum TxPublicKey {
    /// 256 bit elliptic curve based public-key.
    ED25519([u8; 32]),
    /// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
    SECP256K1([u8; 64]),
}

pub struct CryptoHash(pub [u8; 32]);

impl TryFrom<u8> for KeyType {
    type Error = crate::io::Error;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyType::ED25519),
            1 => Ok(KeyType::SECP256K1),
            _unknown_key_type => Err(Self::Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl BorshDeserialize for TxPublicKey {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        let key_type = KeyType::try_from(u8::deserialize_reader(rd)?)?;
        match key_type {
            KeyType::ED25519 => Ok(TxPublicKey::ED25519(BorshDeserialize::deserialize_reader(
                rd,
            )?)),
            KeyType::SECP256K1 => Ok(TxPublicKey::SECP256K1(
                BorshDeserialize::deserialize_reader(rd)?,
            )),
        }
    }
}

impl BorshDeserialize for CryptoHash {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self(BorshDeserialize::deserialize_reader(rd)?))
    }
}

impl BorshDeserialize for TransactionPrefix {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let signer_id: CappedString<64> = BorshDeserialize::deserialize_reader(reader)?;
        let pk: TxPublicKey = BorshDeserialize::deserialize_reader(reader)?;
        drop(pk);
        let nonce: u64 = BorshDeserialize::deserialize_reader(reader)?;
        drop(nonce);
        let receiver_id: CappedString<64> = BorshDeserialize::deserialize_reader(reader)?;

        let crypto_hash: CryptoHash = BorshDeserialize::deserialize_reader(reader)?;
        drop(crypto_hash);

        let number_of_actions: u32 = BorshDeserialize::deserialize_reader(reader)?;

        Ok(Self {
            receiver_id,
            signer_id,
            number_of_actions,
        })
    }
}

#[cfg(feature = "speculos")]
impl TransactionPrefix {
    pub fn debug_print(&self) {
        use numtoa::NumToA;
        testing::debug_print("debug printing tx_prefix:\n");

        testing::debug_print(self.signer_id.as_str());
        testing::debug_print("\n");
        testing::debug_print(self.receiver_id.as_str());
        testing::debug_print("\n");
        let mut numtoa_buf = [0u8; 40];
        testing::debug_print(self.number_of_actions.numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");

        testing::debug_print("debug printing tx prefix finish:\n\n");
    }
}
