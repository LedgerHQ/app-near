use crate::{
    parsing::{self},
    sign_ui::common::tx_public_key_context,
    utils::types::review_field::ReviewField,
};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;
use near_token::TokenBuffer;

use crate::app_ui::{aliases::CappedAccountId, fields_writer::FieldsWriter};

pub struct FieldsContext {
    pub stake_buffer: TokenBuffer,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            stake_buffer: TokenBuffer::new(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
        }
    }
}
/// action type (1) + Stake (1) + Public Key (1)
const MAX_FIELDS: usize = 3;

pub fn format<'b, 'a: 'b>(
    stake: &parsing::types::Stake,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    field_context
        .pub_key_context
        .format_public_key(&stake.public_key);
    writer.push_field(ReviewField::new(Field {
        name: "Action type",
        value: "Stake",
    }));

    stake
        .stake
        .display_as_buffer(&mut field_context.stake_buffer);
    writer.push_field(ReviewField::new(Field {
        name: "Stake",
        value: field_context.stake_buffer.as_str(),
    }));

    writer.push_field(ReviewField::new(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));
}

/// Combined format: signer account + stake amount + validator key.
/// Used by the merged prefix+action review for native Stake transactions.
pub struct CombinedFieldsContext {
    pub stake_buffer: TokenBuffer,
    pub pub_key_context: tx_public_key_context::FieldsContext,
}

impl CombinedFieldsContext {
    pub fn new() -> Self {
        Self {
            stake_buffer: TokenBuffer::new(),
            pub_key_context: tx_public_key_context::FieldsContext::new(),
        }
    }
}

/// Stake account (1) + Stake amount (1) + Stake to pool (1) + Public Key (1)
pub const COMBINED_MAX_FIELDS: usize = 4;

pub fn format_combined<'b, 'a: 'b>(
    signer_id: &'b mut CappedAccountId,
    receiver_id: &'b mut CappedAccountId,
    stake: &parsing::types::Stake,
    field_context: &'a mut CombinedFieldsContext,
    writer: &'_ mut FieldsWriter<'b, COMBINED_MAX_FIELDS>,
) {
    field_context
        .pub_key_context
        .format_public_key(&stake.public_key);

    #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
    let title = "Stake with account";
    #[cfg(not(any(target_os = "stax", target_os = "flex", target_os = "apex_p")))]
    let title = "Stake with acc";

    let signer_fields = ReviewField::from_capped_string(signer_id, title);
    writer.push_field(signer_fields);

    stake
        .stake
        .display_as_buffer(&mut field_context.stake_buffer);
    writer.push_field(ReviewField::new(Field {
        name: "Stake amount",
        value: field_context.stake_buffer.as_str(),
    }));

    let pool_fields = ReviewField::from_capped_string(receiver_id, "Stake to pool");
    writer.push_field(pool_fields);

    writer.push_field(ReviewField::new(Field {
        name: "Public Key",
        value: field_context.pub_key_context.as_str(),
    }));
}
