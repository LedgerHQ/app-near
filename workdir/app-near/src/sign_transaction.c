#include "sign_transaction.h"
#include "parse_transaction.h"
#include "os.h"
#include "ux.h"
#include "utils.h"
#include "main.h"

//////////////////////////////////////////////////////////////////////

#define INFO_STEP(name, info_title, info_text) \
UX_STEP_NOCB( \
    name, \
    bnnn_paging, \
    { \
        .title = info_title, \
        .text = info_text, \
    })

INFO_STEP(sign_flow_intro_step, "Confirm",  ui_context.line1);
INFO_STEP(sign_flow_receiver_step, "To",  ui_context.line2);
INFO_STEP(sign_flow_signer_step, "From",  ui_context.line3);
INFO_STEP(sign_flow_amount_step, "Amount",  ui_context.amount);
INFO_STEP(sign_flow_deposit_step, "Deposit",  ui_context.line5);
INFO_STEP(sign_flow_args_step, "Args",  ui_context.long_line);
INFO_STEP(sign_flow_to_account_step, "To Account",  ui_context.line3);
INFO_STEP(sign_flow_contract_step, "Contract",  ui_context.line2);
INFO_STEP(sign_flow_allowance_step, "Allowance",  ui_context.line5);
INFO_STEP(sign_flow_danger_step, "DANGER", "This gives full access to a device other than Ledger");

UX_STEP_VALID(
    sign_flow_approve_step,
    pb,
    send_response(set_result_sign(), true),
    {
        &C_icon_validate_14,
        "Approve",
    });

UX_STEP_VALID(
    sign_flow_reject_step,
    pb,
    send_response(0, false),
    {
        &C_icon_crossmark,
        "Reject",
    });

UX_FLOW(
    ux_display_sign_flow,
    &sign_flow_intro_step,
    &sign_flow_receiver_step,
    &sign_flow_signer_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

UX_FLOW(
    ux_display_sign_transfer_flow,
    &sign_flow_intro_step,
    &sign_flow_amount_step,
    &sign_flow_receiver_step,
    &sign_flow_signer_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

UX_FLOW(
    ux_display_sign_function_call_flow,
    &sign_flow_intro_step,
    &sign_flow_deposit_step,
    &sign_flow_receiver_step,
    &sign_flow_signer_step,
    &sign_flow_args_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

UX_FLOW(
    ux_display_sign_add_function_call_key_flow,
    &sign_flow_intro_step,
    &sign_flow_to_account_step,
    &sign_flow_contract_step,
    &sign_flow_allowance_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

UX_FLOW(
    ux_display_sign_add_full_access_key_flow,
    &sign_flow_intro_step,
    &sign_flow_danger_step,
    &sign_flow_contract_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

void print_ui_context() {
    for (int i = 0; i < 6; i++) {
        PRINTF("line %d: %s\n", i, &ui_context.line1[sizeof(ui_context.line1) * i]);
    }
}

void sign_ux_flow_init() {
    PRINTF("sign_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_flow, NULL);
}

void sign_transfer_ux_flow_init() {
    PRINTF("sign_transfer_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_transfer_flow, NULL);
}

void sign_function_call_ux_flow_init() {
    PRINTF("sign_function_call_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_function_call_flow, NULL);
}

void sign_add_function_call_key_ux_flow_init() {
    PRINTF("sign_add_function_call_key_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_add_function_call_key_flow, NULL);
}

static void add_chunk_data(const uint8_t *input_data, size_t input_length) {
    // if this is a first chunk
    if (tmp_ctx.signing_context.buffer_used == 0) {
        // then there is the bip32 path in the first chunk - first 20 bytes of data
        size_t path_size = sizeof(tmp_ctx.signing_context.bip32);
        if (input_length < path_size) {
            // TODO: Have specific error for underflow?
            THROW(SW_BUFFER_OVERFLOW);
        }
        read_path_from_bytes(input_data, tmp_ctx.signing_context.bip32);

        input_length -= path_size;
        PRINTF("data_size: %d\n", data_size);

        memcpy(tmp_ctx.signing_context.buffer, &input_data[path_size], input_length);
        PRINTF("buffer: %.*h\n", input_length, tmp_ctx.signing_context.buffer);
    } else {
        // else update the data from entire segment.
        PRINTF("data_size: %d\n", input_length);
        if (tmp_ctx.signing_context.buffer_used + input_length > MAX_DATA_SIZE) {
            THROW(SW_BUFFER_OVERFLOW);
        }
        memcpy(&tmp_ctx.signing_context.buffer[tmp_ctx.signing_context.buffer_used], input_data, input_length);
        PRINTF("buffer: %.*h\n", input_length, &tmp_ctx.signing_context.buffer[tmp_ctx.signing_context.buffer_used]);
    }
    tmp_ctx.signing_context.buffer_used += input_length;
}

void handle_sign_transaction(uint8_t p1, uint8_t p2, const uint8_t *input_buffer, uint16_t input_length, volatile unsigned int *flags, volatile unsigned int *tx) {
    UNUSED(p2);
    UNUSED(tx);

    if (p1 != P1_MORE && p1 != P1_LAST) {
        THROW(SW_INCORRECT_P1_P2);
    }

    if (p1 == P1_LAST) {
        // TODO: Is network_byte used anywhere?
        tmp_ctx.signing_context.network_byte = p2;
        add_chunk_data(input_buffer, input_length);

        switch (parse_transaction()) {
            case SIGN_FLOW_GENERIC:
                sign_ux_flow_init();
                break;
            case SIGN_FLOW_TRANSFER:
                sign_transfer_ux_flow_init();
                break;
            case SIGN_FLOW_FUNCTION_CALL:
                sign_function_call_ux_flow_init();
                break;
            case SIGN_FLOW_ADD_FUNCTION_CALL_KEY:
                sign_add_function_call_key_ux_flow_init();
                break;
            case SIGN_FLOW_ADD_FULL_ACCESS_KEY:
                sign_add_function_call_key_ux_flow_init();
                break;
            case SIGN_PARSING_ERROR:
                THROW(SW_BUFFER_OVERFLOW);
            default:
                THROW(SW_CONDITIONS_NOT_SATISFIED);
        }
    } else {
        add_chunk_data(input_buffer, input_length);
        THROW(SW_OK);
    }

    *flags |= IO_ASYNCH_REPLY;
}
