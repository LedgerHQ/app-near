use crate::utils::types::elipsis_fields::{ElipsisFields, EllipsisBuffer};
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_gas::GasBuffer;
use near_token::TokenBuffer;

use crate::app_ui::{aliases::CappedAccountId, fields_writer::FieldsWriter};

use super::StakeArgs;

pub struct FieldsContext {
    pub signer_display_buf: EllipsisBuffer,
    pub pool_display_buf: EllipsisBuffer,
    pub amount_buffer: TokenBuffer,
    pub deposit_buffer: TokenBuffer,
    pub args_display_buf: EllipsisBuffer,
    pub gas_buf: GasBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            signer_display_buf: EllipsisBuffer::default(),
            pool_display_buf: EllipsisBuffer::default(),
            amount_buffer: TokenBuffer::new(),
            deposit_buffer: TokenBuffer::new(),
            args_display_buf: EllipsisBuffer::default(),
            gas_buf: GasBuffer::new(),
        }
    }
}

/// Stake account (1-2) + amount or "all" (0-1), or Method Name (1) + Args (1-2) when the
/// args are shown as signed + Attached deposit (0-1) + Stake pool (1-2) + Gas (1)
pub const MAX_FIELDS: usize = 9;

/// Labels and amount display depend on the operation kind.
#[derive(Copy, Clone)]
pub enum StakeOpKind {
    /// deposit_and_stake: amount is the attached deposit
    DepositAndStake,
    /// unstake / withdraw: amount is from args JSON
    Unstake,
    /// unstake_all / withdraw_all: no specific amount
    UnstakeAll,
}

pub fn format<'b, 'a: 'b>(
    signer_id: &'b mut CappedAccountId,
    receiver_id: &'b mut CappedAccountId,
    method_name: &'b str,
    args: &'b mut StakeArgs,
    deposit: near_token::NearToken,
    show_deposit: bool,
    gas: near_gas::NearGas,
    kind: StakeOpKind,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    match kind {
        StakeOpKind::DepositAndStake => {
            #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
            let signer_title = "Stake with account";
            #[cfg(not(any(target_os = "stax", target_os = "flex", target_os = "apex_p")))]
            let signer_title = "Stake with acc";

            let signer_fields = ElipsisFields::from_capped_string(
                signer_id,
                signer_title,
                &mut field_context.signer_display_buf,
            );
            writer.push_fields(signer_fields);

            push_amount_or_args(
                "Stake amount",
                method_name,
                args,
                &mut field_context.amount_buffer,
                &mut field_context.args_display_buf,
                writer,
            );

            push_attached_deposit(
                deposit,
                show_deposit,
                &mut field_context.deposit_buffer,
                writer,
            );

            let pool_fields = ElipsisFields::from_capped_string(
                receiver_id,
                "Stake to pool",
                &mut field_context.pool_display_buf,
            );
            writer.push_fields(pool_fields);
        }
        StakeOpKind::Unstake => {
            let pool_fields = ElipsisFields::from_capped_string(
                receiver_id,
                "Unstake from pool",
                &mut field_context.pool_display_buf,
            );
            writer.push_fields(pool_fields);

            push_amount_or_args(
                "Unstake amount",
                method_name,
                args,
                &mut field_context.amount_buffer,
                &mut field_context.args_display_buf,
                writer,
            );

            push_attached_deposit(
                deposit,
                show_deposit,
                &mut field_context.deposit_buffer,
                writer,
            );

            let signer_fields = ElipsisFields::from_capped_string(
                signer_id,
                "Unstake to",
                &mut field_context.signer_display_buf,
            );
            writer.push_fields(signer_fields);
        }
        StakeOpKind::UnstakeAll => {
            let pool_fields = ElipsisFields::from_capped_string(
                receiver_id,
                "Unstake from pool",
                &mut field_context.pool_display_buf,
            );
            writer.push_fields(pool_fields);

            if let StakeArgs::Decoded(_) = args {
                writer.push_fields(ElipsisFields::one(Field {
                    name: "Unstake amount",
                    value: "All staked NEAR",
                }));
            } else {
                push_amount_or_args(
                    "Unstake amount",
                    method_name,
                    args,
                    &mut field_context.amount_buffer,
                    &mut field_context.args_display_buf,
                    writer,
                );
            }

            push_attached_deposit(
                deposit,
                show_deposit,
                &mut field_context.deposit_buffer,
                writer,
            );

            let signer_fields = ElipsisFields::from_capped_string(
                signer_id,
                "Unstake to",
                &mut field_context.signer_display_buf,
            );
            writer.push_fields(signer_fields);
        }
    }

    gas.display_as_buffer(&mut field_context.gas_buf);
    writer.push_fields(ElipsisFields::one(Field {
        name: "Gas",
        value: field_context.gas_buf.as_str(),
    }));
}

/// Pushes the decoded amount, or the method name and the args as signed when
/// they could not be decoded strictly.
fn push_amount_or_args<'b>(
    amount_title: &'b str,
    method_name: &'b str,
    args: &'b mut StakeArgs,
    amount_buffer: &'b mut TokenBuffer,
    args_display_buf: &'b mut EllipsisBuffer,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    let args_fields = match args {
        StakeArgs::Decoded(None) => return,
        StakeArgs::Decoded(Some(amount)) => {
            amount.display_as_buffer(amount_buffer);
            writer.push_fields(ElipsisFields::one(Field {
                name: amount_title,
                value: amount_buffer.as_str(),
            }));
            return;
        }
        StakeArgs::RawStr(args_str) => {
            ElipsisFields::from_capped_string(args_str, "Args String", args_display_buf)
        }
        StakeArgs::RawHex(args_bin) => {
            ElipsisFields::from_hex_display(args_bin, "Args Binary", args_display_buf)
        }
    };
    writer.push_fields(ElipsisFields::one(Field {
        name: "Method Name",
        value: method_name,
    }));
    writer.push_fields(args_fields);
}

/// Pushes the attached deposit as its own field when it is nonzero and not
/// already displayed as the operation amount.
fn push_attached_deposit<'b>(
    deposit: near_token::NearToken,
    show_deposit: bool,
    deposit_buffer: &'b mut TokenBuffer,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    if show_deposit && deposit.as_yoctonear() != 0 {
        deposit.display_as_buffer(deposit_buffer);
        writer.push_fields(ElipsisFields::one(Field {
            name: "Attached deposit",
            value: deposit_buffer.as_str(),
        }));
    }
}
