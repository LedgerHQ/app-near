use crate::app_ui::aliases::CappedAccountId;
use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

pub struct Prefix {
    /// Signer of the delegated actions
    pub sender_id: CappedAccountId,
    /// Receiver of the delegated actions.
    pub receiver_id: CappedAccountId,
    pub number_of_actions: u32,
}

impl Prefix {
    pub fn new() -> Self {
        Self {
            sender_id: CappedAccountId::new(),
            receiver_id: CappedAccountId::new(),
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
