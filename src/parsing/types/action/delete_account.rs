use crate::{
    io::{Read, Result},
    utils::types::capped_string::CappedString,
};

pub struct DeleteAccount {
    pub beneficiary_id: CappedString<64>,
}

impl DeleteAccount {
    pub fn new() -> Self {
        Self {
            beneficiary_id: CappedString::new(false),
        }
        
    }
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.beneficiary_id.deserialize_reader_in_place(reader)?;

        Ok(())
    }
}

impl DeleteAccount {
    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("debug printing delete account action:\n");
        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");
        testing::debug_print("debug printing delete account action finish:\n");
    }
}
