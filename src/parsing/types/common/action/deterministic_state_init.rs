use borsh::{
    BorshDeserialize,
    io::{Error, ErrorKind, Read, Result},
};

use ledger_device_sdk::hash::HashInit;
use near_token::NearToken;

use crate::parsing::types::GlobalContractIdentifier;

#[repr(u8)]
pub enum DeterministicAccountStateInit {
    V1(DeterministicAccountStateInitV1) = 0,
}

pub struct DeterministicAccountStateInitV1 {
    pub code: GlobalContractIdentifier,
    // Considering that contract storage would usually not exceed 10Mb or 10Gb (in extreme cases),
    // u64::MAX should be more than enough to store amount of bytes this contract state has.
    pub data_size_bytes: u64,
    // BTreeMap serialized by borsh could have at most u32::MAX entries
    pub data_entries: u32,
}

pub struct DeterministicAccountStateInitPostfix {
    pub deposit: NearToken,
}

const BUF_SIZE: usize = 64;

#[inline]
fn read_exact_and_hash<R: Read, H: HashInit>(
    reader: &mut R,
    hasher: &mut H,
    num: u32,
) -> Result<()> {
    let mut buf = [0u8; BUF_SIZE];

    let num_loops: usize = (num as usize) / BUF_SIZE;
    let remainder: usize = (num as usize) % BUF_SIZE;

    for _ in 0..num_loops {
        reader.read_exact(&mut buf)?;
        hasher
            .update(&buf)
            .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;
    }

    if remainder != 0 {
        reader.read_exact(&mut buf[..remainder])?;
        hasher
            .update(&buf[..remainder])
            .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;
    }

    Ok(())
}

impl DeterministicAccountStateInit {
    pub fn deserialize_and_hash<R: Read, H: HashInit>(
        reader: &mut R,
        hasher: &mut H,
    ) -> Result<Self> {
        let version = u8::deserialize_reader(reader)?;

        hasher
            .update(&[version])
            .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;

        match version {
            0 => Ok(Self::V1(
                DeterministicAccountStateInitV1::deserialize_and_hash(reader, hasher)?,
            )),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl DeterministicAccountStateInitV1 {
    pub fn deserialize_and_hash<R: Read, H: HashInit>(
        reader: &mut R,
        hasher: &mut H,
    ) -> Result<Self> {
        let contract = GlobalContractIdentifier::deserialize_and_hash(reader, hasher)?;

        let num_entries = u32::deserialize_reader(reader)?;
        hasher
            .update(num_entries.to_le_bytes().as_ref())
            .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;
        let mut total_bytes: u64 = 0;

        for _ in 0..num_entries {
            for _ in 0..2 {
                let len = u32::deserialize_reader(reader)?;
                hasher
                    .update(&len.to_le_bytes())
                    .map_err(|_err| Error::from(ErrorKind::OutOfMemory))?;
                total_bytes = total_bytes.saturating_add(len as u64);
                read_exact_and_hash(reader, hasher, len)?;
            }
        }

        Ok(Self {
            code: contract,
            data_size_bytes: total_bytes,
            data_entries: num_entries,
        })
    }
}

impl BorshDeserialize for DeterministicAccountStateInitPostfix {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self {
            deposit: NearToken::deserialize_reader(reader)?,
        })
    }
}
