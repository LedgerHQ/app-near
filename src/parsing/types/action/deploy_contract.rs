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

impl DeployContract {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing deploy contract  action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing deploy contract  action finish:\n");
    }
}
