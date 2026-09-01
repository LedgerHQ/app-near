use crate::app_ui::aliases::CappedAccountId;
use crate::utils::types::base58_buf::Base58Buf;
use borsh::BorshDeserialize;
use borsh::io::{Error, ErrorKind, Read, Result};
use ledger_device_sdk::hash::HashInit;

/// Also known as GlobalContractIdentifier
#[repr(u8)]
pub enum GlobalContractIdentifier {
    /// 50 bytes is enough to store base58 of a sha256 hash of deployed code
    CodeHash(Base58Buf<50>) = 0,
    AccountId(CappedAccountId) = 1,
}

impl GlobalContractIdentifier {
    pub fn deserialize_and_hash<R: Read, H: HashInit>(
        reader: &mut R,
        hasher: &mut H,
    ) -> Result<Self> {
        let discriminant: u8 = u8::deserialize_reader(reader)?;

        hasher
            .update(&[discriminant])
            .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;

        match discriminant {
            0 => {
                let mut buf: [u8; 32] = [0u8; 32];
                let mut code_hash = Base58Buf::new();

                reader.read_exact(&mut buf)?;

                hasher
                    .update(&buf)
                    .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;

                // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
                // is not expected to be encountered on encoding 32 bytes to 50 bytes long buffer
                code_hash.encode(&buf).unwrap();

                Ok(Self::CodeHash(code_hash))
            }
            1 => {
                let mut account_id = CappedAccountId::new();
                let size = u32::deserialize_reader(reader)?;

                account_id.deserialize_with_bytes_count(reader, size)?;

                hasher
                    .update(size.to_le_bytes().as_ref())
                    .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;
                // TODO: double-check if it works.
                hasher
                    .update(account_id.as_bytes())
                    .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;

                Ok(Self::AccountId(account_id))
            }
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl BorshDeserialize for GlobalContractIdentifier {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let discriminant: u8 = u8::deserialize_reader(reader)?;

        match discriminant {
            0 => {
                let mut buf: [u8; 32] = [0u8; 32];
                let mut code_hash = Base58Buf::new();

                reader.read_exact(&mut buf)?;

                // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
                // is not expected to be encountered on encoding 32 bytes to 50 bytes long buffer
                code_hash.encode(&buf).unwrap();

                Ok(Self::CodeHash(code_hash))
            }
            1 => {
                let mut account_id = CappedAccountId::new();

                account_id.deserialize_reader_in_place(reader)?;

                Ok(Self::AccountId(account_id))
            }
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}
