use borsh::BorshDeserialize;
use borsh::io::{self, ErrorKind, Read, Result};
use ledger_device_sdk::hash::HashInit;
use ledger_device_sdk::hash::sha3::Sha3_256;

pub enum KeyType {
    ED25519 = 0,
    SECP256K1 = 1,
    MlDsa65 = 2,
}

#[derive(Clone, Copy)]
pub enum TxPublicKey {
    /// 256 bit elliptic curve based public-key.
    ED25519([u8; 32]),
    /// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
    SECP256K1([u8; 64]),
    /// SHA-3 hash of FIPS 204 ML-DSA-65 post-quantum public key.
    MlDsa65Hash([u8; 32]),
}

impl TryFrom<u8> for KeyType {
    type Error = borsh::io::Error;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyType::ED25519),
            1 => Ok(KeyType::SECP256K1),
            2 => Ok(KeyType::MlDsa65),
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
            KeyType::MlDsa65 => {
                let mut sha3_256 = Sha3_256::new();
                let mut buf: [u8; 32] = [0u8; 32];

                // Domain separator for the hash as specified in [NEP-645](https://github.com/near/NEPs/blob/master/neps/nep-0645.md#on-trie-storage-publickeyhandle-and-hashing)
                sha3_256
                    .update(b"near:ml-dsa-65-pubkey-hash:v1")
                    .map_err(|_err| io::ErrorKind::OutOfMemory)?;

                // We expect to receive ML-DSA-65 pubkey, which has 1952 bytes.
                // Hence, 1952 / 32 = 61
                for _ in 0..61 {
                    buf = BorshDeserialize::deserialize_reader(rd)?;
                    sha3_256
                        .update(&buf)
                        .map_err(|_err| io::ErrorKind::OutOfMemory)?;
                }

                sha3_256
                    .finalize(&mut buf)
                    .map_err(|_err| io::ErrorKind::OutOfMemory)?;

                Ok(TxPublicKey::MlDsa65Hash(buf))
            }
        }
    }
}
