use crate::parsing::HashingStream;
use crate::utils::types::base58_buf::Base58Buf;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

const CHUNK_SIZE: usize = 100;

#[repr(u8)]
pub enum GlobalContractDeployMode {
    CodeHash = 0,
    AccountId = 1,
}

pub struct DeployGlobalContract {
    pub code_sha256: Base58Buf<50>,
    pub deploy_mode: GlobalContractDeployMode,
}

impl BorshDeserialize for GlobalContractDeployMode {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let variant: u8 = u8::deserialize_reader(reader)?;

        match variant {
            0 => Ok(GlobalContractDeployMode::CodeHash),
            1 => Ok(GlobalContractDeployMode::AccountId),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl BorshDeserialize for DeployGlobalContract {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;

        let mut buf: [u8; CHUNK_SIZE] = [0u8; CHUNK_SIZE];

        let chunks = bytes_count / (CHUNK_SIZE as u32);
        let remainder = bytes_count % (CHUNK_SIZE as u32);

        // XXX: is this a right way of borrowing? At least compiler suggested it..
        let mut stream =
            HashingStream::new(&mut *reader).map_err(|_err| Error::from(ErrorKind::Other))?;

        for _i in 0..(chunks as usize) {
            stream.read_exact(&mut buf)?;
        }
        stream.read_exact(&mut buf[0..(remainder as usize)])?;
        let digest = stream
            .finalize()
            .map_err(|_err| Error::from(ErrorKind::Other))?;

        let mut code_sha256 = Base58Buf::new();
        // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
        // is not expected to be encountered on encoding 32 bytes to 50 bytes long buffer
        code_sha256.encode(&digest.0).unwrap();

        let deploy_mode: GlobalContractDeployMode =
            GlobalContractDeployMode::deserialize_reader(reader)?;

        Ok(Self {
            code_sha256,
            deploy_mode,
        })
    }
}
