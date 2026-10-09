use crate::utils::types::review_field::ReviewField;
#[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
use ledger_device_sdk::nbgl::Field;
#[cfg(any(target_os = "nanox", target_os = "nanosplus"))]
use ledger_device_sdk::ui::gadgets::Field;

use near_gas::GasBuffer;
use near_token::TokenBuffer;

use crate::app_ui::{aliases::CappedAccountId, fields_writer::FieldsWriter};

use super::StakeArgs;

pub struct FieldsContext {
    pub amount_buffer: TokenBuffer,
    pub deposit_buffer: TokenBuffer,
    pub gas_buf: GasBuffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            amount_buffer: TokenBuffer::new(),
            deposit_buffer: TokenBuffer::new(),
            gas_buf: GasBuffer::new(),
        }
    }
}

/// Signer account (1) + Method Name (1) + amount or "all" (1), or Args (1) when the
/// args are shown as signed + Attached deposit (0-1) + Pool (1) + Gas (1).
/// The worst case (nonzero deposit) uses all 6.
pub const MAX_FIELDS: usize = 6;

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

            let signer_fields = ReviewField::from_capped_string(signer_id, signer_title);
            writer.push_field(signer_fields);

            push_method_and_amount(
                "Stake amount",
                None,
                method_name,
                args,
                &mut field_context.amount_buffer,
                writer,
            );

            push_attached_deposit(
                deposit,
                show_deposit,
                &mut field_context.deposit_buffer,
                writer,
            );

            let pool_fields = ReviewField::from_capped_string(receiver_id, "Stake to pool");
            writer.push_field(pool_fields);
        }
        Some(labels) => {
            let pool_fields = ReviewField::from_capped_string(receiver_id, labels.pool);
            writer.push_field(pool_fields);

            push_method_and_amount(
                labels.amount,
                labels.all,
                method_name,
                args,
                &mut field_context.amount_buffer,
                writer,
            );

            push_attached_deposit(
                deposit,
                show_deposit,
                &mut field_context.deposit_buffer,
                writer,
            );

            let signer_fields = ReviewField::from_capped_string(signer_id, labels.to);
            writer.push_field(signer_fields);
        }
    }

    gas.display_as_buffer(&mut field_context.gas_buf);
    writer.push_field(ReviewField::new(Field {
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
    writer: &'_ mut FieldsWriter<'b, MAX_FIELDS>,
) {
    writer.push_field(ReviewField::new(Field {
        name: "Method Name",
        value: method_name,
    }));
    let fields = match args {
        StakeArgs::Decoded(None) => match all_value {
            Some(value) => ReviewField::new(Field {
                name: amount_title,
                value,
            }),
            None => return,
        },
        StakeArgs::Decoded(Some(amount)) => {
            amount.display_as_buffer(amount_buffer);
            ReviewField::new(Field {
                name: amount_title,
                value: amount_buffer.as_str(),
            })
        }
        StakeArgs::RawStr(args_str) => ReviewField::from_capped_string(args_str, "Args String"),
        StakeArgs::RawHex(args_bin) => ReviewField::from_hex_display(args_bin, "Args Binary"),
    };
    writer.push_field(fields);
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
        writer.push_field(ReviewField::new(Field {
            name: "Attached deposit",
            value: deposit_buffer.as_str(),
        }));
    }
}
