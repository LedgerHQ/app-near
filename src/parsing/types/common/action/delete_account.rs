use crate::utils::types::capped_string::CappedString;
use borsh::io::{Read, Result};

pub struct DeleteAccount {
    pub beneficiary_id: CappedString<64>,
}

impl DeleteAccount {
    pub fn new() -> Self {
        Self {
            beneficiary_id: CappedString::new(),
        }
    }
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.beneficiary_id.deserialize_reader_in_place(reader)?;

        Ok(())
    }
}
