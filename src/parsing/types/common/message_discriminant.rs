const MIN_OFF_CHAIN_DISCRIMINANT: u32 = 1 << 31;

const MIN_ON_CHAIN_DISCRIMINANT: u32 = 1 << 30;

pub const NEP_366_META_TRANSACTIONS: u32 = MIN_ON_CHAIN_DISCRIMINANT + 366;
pub const NEP_413_SIGN_MESSAGE: u32 = MIN_OFF_CHAIN_DISCRIMINANT + 413;

pub struct MessageDiscriminant {
    /// The unique prefix, serialized in little-endian by borsh.
    discriminant: u32,
}

#[derive(Debug)]
pub enum CreateDiscriminantError {
    NepTooLarge(u32),
}

impl MessageDiscriminant {
    pub fn new(nep: u32) -> Self {
        Self { discriminant: nep }
    }

    pub fn borsh_serialize(&self) -> [u8; 4] {
        self.discriminant.to_le_bytes()
    }
}
