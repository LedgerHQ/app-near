use crate::{parsing::types::TxPublicKey, utils::types::base58_buf::Base58Buf};
use fmt_buffer::Buffer;

pub struct FieldsContext {
    /// large enough buffer to fit `key_type:` prefix and key base 58 representation
    buffer: Buffer<100>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    pub fn format_public_key(&mut self, public_key: &TxPublicKey) {
        match public_key {
            TxPublicKey::ED25519(arr) => {
                let mut bs58_buf: Base58Buf<50> = Base58Buf::new();
                // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
                // is not expected to be encountered on encoding 32 bytes to 50 bytes long buffer
                bs58_buf.encode(arr).unwrap();

                self.buffer.write_str("ed25519:");
                self.buffer.write_str(bs58_buf.as_str());
            }
            TxPublicKey::SECP256K1(arr) => {
                let mut bs58_buf: Base58Buf<90> = Base58Buf::new();
                // expecting `tmp_buf` to be always large enough: 1.4 * 64
                // .unwrap() is ok, as [`bs58::encode::Error::BufferTooSmall`](https://docs.rs/bs58/0.5.0/bs58/encode/enum.Error.html)
                // is not expected to be encountered on encoding 64 bytes to 90 bytes long buffer
                bs58_buf.encode(arr).unwrap();

                self.buffer.write_str("secp256k1:");
                self.buffer.write_str(bs58_buf.as_str());
            }
        }
    }

    pub fn as_str(&mut self) -> &str {
        self.buffer.as_str()
    }
}
