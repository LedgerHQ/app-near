use crate::app_ui::aliases::{CappedAccountId, MethodNamesBuffer};
use crate::{parsing::types::TxPublicKey, utils::types::capped_string::CappedString};
use borsh::io::{Error, ErrorKind, Read, Result};
use borsh::BorshDeserialize;
use near_token::NearToken;

use super::Nonce;

pub struct AddKey {
    /// A public key which will be associated with an access_key
    pub public_key: TxPublicKey,
    /// An access key with the permission
    pub access_key: AccessKey,
}

pub struct AccessKey {
    /// Nonce for this access key, used for tx nonce generation. When access key is created, nonce
    /// is set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation.
    /// See <https://github.com/near/nearcore/issues/3779> for more details.
    pub nonce: Nonce,

    /// Defines permissions for this access key.
    pub permission: AccessKeyPermission,
}

pub enum AccessKeyPermission {
    FunctionCall,

    /// Grants full access to the account.
    /// NOTE: It's used to replace account-level public keys.
    FullAccess,
}

pub struct FunctionCallPermission {
    /// Allowance is a balance limit to use by this access key to pay for function call gas and
    /// transaction fees. When this access key is used, both account balance and the allowance is
    /// decreased by the same value.
    /// `None` means unlimited allowance.
    /// NOTE: To change or increase the allowance, the old access key needs to be deleted and a new
    /// access key should be created.
    pub allowance: Option<NearToken>,

    // This isn't an AccountId because already existing records in testnet genesis have invalid
    // values for this field (see: https://github.com/near/nearcore/pull/4621#issuecomment-892099860)
    // we accommodate those by using a string, allowing us to read and parse genesis.
    /// The access key only allows transactions with the given receiver's account id.
    pub receiver_id: CappedAccountId,

    pub number_of_method_names: u32,
    /// A list of method names that can be used. The access key only allows transactions with the
    /// function call of one of the given method names.
    /// Empty list means any method name can be used.
    // pub method_names: Vec<String>,
    pub method_names: MethodNamesBuffer,
}

impl BorshDeserialize for AccessKeyPermission {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        let tag = u8::deserialize_reader(rd)?;
        match tag {
            0 => Ok(Self::FunctionCall),
            1 => Ok(Self::FullAccess),
            _unknown_key_type => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl BorshDeserialize for AccessKey {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self {
            nonce: BorshDeserialize::deserialize_reader(rd)?,
            permission: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}

impl BorshDeserialize for AddKey {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self {
            public_key: BorshDeserialize::deserialize_reader(rd)?,
            access_key: BorshDeserialize::deserialize_reader(rd)?,
        })
    }
}

/// buffer to store each individual `method_name` in `method_names` of
/// https://docs.rs/near-primitives/0.21.2/near_primitives/account/struct.FunctionCallPermission.html .
/// If a `method_name` is longer than this buffer during parsing, it's
/// truncated and will be displayed with ellipsis `is_too_long_method_name_suffix_truncated...`
type PerMethodBuffer = CappedString<40>;

impl FunctionCallPermission {
    pub fn new() -> Self {
        Self {
            allowance: None,
            receiver_id: CappedAccountId::new(),
            number_of_method_names: 0,
            method_names: MethodNamesBuffer::new(),
        }
    }
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let allowance: Option<u128> = BorshDeserialize::deserialize_reader(reader)?;
        self.allowance = allowance.map(NearToken::from_yoctonear);
        self.receiver_id.deserialize_reader_in_place(reader)?;

        self.number_of_method_names = BorshDeserialize::deserialize_reader(reader)?;

        let mut per_method_buffer: PerMethodBuffer = PerMethodBuffer::new();
        for _i in 0..(self.number_of_method_names as usize) {
            per_method_buffer.deserialize_reader_in_place(reader)?;

            self.method_names.write_str(per_method_buffer.as_str());
            if per_method_buffer.truncated() {
                self.method_names.write_str("...");
            }
            self.method_names.write_str(";");
        }
        Ok(())
    }
}
