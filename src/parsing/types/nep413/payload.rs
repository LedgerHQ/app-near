use crate::utils::types::capped_string::CappedString;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

pub struct Payload {
    pub message: CappedString<400>,
    pub nonce: [u8; 32],
    pub recipient: CappedString<64>,
    pub callback_url: Option<CappedString<100>>,
}

impl Payload {
    pub fn new() -> Self {
        Self {
            message: CappedString::new(),
            nonce: [0u8; 32],
            recipient: CappedString::new(),
            callback_url: Some(CappedString::new()),
        }
    }
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.message.deserialize_reader_in_place(reader)?;

        let nonce: [u8; 32] = BorshDeserialize::deserialize_reader(reader)?;
        self.nonce = nonce;

        self.recipient.deserialize_reader_in_place(reader)?;

        let option_flag: u8 = BorshDeserialize::deserialize_reader(reader)?;

        if option_flag == 0 {
            self.callback_url = None;
        } else if option_flag == 1 {
            self.callback_url
                .as_mut()
                .unwrap()
                .deserialize_reader_in_place(reader)?;
        } else {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        Ok(())
    }
}
