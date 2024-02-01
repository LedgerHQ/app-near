const MIN_OFF_CHAIN_DISCRIMINANT: u32 = 1 << 31;
const MAX_OFF_CHAIN_DISCRIMINANT: u32 = u32::MAX;

#[derive(Debug)]
pub struct MessageDiscriminant {
    /// The unique prefix, serialized in little-endian by borsh.
    discriminant: u32,
}

#[derive(Debug)]
pub enum CreateDiscriminantError {
    NepTooLarge(u32),
}

impl MessageDiscriminant {
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
