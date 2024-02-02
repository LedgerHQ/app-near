use crate::{
    io::{Read, Result},
    parsing::{
        borsh::BorshDeserialize,
        types::{
            common::action::{BlockHeight, Nonce},
            TxPublicKey,
        },
    },
};

pub struct Suffix {
    /// Nonce to ensure that the same delegate action is not sent twice by a
    /// relayer and should match for given account's `public_key`.
    /// After this action is processed it will increment.
    pub nonce: Nonce,
    /// The maximal height of the block in the blockchain below which the given DelegateAction is valid.
    pub max_block_height: BlockHeight,
    /// Public key used to sign this delegated action.
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for Suffix {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self {
            nonce: BorshDeserialize::deserialize_reader(rd)?,
            max_block_height: BorshDeserialize::deserialize_reader(rd)?,
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}

#[cfg(feature = "speculos")]
impl Suffix {
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;
        testing::debug_print("debug printing delegate action suffix:\n");
        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");

        testing::debug_print("debug printing delegate action suffix finish:\n\n");
    }
}
