use fmt_buffer::Buffer;

use crate::utils::types::{capped_string::CappedString, hex_display::HexDisplay};

/// A capped string for storing
/// https://docs.rs/near-account-id/1.0.0/near_account_id/struct.AccountId.html
/// where all bytes after 64-byte prefix are truncated and displayed as `... N bytes` ellipsis
///
/// 64 is enough to show implicit account ID-s and most of
/// practical named account ID-s
pub type CappedAccountId = CappedString<64>;

/// A buffer, large enough to contain string representation
/// of u64
pub type U64Buffer = [u8; 20];
/// A buffer, large enough to contain string representation
/// of u32
pub type U32Buffer = [u8; 10];

/// Type, which is used for displaying full `args` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/action/struct.FunctionCallAction.html
/// or prefix of `args` with suffix truncated and displayed as `... N bytes` ellipsis,
/// displayed in hexadecimal form, when `args` are considered just arbitrary bytes.
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type FnCallHexDisplay = HexDisplay<200>;

/// Type, which is used for displaying full `args` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/action/struct.FunctionCallAction.html
/// or prefix of `args` with suffix truncated and displayed as `... N bytes` ellipsis,
/// displayed as string, when `args` is considered to be a valid utf-8 string
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type FnCallCappedString = CappedString<200>;

/// Type, which is used for displaying full `method_names` field of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/account/struct.FunctionCallPermission.html
/// or prefix of `method_names` with suffix truncated and displayed as `... N bytes` ellipsis
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type MethodNamesBuffer = Buffer<210>;

/// Type, which is used for displaying full `message` field of
/// https://docs.rs/near-ledger/0.5.0/near_ledger/struct.NEP413Payload.html
/// or prefix of `message` with suffix truncated and displayed as `... N bytes` ellipsis
///
/// Current parameter value is limited by `nanos` capabilities.
/// The parameter value can be made larger with `#[cfg(target_os = "...")]`
/// for other platforms
/// see https://github.com/dj8yfo/app-near-rs/pull/45/files
pub type NEP413CappedString = CappedString<400>;
