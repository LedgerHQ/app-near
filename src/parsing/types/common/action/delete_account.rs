use crate::app_ui::aliases::CappedAccountId;
use borsh::io::{Read, Result};

pub struct DeleteAccount {
    pub beneficiary_id: CappedAccountId,
}

impl DeleteAccount {
    pub fn new() -> Self {
        Self {
            beneficiary_id: CappedAccountId::new(),
        }
    }
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.beneficiary_id.deserialize_reader_in_place(reader)?;

        Ok(())
    }
}
