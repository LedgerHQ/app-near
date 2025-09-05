use near_gas::NearGas;

use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

use super::prefix::TxVersion;

pub struct V1Suffix {
    pub priority_fee: NearGas,
}

#[repr(u8)]
pub enum TxSuffix {
    V1(V1Suffix),
}

impl BorshDeserialize for V1Suffix {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let gas: NearGas = BorshDeserialize::deserialize_reader(reader)?;

        Ok(Self { priority_fee: gas })
    }
}

impl TxSuffix {
    pub fn deserialize_with_tx_version<R: Read>(
        reader: &mut R,
        tx_version: TxVersion,
    ) -> Result<Self> {
        match tx_version {
            TxVersion::V0 => Err(Error::from(ErrorKind::InvalidData)),
            TxVersion::V1 => Ok(Self::V1(V1Suffix::deserialize_reader(reader)?)),
        }
    }
}
