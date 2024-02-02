#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use crate::{
    io::{Read, Result},
    parsing::{borsh::BorshDeserialize, types::TxPublicKey},
    utils::types::capped_string::CappedString,
};

pub struct TransactionPrefix {
    pub signer_id: CappedString<64>,
    pub receiver_id: CappedString<64>,
    pub number_of_actions: u32,
}

pub struct CryptoHash(pub [u8; 32]);

impl BorshDeserialize for CryptoHash {
    fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self> {
        Ok(Self(BorshDeserialize::deserialize_reader(rd)?))
    }
}

impl TransactionPrefix {
    // NOTE: using this instead of `BorshDeserialize`
    // allows to increase available buffers
    pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.signer_id.deserialize_reader_in_place(reader)?;
        let pk: TxPublicKey = BorshDeserialize::deserialize_reader(reader)?;
        drop(pk);
        let nonce: u64 = BorshDeserialize::deserialize_reader(reader)?;
        #[allow(dropping_copy_types)]
        drop(nonce);
        self.receiver_id.deserialize_reader_in_place(reader)?;

        let crypto_hash: CryptoHash = BorshDeserialize::deserialize_reader(reader)?;
        drop(crypto_hash);

        let number_of_actions: u32 = BorshDeserialize::deserialize_reader(reader)?;
        self.number_of_actions = number_of_actions;

        Ok(())
    }
}

#[cfg(feature = "speculos")]
impl TransactionPrefix {
    pub fn debug_print(&self) {
        use numtoa::NumToA;
        testing::debug_print("debug printing tx_prefix:\n");
        let mut numtoa_buf = [0u8; 40];

        testing::debug_print("size of self: \n");
        testing::debug_print(core::mem::size_of_val(self).numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");

        testing::debug_print(self.signer_id.as_str());
        testing::debug_print("\n");
        testing::debug_print(self.receiver_id.as_str());
        testing::debug_print("\n");
        testing::debug_print(self.number_of_actions.numtoa_str(10, &mut numtoa_buf));
        testing::debug_print("\n");

        testing::debug_print("debug printing tx prefix finish:\n\n");
    }
}
