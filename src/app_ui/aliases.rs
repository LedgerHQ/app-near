use fmt_buffer::Buffer;

use crate::utils::types::{capped_string::CappedString, hex_display::HexDisplay};

/// A capped string for storing
/// https://docs.rs/near-account-id/1.0.0/near_account_id/struct.AccountId.html
///
/// An account ID longer than this buffer is rejected during parsing.
/// 64 is the maximum length of a valid account ID, which also fits
/// implicit account ID-s: `MAX_LEN` in near-account-id
/// (https://github.com/near/near-account-id-rs/blob/v3.0.0/src/validation.rs),
/// the version nearcore uses to reject longer IDs when it deserializes a transaction.
pub type CappedAccountId = CappedString<64>;

/// A buffer, large enough to contain string representation
/// of u64
pub type U64Buffer = [u8; 20];
/// A buffer, large enough to contain string representation
/// of u32
pub type U32Buffer = [u8; 10];

/// Type, which is used for displaying full `args` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/action/struct.FunctionCallAction.html
/// in hexadecimal form, when `args` are considered just arbitrary bytes.
/// `args` that do not fit are rejected before the review.
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type FnCallHexDisplay = HexDisplay<200>;

/// Type, which is used for displaying full `args` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/action/struct.FunctionCallAction.html
/// as string, when `args` is considered to be a valid utf-8 string.
/// `args` that do not fit are rejected before the review.
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type FnCallCappedString = CappedString<200>;

/// Type, which is used for displaying full `method_names` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/account/struct.FunctionCallPermission.html
///
/// A list that does not fit is rejected during parsing.
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type MethodNamesBuffer = Buffer<210>;

/// Type, which is used for displaying full `message` field of
/// https://docs.rs/near-ledger/0.5.0/near_ledger/struct.NEP413Payload.html
///
/// A `message` that does not fit is rejected before the review.
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type NEP413CappedString = CappedString<400>;
