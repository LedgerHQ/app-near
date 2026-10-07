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

/// Signer account (1-2) + Method Name (1) + amount or "all" (1), or Args (1-2) when the
/// args are shown as signed + Attached deposit (0-1) + Pool (1-2) + Gas (1).
/// The worst case (raw args, long signer and pool ids, nonzero deposit) uses all 9.
pub const MAX_FIELDS: usize = 9;

/// Labels and amount display depend on the operation kind.
#[derive(Copy, Clone)]
pub enum StakeOpKind {
    /// deposit_and_stake / stake
    DepositAndStake,
    /// unstake: amount is from args JSON
    Unstake,
    /// unstake_all: no specific amount
    UnstakeAll,
    /// withdraw: amount is from args JSON
    Withdraw,
    /// withdraw_all: no specific amount
    WithdrawAll,
}

/// Labels of the operations that move funds out of a pool's staked or unstaked balance.
struct ReleaseLabels {
    pool: &'static str,
    amount: &'static str,
    /// Value shown as the amount by the `_all` methods.
    all: Option<&'static str>,
    to: &'static str,
}

impl StakeOpKind {
    fn release_labels(self) -> Option<ReleaseLabels> {
        let (pool, amount, to) = match self {
            StakeOpKind::DepositAndStake => return None,
            StakeOpKind::Unstake | StakeOpKind::UnstakeAll => {
                ("Unstake from pool", "Unstake amount", "Unstake to")
            }
            StakeOpKind::Withdraw | StakeOpKind::WithdrawAll => {
                ("Withdraw from pool", "Withdraw amount", "Withdraw to")
            }
        };
        let all = match self {
            StakeOpKind::UnstakeAll => Some("All staked NEAR"),
            StakeOpKind::WithdrawAll => Some("All unstaked NEAR"),
            _ => None,
        };
        Some(ReleaseLabels {
            pool,
            amount,
            all,
            to,
        })
    }
}

#[allow(clippy::too_many_arguments)]
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
    match kind.release_labels() {
        None => {
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

            push_method_and_amount(
                "Stake amount",
                None,
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
        Some(labels) => {
            let pool_fields = ElipsisFields::from_capped_string(
                receiver_id,
                labels.pool,
                &mut field_context.pool_display_buf,
            );
            writer.push_fields(pool_fields);

            push_method_and_amount(
                labels.amount,
                labels.all,
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
                labels.to,
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

/// Pushes the method name, then the decoded amount (or `all_value` for the
/// `_all` methods), or the args as signed when they could not be decoded strictly.
#[allow(clippy::too_many_arguments)]
fn push_method_and_amount<'b>(
    amount_title: &'b str,
    all_value: Option<&'static str>,
    method_name: &'b str,
    args: &'b mut StakeArgs,
    amount_buffer: &'b mut TokenBuffer,
    args_display_buf: &'b mut EllipsisBuffer,
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_fields(ElipsisFields::one(Field {
        name: "Method Name",
        value: method_name,
    }));
    let fields = match args {
        StakeArgs::Decoded(None) => match all_value {
            Some(value) => ElipsisFields::one(Field {
                name: amount_title,
                value,
            }),
            None => return,
        },
        StakeArgs::Decoded(Some(amount)) => {
            amount.display_as_buffer(amount_buffer);
            ElipsisFields::one(Field {
                name: amount_title,
                value: amount_buffer.as_str(),
            })
        }
        StakeArgs::RawStr(args_str) => {
            ElipsisFields::from_capped_string(args_str, "Args String", args_display_buf)
        }
        StakeArgs::RawHex(args_bin) => {
            ElipsisFields::from_hex_display(args_bin, "Args Binary", args_display_buf)
        }
    };
    writer.push_fields(fields);
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
