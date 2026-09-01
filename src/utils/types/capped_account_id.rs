use crate::utils::types::capped_string::CappedString;

/// A capped string for storing
/// https://docs.rs/near-account-id/1.0.0/near_account_id/struct.AccountId.html
/// where all bytes after 64-byte prefix are truncated and displayed as `... N bytes` ellipsis
///
/// 64 is enough to show implicit account ID-s and most of
/// practical named account ID-s
pub type CappedAccountId = CappedString<64>;

impl CappedAccountId {
    /// This function can be used before [as_str](crate::utils::types::capped_string::CappedString::as_str) to access internal
    /// 64 bytes-long buffer.
    /// Please note that [as_str](crate::utils::types::capped_string::CappedString::as_str) will modify internal buffer
    /// in order to make string displayable on Ledger devices,
    /// so `account_id.as_str().as_bytes()` will be different from `account_id.as_bytes()`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.used]
    }
}
