use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
};

use super::Balance;

pub struct Transfer {
    pub deposit: Balance,
}

impl BorshDeserialize for Transfer {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let balance = u128::deserialize_reader(reader)?;
        Ok(Self { deposit: balance })
    }
}

impl Transfer {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        use crate::parsing::types::action::ONE_NEAR;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing transfer action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing amount:\n");

        testing::debug_print(self.deposit.numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        let deposit = (self.deposit as f64) / (ONE_NEAR as f64);
        let mut buffer = dtoa::Buffer::new();
        let printed = buffer.format(deposit);
        testing::debug_print(printed);
        testing::debug_print("\n");
        testing::debug_print("debug printing amount finish:\n\n");
        testing::debug_print("debug printing transfer action finish:\n");
    }
}
