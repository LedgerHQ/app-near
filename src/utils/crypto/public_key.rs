use ledger_device_sdk::ecc::ECPublicKey;

use crate::AppSW;

use crate::parsing::types::TxPublicKey;
use crate::utils::types::base58_buf::Base58Buf;
use fmt_buffer::Buffer;

const PUBLIC_KEY_BIG_ENDIAN_LEN: usize = 32;
const PUBLIC_KEY_LITTLE_ENDIAN_LEN: usize = 65;

#[derive(PartialEq, Eq)]
pub struct PublicKeyBe(pub [u8; PUBLIC_KEY_BIG_ENDIAN_LEN]);

pub struct NoSecpAllowed;

impl TryFrom<TxPublicKey> for PublicKeyBe {
    type Error = NoSecpAllowed;
    fn try_from(value: TxPublicKey) -> Result<Self, Self::Error> {
        match value {
            TxPublicKey::SECP256K1(_arr) => Err(NoSecpAllowed),
            TxPublicKey::ED25519(arr) => Ok(Self(arr)),
        }
    }
}

impl PublicKeyBe {
    /// converts little endian 65 byte (0x4 32X 32Y) public key to 32 byte Y big endian form (for other applications)
    pub fn from_little_endian(input: ECPublicKey<PUBLIC_KEY_LITTLE_ENDIAN_LEN, 'E'>) -> Self {
        let mut out = [0u8; PUBLIC_KEY_BIG_ENDIAN_LEN];

        // copy public key little endian to big endian

        for (i, out_byte) in out.iter_mut().enumerate().take(PUBLIC_KEY_BIG_ENDIAN_LEN) {
            *out_byte = input.pubkey[PUBLIC_KEY_LITTLE_ENDIAN_LEN - 1 - i];
        }
        // set sign bit
        if (input.pubkey[PUBLIC_KEY_BIG_ENDIAN_LEN] & 1) != 0 {
            out[PUBLIC_KEY_BIG_ENDIAN_LEN - 1] |= 0x80;
        }

        PublicKeyBe(out)
    }

    pub fn display_str_base58(&self, buffer: &mut Buffer<60>) -> Result<(), AppSW> {
        let mut bs58_buf: Base58Buf<50> = Base58Buf::new();
        bs58_buf
            .encode(&self.0)
            .map_err(|_| AppSW::AddrDisplayFail)?;

        buffer.write_str("ed25519:");
        buffer.write_str(bs58_buf.as_str());

        Ok(())
    }

    pub fn display_str_hex<'b>(
        &self,
        buffer: &'b mut [u8; PUBLIC_KEY_BIG_ENDIAN_LEN * 2],
    ) -> &'b str {
        // .unwrap() is ok, as `64 == 32 * 2` holds true
        hex::encode_to_slice(self.0, buffer).unwrap();

        // .unwrap() is ok, as buffer contains only bytes, encoding hex chars
        core::str::from_utf8(buffer).unwrap()
    }
}
