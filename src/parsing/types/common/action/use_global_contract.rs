use crate::app_ui::aliases::CappedAccountId;
use crate::parsing::HashingStream;
use crate::utils::types::base58_buf::Base58Buf;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

#[repr(u8)]
pub enum UseGlobalContract {
    /// 50 bytes is enough to store base58 of a sha256 hash of deployed code
    CodeHash(Base58Buf<50>) = 0,
    AccountId(CappedAccountId) = 1,
}

impl BorshDeserialize for UseGlobalContract {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let discriminant: u8 = u8::deserialize_reader(reader)?;

        match discriminant {
            0 => {
                let mut buf: [u8; 32] = [0u8; 32];

                let mut stream =
                    HashingStream::new(reader).map_err(|_err| Error::from(ErrorKind::Other))?;

                stream.read_exact(&mut buf)?;

                let mut code_hash = Base58Buf::new();
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
