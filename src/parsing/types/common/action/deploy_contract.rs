use crate::parsing::HashingStream;
use crate::utils::types::base58_buf::Base58Buf;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

/// arbitrary chunk size, which is set to around 40% of apdu buffer size
/// in order to not consume much stack space on `nanos`
const CHUNK_SIZE: usize = 100;

pub struct DeployContract {
    /// WebAssembly binary (hash)
    /// 50 bytes is enough to store base58 of a sha256 hash
    pub code_sha256: Base58Buf<50>,
}

impl BorshDeserialize for DeployContract {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;

        let mut buf: [u8; CHUNK_SIZE] = [0u8; CHUNK_SIZE];

        let chunks = bytes_count / (CHUNK_SIZE as u32);
        let remainder = bytes_count % (CHUNK_SIZE as u32);

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
        // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
        // is not expected to be encountered on encoding 32 bytes to 50 bytes long buffer
        code_sha256.encode(&digest.0).unwrap();
        let r = Self { code_sha256 };
        Ok(r)
    }
}
