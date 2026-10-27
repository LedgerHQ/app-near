use crate::app_ui::aliases::{CappedAccountId, NEP413CappedString};
use crate::utils::types::capped_string::CappedString;
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;

/// buffer to store nonce, corresponding to
/// https://github.com/near/NEPs/blob/master/neps/nep-0413.md and
/// `nonce` field of https://docs.rs/near-ledger/0.5.0/near_ledger/struct.NEP413Payload.html
pub type NonceBuffer = [u8; 32];

/// a buffer for storing `callback_url`, should be long enough
/// for most of practical cases
pub type CappedCallbackUrl = CappedString<100>;

pub struct Payload {
    pub message: NEP413CappedString,
    /// Declared length of `message`, which may exceed what the buffer holds.
    pub message_len: u32,
    pub nonce: NonceBuffer,
    pub recipient: CappedAccountId,
    pub callback_url: Option<CappedCallbackUrl>,
    /// Declared length of `callback_url`, which may exceed what the buffer holds.
    pub callback_url_len: u32,
}

impl Payload {
    pub fn new() -> Self {
        Self {
            message: NEP413CappedString::new(),
            message_len: 0,
            nonce: NonceBuffer::default(),
            recipient: CappedAccountId::new(),
            callback_url: Some(CappedCallbackUrl::new()),
            callback_url_len: 0,
        }
    }
    /// must be only called once after `Self::new` to avoid unexpected panics
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.message_len = u32::deserialize_reader(reader)?;
        self.message
            .deserialize_with_bytes_count(reader, self.message_len)?;

        let nonce: NonceBuffer = BorshDeserialize::deserialize_reader(reader)?;
        self.nonce = nonce;

        self.recipient.deserialize_reader_in_place_whole(reader)?;

        let option_flag: u8 = BorshDeserialize::deserialize_reader(reader)?;

        if option_flag == 0 {
            self.callback_url = None;
        } else if option_flag == 1 {
            // .unwrap() is ok if `self.callback_url` is Some, which holds true
            // for calling `self.deserialize_reader_in_place` once after `Self::new`
            self.callback_url_len = u32::deserialize_reader(reader)?;
            self.callback_url
                .as_mut()
                .unwrap()
                .deserialize_with_bytes_count(reader, self.callback_url_len)?;
        } else {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        Ok(())
    }
}
