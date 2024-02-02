use crate::{
    io::{Read, Result},
    parsing::{borsh::BorshDeserialize, types::TxPublicKey},
};

use super::Balance;

pub struct Stake {
    /// Amount of tokens to stake.
    pub stake: Balance,
    /// Validator key which will be used to sign transactions on behalf of signer_id
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for Stake {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        let stake = u128::deserialize_reader(rd)?;
        Ok(Self {
            stake,
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}

impl Stake {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing stake action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing stake action finish:\n");
    }
}
