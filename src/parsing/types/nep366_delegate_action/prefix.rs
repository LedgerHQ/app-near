use crate::{
    io::{Read, Result},
    parsing::borsh::BorshDeserialize,
    utils::types::capped_string::CappedString,
};

pub struct Prefix {
    /// Signer of the delegated actions
    pub sender_id: CappedString<64>,
    /// Receiver of the delegated actions.
    pub receiver_id: CappedString<64>,
    pub number_of_actions: u32,
}

impl Prefix {
    pub fn new() -> Self {
        Self {
            sender_id: CappedString::new(),
            receiver_id: CappedString::new(),
            number_of_actions: 0,
        }
    }
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.sender_id.deserialize_reader_in_place(reader)?;
        self.receiver_id.deserialize_reader_in_place(reader)?;

        let number_of_actions: u32 = BorshDeserialize::deserialize_reader(reader)?;
        self.number_of_actions = number_of_actions;

        Ok(())
    }
}

#[cfg(feature = "speculos")]
impl Prefix {
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;
        testing::debug_print("debug printing delegate action prefix:\n");
        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");

        testing::debug_print("debug printing delegate action prefix finish:\n\n");
    }
}
