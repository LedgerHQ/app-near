use ledger_device_sdk::ecc::{CurvesId, ECPrivateKey, ECPublicKey, Ed25519, Secret};
use ledger_secure_sdk_sys::os_perso_derive_node_with_seed_key;

use crate::AppSW;

use crate::parsing::types::TxPublicKey;
use crate::utils::types::base58_buf::Base58Buf;
use fmt_buffer::Buffer;

const PUBLIC_KEY_BIG_ENDIAN_LEN: usize = 32;
const PUBLIC_KEY_LITTLE_ENDIAN_LEN: usize = 65;

const HDW_ED25519_SLIP10: u32 = 1;

#[derive(PartialEq, Eq)]
pub struct PublicKeyBe(pub [u8; PUBLIC_KEY_BIG_ENDIAN_LEN]);

pub fn bip32_derive(path: &[u32]) -> ECPrivateKey<32, 'E'> {
    let mut tmp = Secret::<32>::new();
    let curve = CurvesId::Ed25519;

    unsafe {
        os_perso_derive_node_with_seed_key(
            HDW_ED25519_SLIP10,
            curve as u8,
            path.as_ptr(),
            path.len() as u32,
            tmp.as_mut().as_mut_ptr(),
            core::ptr::null_mut(), // chain
            core::ptr::null_mut(), // seed_key
            0u32,                  // seed_key_length
        )
    };

    Ed25519::from(tmp.as_ref())
}

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

    pub fn display_str_hex<'b>(&self, buffer: &'b mut [u8; 64]) -> &'b str {
        hex::encode_to_slice(self.0, buffer).unwrap();

        core::str::from_utf8(buffer).unwrap()
    }
}
