use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
};

pub struct CreateAccount {}

impl BorshDeserialize for CreateAccount {
    fn deserialize_reader<R: Read>(_reader: &mut R) -> Result<Self> {
        Ok(Self {})
    }
}

impl CreateAccount {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing create account action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing create account action finish:\n");
    }
}
