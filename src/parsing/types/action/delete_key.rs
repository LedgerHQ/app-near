use crate::{
    io::{Read, Result},
    parsing::{borsh::BorshDeserialize, types::tx_public_key::TxPublicKey},
};

pub struct DeleteKey {
    /// A public key associated with the access_key to be deleted.
    pub public_key: TxPublicKey,
}

impl BorshDeserialize for DeleteKey {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self {
            public_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}

impl DeleteKey {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing delete key action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing delete key action finish:\n");
    }
}
