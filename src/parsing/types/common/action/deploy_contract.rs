use crate::io::{Error, ErrorKind, Read, Result};
use crate::parsing::borsh::BorshDeserialize;
use crate::parsing::HashingStream;
use crate::utils::types::base58_buf::Base58Buf;

pub struct DeployContract {
    /// WebAssembly binary (hash)
    pub code_sha256: Base58Buf<50>,
}

impl BorshDeserialize for DeployContract {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;

        let mut buf: [u8; 100] = [0u8; 100];

        let chunks = bytes_count / 100;
        let remainder = bytes_count % 100;

        let mut stream =
            HashingStream::new(reader).map_err(|_err| Error::from(ErrorKind::Other))?;
        for _i in 0..(chunks as usize) {
            stream.read_exact(&mut buf)?;
        }
        stream.read_exact(&mut buf[0..(remainder as usize)])?;
        let digest = stream
            .finalize()
            .map_err(|_err| Error::from(ErrorKind::Other))?;

        let mut code_sha256 = Base58Buf::new();
        code_sha256.encode(&digest.0).unwrap();
        let r = Self { code_sha256 };
        Ok(r)
    }
}
