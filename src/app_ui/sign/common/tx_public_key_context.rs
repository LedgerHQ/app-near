use crate::{
    parsing::types::TxPublicKey,
    utils::types::{base58_buf::Base58Buf, fmt_buffer::FmtBuffer},
};

pub struct FieldsContext {
    pub buffer: FmtBuffer<100>,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            buffer: FmtBuffer::new(),
        }
    }

    pub fn format_public_key(&mut self, public_key: &TxPublicKey) {
        match public_key {
            TxPublicKey::ED25519(arr) => {
                let mut bs58_buf: Base58Buf<50> = Base58Buf::new();
                // NOTE: expecting `tmp_buf` to be always large enough : 1.4 * 32
                bs58_buf.encode(arr).unwrap();

                self.buffer.write_str("ed25519:");
                self.buffer.write_str(bs58_buf.as_str());
            }
            TxPublicKey::SECP256K1(arr) => {
                let mut bs58_buf: Base58Buf<90> = Base58Buf::new();
                // expecting `tmp_buf` to be always large enough: 1.4 * 64
                bs58_buf.encode(arr).unwrap();

                self.buffer.write_str("secp256k1:");
                self.buffer.write_str(bs58_buf.as_str());
            }
        }
    }
}
