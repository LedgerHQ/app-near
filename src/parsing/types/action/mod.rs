use crate::{
    io::{Error, ErrorKind, Read, Result},
    parsing::borsh::BorshDeserialize,
};

/// Balance is type for storing amounts of tokens.
pub type Balance = u128;

pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub struct TransferAction {
    pub deposit: Balance,
}

impl BorshDeserialize for TransferAction {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let balance = u128::deserialize_reader(reader)?;
        Ok(Self { deposit: balance })
    }
}
pub enum Action {
    CreateAccount,
    DeployContract,
    FunctionCall,
    Transfer(TransferAction),
    Stake,
    AddKey,
    DeleteKey,
    DeleteAccount,
    Delegate,
}

impl BorshDeserialize for Action {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let variant_tag = u8::deserialize_reader(reader)?;
        match variant_tag {
            0 | 1 | 2 | 4 | 5 | 6 | 7 | 8 => unimplemented!("stub for other variants"),
            3 => {
                let action = TransferAction::deserialize_reader(reader)?;
                Ok(Self::Transfer(action))
            }
            _ => {
                return Err(Error::from(ErrorKind::InvalidData));
            }
        }
    }
}

impl Action {
    pub fn _type(&self) -> &'static str {
        match self {
            Action::Transfer(_transfer) => "Transfer",
            _ => {
                unimplemented!("stub for other variants");
            }
        }
    }


    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) {
        use ledger_device_sdk::testing;
        use numtoa::NumToA;

        testing::debug_print("debug printing action:\n");
        match self {
            Action::Transfer(transfer) => {
                testing::debug_print("debug printing amount:\n");

                let mut numtoa_buf = [0u8; 40];

                testing::debug_print(transfer.deposit.numtoa_str(10, &mut numtoa_buf));
                testing::debug_print("\n");
                let deposit = (transfer.deposit as f64) / (ONE_NEAR as f64);
                let mut buffer = dtoa::Buffer::new();
                let printed = buffer.format(deposit);
                testing::debug_print(printed);
                testing::debug_print("\n");
                testing::debug_print("debug printing amount finish:\n\n");
            }
            _ => {
                unimplemented!("stub for other variants");
            }
        }
        testing::debug_print("debug printing action finish:\n");
    }
}
