#![no_std]
pub mod io;

use crate::io::{Error, ErrorKind, Read, Result};
use core::mem::size_of;

/// A data-structure that can be de-serialized from binary format by NBOR.
pub trait BorshDeserialize: Sized {
    /// Deserializes this instance from a given slice of bytes.
    /// Updates the buffer to point at the remaining bytes.
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        Self::deserialize_reader(&mut *buf)
    }

    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self>;

    /// Deserialize this instance from a slice of bytes.
    fn try_from_slice(v: &[u8]) -> Result<Self> {
        let mut v_mut = v;
        let result = Self::deserialize(&mut v_mut)?;
        if !v_mut.is_empty() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        Ok(result)
    }

    fn try_from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let result = Self::deserialize_reader(reader)?;
        let mut buf = [0u8; 1];
        match reader.read_exact(&mut buf) {
            Err(f) if f.kind() == ErrorKind::UnexpectedEof => Ok(result),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

fn unexpected_eof_to_unexpected_length_of_input(e: Error) -> Error {
    if e.kind() == ErrorKind::UnexpectedEof {
        Error::from(ErrorKind::InvalidData)
    } else {
        e
    }
}

impl BorshDeserialize for u8 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(buf[0])
    }
}

impl BorshDeserialize for u32 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u32>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u32::from_le_bytes(buf);
        Ok(res)
    }
}

impl BorshDeserialize for u64 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u64>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u64::from_le_bytes(buf);
        Ok(res)
    }
}

impl BorshDeserialize for u128 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u128>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u128::from_le_bytes(buf);
        Ok(res)
    }
}

impl<const N: usize> BorshDeserialize for [u8; N] {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut arr = [0u8; N];
        reader
            .read_exact(&mut arr)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(arr)
    }
}

impl<T> BorshDeserialize for Option<T>
where
    T: BorshDeserialize,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
        if flag == 0 {
            Ok(None)
        } else if flag == 1 {
            Ok(Some(T::deserialize_reader(reader)?))
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}
