use crate::{
    app_ui::{aliases::U32Buffer, fields_writer::FieldsWriter},
    parsing,
    utils::types::elipsis_fields::ElipsisFields,
};
use fmt_buffer::Buffer;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use numtoa::NumToA;

/// A buffer, large enough to fit all bytes representations from u64
pub type U64BytesBuffer = Buffer<30>;

pub struct V1FieldsContext {
    pub data_size_buf: U64BytesBuffer,
    pub data_entries_buf: U32Buffer,
}

impl V1FieldsContext {
    pub fn new() -> Self {
        Self {
            data_size_buf: U64BytesBuffer::new(),
            data_entries_buf: U32Buffer::default(),
        }
    }
}

pub fn format_v1<'b, 'a: 'b, const N: usize>(
    state_init_v1: &'a mut parsing::types::DeterministicAccountStateInitV1,
    field_context: &'a mut V1FieldsContext,
    writer: &'_ mut FieldsWriter<'b, N>,
) {
    let (display_name, display_val) = match &mut state_init_v1.code {
        parsing::types::GlobalContractIdentifier::CodeHash(code_hash) => {
            ("Contract SHA256", code_hash.as_str())
        }
        parsing::types::GlobalContractIdentifier::AccountId(account_id) => {
            ("Contract Account", account_id.as_str())
        }
    };

    writer.push_fields(ElipsisFields::one(Field {
        name: display_name,
        value: display_val,
    }));

    bytes_to_str(
        state_init_v1.data_size_bytes,
        &mut field_context.data_size_buf,
    );

    writer.push_fields(ElipsisFields::one(Field {
        name: "State Size",
        value: field_context.data_size_buf.as_str(),
    }));

    writer.push_fields(ElipsisFields::one(Field {
        name: "State Total Entries",
        value: state_init_v1
            .data_entries
            // numtoa_buf has to be at least 10 bytes for u32 (4 bytes) : ok
            .numtoa_str(10, &mut field_context.data_entries_buf),
    }));
}

/// Converts bytes represented in u64 to a TokenBuffer for displaying in Ledger
fn bytes_to_str(input: u64, result: &mut U64BytesBuffer) {
    let disp_str: &str;
    let div_by: u64;
    let expected_len: u64;

    if input < 10u64.pow(3) {
        disp_str = "bytes";
        div_by = 1;
        expected_len = 0;
    } else if input < 10u64.pow(6) {
        disp_str = "kBytes";
        div_by = 10u64.pow(3);
        expected_len = 3;
    } else if input < 10u64.pow(9) {
        disp_str = "mBytes";
        div_by = 10u64.pow(6);
        expected_len = 6;
    } else {
        disp_str = "gBytes";
        div_by = 10u64.pow(9);
        expected_len = 9;
    }

    let bef_dot = input / div_by;
    let aft_dot = input % div_by;

    let mut str_buf = [0u8; 20];

    if div_by == 1 {
        // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
        result.write_str(bef_dot.numtoa_str(10, &mut str_buf));
        result.write_str(" bytes");
    } else {
        // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
        result.write_str(bef_dot.numtoa_str(10, &mut str_buf));
        result.write_str(".");

        let mut aft_str_buf = [0u8; 20];
        // numtoa_buf has to be at least 20 bytes for u64 (8 bytes) : ok
        let aft_str = aft_dot.numtoa_str(10, &mut aft_str_buf);

        let leading_zeros = expected_len - aft_str.len() as u64;
        for _ in 0..leading_zeros {
            result.write_str("0");
        }

        // Remove trailing zeros from aft_str before writing to buffer
        let trimmed_aft_str = aft_str.trim_end_matches('0');
        if trimmed_aft_str.is_empty() && leading_zeros == 0 {
            result.write_str("0");
        } else {
            result.write_str(trimmed_aft_str);
        }

        result.write_str(" ");
        result.write_str(disp_str);
    }
}
