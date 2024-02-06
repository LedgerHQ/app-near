const MIN_OFF_CHAIN_DISCRIMINANT: u32 = 1 << 31;
const MAX_OFF_CHAIN_DISCRIMINANT: u32 = u32::MAX;

const MAX_ON_CHAIN_DISCRIMINANT: u32 = (1 << 31) - 1;
const MIN_ON_CHAIN_DISCRIMINANT: u32 = 1 << 30;

pub const NEP_366_META_TRANSACTIONS: u32 = 366;

pub struct MessageDiscriminant {
    /// The unique prefix, serialized in little-endian by borsh.
    discriminant: u32,
}

#[derive(Debug)]
pub enum CreateDiscriminantError {
    NepTooLarge(u32),
}

impl MessageDiscriminant {
    /// Create a discriminant for an on-chain actionable message that was introduced in the specified NEP.
    ///
    /// Allows creating discriminants currently unknown in this crate, which can
    /// be useful to prototype new standards. For example, when the client
    /// project still relies on an older version of this crate while nightly
    /// nearcore already supports a new NEP.
    pub fn new_on_chain(nep: u32) -> Result<Self, CreateDiscriminantError> {
        // unchecked arithmetic: these are constants
        if nep > MAX_ON_CHAIN_DISCRIMINANT - MIN_ON_CHAIN_DISCRIMINANT {
            Err(CreateDiscriminantError::NepTooLarge(nep))
        } else {
            Ok(Self {
                // unchecked arithmetic: just checked range
                discriminant: MIN_ON_CHAIN_DISCRIMINANT + nep,
            })
        }
    }
    pub fn new_off_chain(nep: u32) -> Result<Self, CreateDiscriminantError> {
        // unchecked arithmetic: these are constants
        if nep > MAX_OFF_CHAIN_DISCRIMINANT - MIN_OFF_CHAIN_DISCRIMINANT {
            Err(CreateDiscriminantError::NepTooLarge(nep))
        } else {
            Ok(Self {
                // unchecked arithmetic: just checked range
                discriminant: MIN_OFF_CHAIN_DISCRIMINANT + nep,
            })
        }
    }

    pub fn borsh_serialize(&self) -> [u8; 4] {
        self.discriminant.to_le_bytes()
    }
}
