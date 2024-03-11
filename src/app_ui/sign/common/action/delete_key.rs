use ledger_device_sdk::ui::gadgets::Field;

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing, sign_ui::common::tx_public_key_context,
    utils::types::elipsis_fields::ElipsisFields,
};

pub fn format<'b, 'a: 'b>(
    delete_key: &parsing::types::DeleteKey,
    field_context: &'a mut tx_public_key_context::FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 2>,
) {
    field_context.format_public_key(&delete_key.public_key);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Action type",
        value: "Delete Key",
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "Public Key",
        value: field_context.buffer.as_str(),
    }));
}
