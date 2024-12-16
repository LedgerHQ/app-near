use crate::utils::{
    crypto::{PathBip32, PublicKeyBe},
    types::base58_buf::Base58Buf,
};
use near_token::{NearToken, TokenBuffer};

use ledger_device_sdk::{
    ecc,
    io::Comm,
    libcall::{self, swap, swap::CreateTxParams},
    testing::debug_print,
};

use crate::parsing::transaction_stream_reader::SingleTxStream;

pub fn swap_main(arg0: u32) {
    debug_print("call app for swap \n");

    let cmd = libcall::get_command(arg0);

    match cmd {
        libcall::LibCallCommand::SwapCheckAddress => {
            let mut params = swap::get_check_address_params(arg0);

            let mut res = 0i32;
            match PathBip32::parse(&params.dpath[..params.dpath_len * 4]) {
                Ok(path) => match ecc::Ed25519::derive_from_path_slip10(&path.0).public_key() {
                    Ok(pk) => {
                        let pk = PublicKeyBe::from_little_endian(pk);
                        let mut bs58_buf: Base58Buf<50> = Base58Buf::new();
                        match bs58_buf.encode(&pk.0) {
                            Ok(_) => {
                                if bs58_buf.as_str().eq(core::str::from_utf8(
                                    &params.ref_address[..params.ref_address_len],
                                )
                                .unwrap())
                                {
                                    res = 1i32;
                                }
                            }
                            Err(_) => {
                                debug_print("PK base58 encoding failure\n");
                            }
                        }
                    }
                    Err(_) => {
                        debug_print("Public key derivation failure\n");
                    }
                },
                Err(_) => {
                    debug_print("Derivation path failure\n");
                }
            }
            swap::swap_return(swap::SwapResult::CheckAddressResult(&mut params, res));
        }
        libcall::LibCallCommand::SwapGetPrintableAmount => {
            let mut params = swap::get_printable_amount_params(arg0);

            let amount = u128::from_be_bytes(params.amount);
            let near_token = NearToken::from_yoctonear(amount);
            let mut near_token_buffer = TokenBuffer::new();
            near_token.display_as_buffer(&mut near_token_buffer);
            let s = near_token_buffer.as_str();

            swap::swap_return(swap::SwapResult::PrintableAmountResult(&mut params, s));
        }
        libcall::LibCallCommand::SwapSignTransaction => {
            let mut params = swap::sign_tx_params(arg0);

            {
                let mut comm = Comm::new().set_expected_cla(super::CLA);

                debug_print("Wait for APDU\n");

                // Wait for an APDU command
                let ins: super::Instruction = comm.next_command();

                debug_print("APDU received\n");

                match handle_apdu(&mut comm, ins, &params) {
                    Ok(sig) => {
                        debug_print("send back signature APDU\n");
                        comm.append(&sig);
                        comm.swap_reply_ok();
                        swap::swap_return(swap::SwapResult::CreateTxResult(&mut params, 1));
                    }
                    Err(sw) => {
                        comm.swap_reply(sw);
                        swap::swap_return(swap::SwapResult::CreateTxResult(&mut params, 0));
                    }
                }
            }
        }
    }
}

fn handle_apdu(
    comm: &mut Comm,
    ins: super::Instruction,
    tx_params: &CreateTxParams,
) -> Result<[u8; 64], crate::AppSW> {
    match ins {
        super::Instruction::SignTx {
            is_last_chunk,
            sign_mode,
        } => {
            debug_print("handle_swap_apdu => Sign Tx\n");
            let stream = SingleTxStream::new(comm, is_last_chunk, sign_mode);
            match sign_mode {
                super::SignMode::Transaction => {
                    let signature = crate::handlers::sign_tx::handler_swap(stream, tx_params);
                    match signature {
                        Ok(sig) => Ok(sig.0),
                        Err(sw) => Err(sw),
                    }
                }
                _ => Err(crate::AppSW::TxSignFail),
            }
        }
        _ => Err(crate::AppSW::InsNotSupported),
    }
}
