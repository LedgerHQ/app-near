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

