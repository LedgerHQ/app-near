use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter,
    parsing::{self, types::TxPublicKey},
    utils::types::{base58_buf::Base58Buf, elipsis_fields::ElipsisFields, fmt_buffer::FmtBuffer},
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

pub fn format<'b, 'a: 'b>(
    delete_key: &parsing::types::DeleteKey,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    field_context.format_public_key(&delete_key.public_key);
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Action type",
            value: "Delete Key",
        }))
        .unwrap();

    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Public Key",
            value: field_context.buffer.as_str(),
        }))
        .unwrap();
}
