use core::iter::zip;
use core::mem;

use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

pub const ALLOWED_PATH_LEN: usize = 5;
pub struct PathBip32(pub [u32; ALLOWED_PATH_LEN]);

impl PathBip32 {
    fn new() -> Self {
        Self([0u32; ALLOWED_PATH_LEN])
    }
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut result = Self::new();
        if data.len() != ALLOWED_PATH_LEN * mem::size_of::<u32>() {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        for (path_element, chunk) in zip(result.0.iter_mut(), data.chunks(mem::size_of::<u32>())) {
            // .unwrap() is ok, as `chunk.len() == 4` holds true
            // https://doc.rust-lang.org/std/primitive.array.html#impl-TryFrom%3C%26%5BT%5D%3E-for-%5BT;+N%5D
            *path_element = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        Ok(result)
    }
}

fn unexpected_eof_to_unexpected_length_of_input(e: Error) -> Error {
    if e.kind() == ErrorKind::UnexpectedEof {
        Error::from(ErrorKind::InvalidData)
    } else {
        e
    }
}

impl BorshDeserialize for PathBip32 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; ALLOWED_PATH_LEN * mem::size_of::<u32>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Self::parse(&buf)
    }
}
