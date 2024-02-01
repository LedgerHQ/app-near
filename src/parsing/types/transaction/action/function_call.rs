use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
    utils::types::capped_string::CappedString,
};

use super::{Balance, Gas};

pub struct FunctionCallCommon {
    pub method_name: CappedString<50>,
    pub gas: Gas,
    pub deposit: Balance,
}

impl FunctionCallCommon {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing function call common  action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing function call common  action finish:\n");
    }
}

impl FunctionCallCommon {
    pub fn deserialize_with_method_name<R: Read>(
        reader: &mut R,
        method_name: CappedString<50>,
    ) -> Result<Self> {
        let gas: Gas = BorshDeserialize::deserialize_reader(reader)?;
        let deposit: Balance = BorshDeserialize::deserialize_reader(reader)?;

        let r = Self {
            method_name,
            gas,
            deposit,
        };
        Ok(r)
    }
}
