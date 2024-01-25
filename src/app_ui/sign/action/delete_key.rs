use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing::{self, types::tx_public_key::TxPublicKey},
    utils::types::{capped_string::ElipsisFields, fmt_buffer::FmtBuffer},
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
                let mut tmp_buf = [0u8; 50];
                // NOTE: expecting `tmp_buf` to be always large enough : 1.4 * 32
                let len = bs58::encode(arr).onto(&mut tmp_buf[..]).unwrap();
                // expecting `bs58` to always produce correct strings
                // https://docs.rs/bs58/0.5.0/src/bs58/encode.rs.html#201
                let bs58_str = core::str::from_utf8(&tmp_buf[..len]).unwrap();

                self.buffer.write_str("ed25519:");
                self.buffer.write_str(bs58_str);
            }
            TxPublicKey::SECP256K1(arr) => {
                let mut tmp_buf = [0u8; 90];

                // expecting `tmp_buf` to be always large enough: 1.4 * 64
                let len = bs58::encode(arr).onto(&mut tmp_buf[..]).unwrap();
                // expecting `bs58` to always produce correct strings
                // https://docs.rs/bs58/0.5.0/src/bs58/encode.rs.html#201
                let bs58_str = core::str::from_utf8(&tmp_buf[..len]).unwrap();

                self.buffer.write_str("secp256k1:");
                self.buffer.write_str(bs58_str);
            }
        }
    }
}

pub fn format<'b, 'a: 'b>(
    delete_key: &parsing::types::DeleteKey,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    field_context.format_public_key(&delete_key.public_key);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type:",
            value: "Delete Key",
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Public Key:",
            value: field_context.buffer.as_str(),
        }))
        .unwrap();
}
