use ledger_device_sdk::ecc::{CurvesId, ECPrivateKey, ECPublicKey, Ed25519, Secret};
#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;
use ledger_secure_sdk_sys::os_perso_derive_node_with_seed_key;

use crate::AppSW;

use crate::utils::types::fmt_buffer::FmtBuffer;

const PUBLIC_KEY_BIG_ENDIAN_LEN: usize = 32;
const PUBLIC_KEY_LITTLE_ENDIAN_LEN: usize = 65;

const HDW_ED25519_SLIP10: u32 = 1;
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

impl PublicKeyBe {
    /// converts little endian 65 byte (0x4 32X 32Y) public key to 32 byte Y big endian form (for other applications)
    pub fn from_little_endian(input: ECPublicKey<PUBLIC_KEY_LITTLE_ENDIAN_LEN, 'E'>) -> Self {
        let mut out = [0u8; PUBLIC_KEY_BIG_ENDIAN_LEN];

        // copy public key little endian to big endian
        for i in 0..PUBLIC_KEY_BIG_ENDIAN_LEN {
            out[i] = input.pubkey[PUBLIC_KEY_LITTLE_ENDIAN_LEN - 1 - i];
        }
        // set sign bit
        if (input.pubkey[PUBLIC_KEY_BIG_ENDIAN_LEN] & 1) != 0 {
            out[PUBLIC_KEY_BIG_ENDIAN_LEN - 1] |= 0x80;
        }

        PublicKeyBe(out)
    }

    pub fn display_str_base58(&self, buffer: &mut FmtBuffer<60>) -> Result<(), AppSW> {
        let mut out = [0u8; 50];
        let len = bs58::encode(&self.0)
            .onto(&mut out[..])
            .map_err(|_| AppSW::AddrDisplayFail)?;
        let bs58_str = core::str::from_utf8(&out[..len]).map_err(|_| AppSW::AddrDisplayFail)?;

        buffer.write_str("ed25519:");
        buffer.write_str(bs58_str);

        Ok(())
    }

    pub fn display_str_hex<'a, 'b>(&'a self, buffer: &'b mut [u8; 64]) -> &'b str {
        hex::encode_to_slice(&self.0, buffer).unwrap();

        core::str::from_utf8(buffer).unwrap()
    }

    #[cfg(feature = "speculos")]
    pub fn debug_print(&self) -> Result<(), AppSW> {
        testing::debug_print("debug printing pub key:\n");

        let mut out_buf = FmtBuffer::<60>::new();

        self.display_str_base58(&mut out_buf)?;

        testing::debug_print(out_buf.as_str());
        testing::debug_print("\n");
        testing::debug_print("debug printing pub key finish:\n\n");
        Ok(())
    }
}
