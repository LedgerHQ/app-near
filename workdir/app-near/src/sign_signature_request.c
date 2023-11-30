#include "sign_signature_request.h"
#include "parse_signature_request.h"
#include "os.h"
#include "ux.h"
#include "utils.h"
#include "main.h"

//////////////////////////////////////////////////////////////////////

#ifdef HAVE_BAGL

#define INFO_STEP(name, info_title, info_text) \
    UX_STEP_NOCB(                              \
        name,                                  \
        bnnn_paging,                           \
        {                                      \
            .title = info_title,               \
            .text = info_text,                 \
        })

INFO_STEP(sign_flow_intro_step, "Confirm", ui_context.line1);
INFO_STEP(sign_flow_receiver_step, "To", ui_context.line2);
INFO_STEP(sign_flow_signer_step, "From", ui_context.line3);
INFO_STEP(sign_flow_amount_step, "Amount (NEAR)", ui_context.amount);
INFO_STEP(sign_flow_deposit_step, "Deposit", ui_context.line5);
INFO_STEP(sign_flow_args_step, "Args", ui_context.long_line);
INFO_STEP(sign_flow_to_account_step, "To Account", ui_context.line3);
INFO_STEP(sign_flow_contract_step, "Contract", ui_context.line2);
INFO_STEP(sign_flow_allowance_step, "Allowance", ui_context.line5);
INFO_STEP(sign_flow_callback_url_step, "Callback URL", ui_context.line3);
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

UX_FLOW(
    ux_display_sign_nep_413,
    &sign_flow_intro_step,
    &sign_flow_receiver_step,
    &sign_flow_callback_url_step,
    &sign_flow_approve_step,
    &sign_flow_reject_step);

void print_ui_context()
{
    for (int i = 0; i < 6; i++)
    {
        PRINTF("line %d: %s\n", i, &ui_context.line1[sizeof(ui_context.line1) * i]);
    }
}

void sign_ux_flow_init()
{
    PRINTF("sign_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_flow, NULL);
}

void sign_transfer_ux_flow_init()
{
    PRINTF("sign_transfer_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_transfer_flow, NULL);
}

void sign_function_call_ux_flow_init()
{
    PRINTF("sign_function_call_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_function_call_flow, NULL);
}

void sign_add_function_call_key_ux_flow_init()
{
    PRINTF("sign_add_function_call_key_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_add_function_call_key_flow, NULL);
}

void sign_message_nep_413_ux_flow_init()
{
    PRINTF("sign_message_nep_413_ux_flow_init\n");
    print_ui_context();
    ux_flow_init(0, ux_display_sign_nep_413, NULL);
}

#endif

#ifdef HAVE_NBGL

//  ----------------------------------------------------------- 
//  ---------------- SIGN TRANSACTION FLOWS -------------------
//  ----------------------------------------------------------- 

#include "nbgl_use_case.h"
#include "menu.h"

#define MAX_TAG_VALUE_PAIRS_DISPLAYED (5)
static nbgl_layoutTagValueList_t list  = {0};
static nbgl_layoutTagValue_t pairs[MAX_TAG_VALUE_PAIRS_DISPLAYED];
static nbgl_pageInfoLongPress_t long_press_infos;

static void approve_callback(void)
{
    send_response(set_result_sign(), true);
    ui_idle();
}

static void reject_callback(void)
{
    send_response(0, false);
    nbgl_useCaseStatus("Transaction rejected", false, ui_idle);
}

static void reject_confirmation(void) 
{
    nbgl_useCaseConfirm("Reject transaction?", NULL, "Yes, Reject", "Go back to transaction", reject_callback);
}

static void choice_callback(bool confirm)
{
    if (confirm)
    {
        nbgl_useCaseStatus("TRANSACTION\nSIGNED", true, approve_callback);
    }
    else
    {
        reject_confirmation();
    }
}


// Available field to be displayed

#define INTRO_ITEM "Review transaction to\n"
#define INTRO_VALUE ui_context.line1
#define RECEIVER_ITEM "To"
#define RECEIVER_VALUE ui_context.line2
#define SIGNER_ITEM "From"
#define SIGNER_VALUE ui_context.line3
#define AMOUNT_ITEM "Amount (NEAR)"
#define AMOUNT_VALUE ui_context.amount
#define DEPOSIT_ITEM "Deposit"
#define DEPOSIT_VALUE ui_context.line5
#define ARGS_ITEM "Args"
#define ARGS_VALUE ui_context.long_line
#define TO_ACCOUNT_ITEM "To account"
#define TO_ACCOUNT_VALUE ui_context.line3
#define CONTRACT_ITEM "Contract"
#define CONTRACT_VALUE ui_context.line2
#define ALLOWANCE_ITEM "Allowance"
#define ALLOWANCE_VALUE ui_context.line5
#define SIGN_ITEM "Sign transaction to\n"
#define SIGN_VALUE INTRO_VALUE
#define MESSAGE_ITEM "Review message\n"
#define MESSAGE_VALUE INTRO_VALUE
#define MAX_DISPLAYED_STRING_LENGTH 100

static char review_displayed_string[MAX_DISPLAYED_STRING_LENGTH] = {0};
static char sign_displayed_string[MAX_DISPLAYED_STRING_LENGTH] = {0};

// Utility macros

static uint8_t field_cnt = 0;

#define START_ADD_FIELD() \
    field_cnt = 0;

#define END_ADD_FIELD() \
    list.nbPairs = field_cnt;

#define ADD_FIELD(field)                  \
    pairs[field_cnt].item = field##_ITEM; \
    pairs[field_cnt++].value = field##_VALUE;

#define START_REVIEW() \
    nbgl_useCaseStaticReview(&list, &long_press_infos, "Reject transaction", choice_callback);

// Generics

static void generic_init_list(void)
{
    list.nbMaxLinesForValue = 0;
    list.pairs = pairs;
}

static void generic_init_hold_to_approve(void)
{  
    long_press_infos.icon = &C_stax_app_near_64px;
    long_press_infos.longPressText = "Hold to sign";
    memcpy(sign_displayed_string, SIGN_ITEM, sizeof(SIGN_ITEM));
    strlcat(sign_displayed_string, SIGN_VALUE, MAX_DISPLAYED_STRING_LENGTH);
    strlcat(sign_displayed_string, "?", MAX_DISPLAYED_STRING_LENGTH);
    long_press_infos.text = sign_displayed_string;
}

static void generic_intro_flow(nbgl_callback_t continue_callback)
{
    memcpy(review_displayed_string, INTRO_ITEM, sizeof(INTRO_ITEM));
    strlcat(review_displayed_string, INTRO_VALUE, MAX_DISPLAYED_STRING_LENGTH);

    generic_init_list();
    generic_init_hold_to_approve();

    nbgl_useCaseReviewStart(
        &C_stax_app_near_64px,
        review_displayed_string,
        NULL,
        "Reject transaction",
        continue_callback,
        reject_confirmation);
}

static void nep_413_intro_flow(nbgl_callback_t continue_callback)
{
    memcpy(review_displayed_string, MESSAGE_ITEM, sizeof(MESSAGE_ITEM));
    strlcat(review_displayed_string, MESSAGE_VALUE, MAX_DISPLAYED_STRING_LENGTH);

    generic_init_list();
    generic_init_hold_to_approve();

    nbgl_useCaseReviewStart(
        &C_stax_app_near_64px,
        review_displayed_string,
        NULL,
        "Reject",
        continue_callback,
        reject_confirmation);
}

// Sign
static void display_sign_flow(void)
{
    // Fill fields
    START_ADD_FIELD()
    ADD_FIELD(RECEIVER)
    ADD_FIELD(SIGNER)
    END_ADD_FIELD()

    // Start review
    START_REVIEW()
}

void sign_ux_flow_init(void)
{
    generic_intro_flow(display_sign_flow);
}

// ------------------ Transfer -------------------

static void display_transfer_flow(void)
{
    // Fill pairs
    START_ADD_FIELD()
    ADD_FIELD(AMOUNT)
    ADD_FIELD(RECEIVER)
    ADD_FIELD(SIGNER)
    END_ADD_FIELD()

    // Start review
    START_REVIEW()
}

void sign_transfer_ux_flow_init(void)
{
    generic_intro_flow(display_transfer_flow);
}

// ------------------ Function call -------------------

static void display_function_call_flow(void)
{
    // Fill fields
    START_ADD_FIELD()
    ADD_FIELD(DEPOSIT)
    ADD_FIELD(RECEIVER)
    ADD_FIELD(SIGNER)
    ADD_FIELD(ARGS)
    END_ADD_FIELD()

    // Start review
    START_REVIEW()
}

void sign_function_call_ux_flow_init()
{
    generic_intro_flow(display_function_call_flow);
}


// ------------------ Function call key -------------------
static void display_call_key_flow(void)
{
    // Fill fields
    START_ADD_FIELD()
    ADD_FIELD(TO_ACCOUNT)
    ADD_FIELD(CONTRACT)
    ADD_FIELD(ALLOWANCE)
    END_ADD_FIELD()

    // Start review
    START_REVIEW()
}

void sign_add_function_call_key_ux_flow_init()
{
    generic_intro_flow(display_call_key_flow);
}

// ------------------ NEP 413 -------------------
static void display_nep_413_flow(void)
{
    // Fill fields
    START_ADD_FIELD()
    ADD_FIELD(MESSAGE)
    ADD_FIELD(RECEIVER)
    END_ADD_FIELD()

    // Start review
    START_REVIEW()
}

void sign_message_nep_413_ux_flow_init()
{
    nep_413_intro_flow(display_nep_413_flow);
}

#endif

static void add_chunk_data(const uint8_t *input_data, size_t input_length)
{
    // if this is a first chunk
    PRINTF("Buffer used: %d\n", tmp_ctx.signing_context.buffer_used);
    if (tmp_ctx.signing_context.buffer_used == 0)
    {
        // then there is the bip32 path in the first chunk - first 20 bytes of data
        size_t path_size = sizeof(tmp_ctx.signing_context.bip32);
        if (input_length < path_size)
        {
            // TODO: Have specific error for underflow?
            THROW(SW_BUFFER_OVERFLOW);
        }
        read_path_from_bytes(input_data, tmp_ctx.signing_context.bip32);

        input_length -= path_size;
        // PRINTF("data_size: %d\n", data_size);

        memcpy(tmp_ctx.signing_context.buffer, &input_data[path_size], input_length);
        PRINTF("buffer: %.*h\n", input_length, tmp_ctx.signing_context.buffer);
    }
    else
    {
        // else update the data from entire segment.
        // PRINTF("data_size: %d\n", input_length);
        if (tmp_ctx.signing_context.buffer_used + input_length > MAX_DATA_SIZE)
        {
            THROW(SW_BUFFER_OVERFLOW);
        }
        memcpy(&tmp_ctx.signing_context.buffer[tmp_ctx.signing_context.buffer_used], input_data, input_length);
        PRINTF("buffer: %.*h\n", input_length, &tmp_ctx.signing_context.buffer[tmp_ctx.signing_context.buffer_used]);
    }
    tmp_ctx.signing_context.buffer_used += input_length;
}

void handle_signature_request(uint8_t p1, uint8_t p2, const uint8_t *input_buffer, uint16_t input_length, volatile unsigned int *flags, volatile unsigned int *tx)
{
    UNUSED(p2);
    UNUSED(tx);

    if (p1 != P1_MORE && p1 != P1_LAST)
    {
        THROW(SW_INCORRECT_P1_P2);
    }

    if (p1 == P1_LAST)
    {
        // TODO: Is network_byte used anywhere?
        tmp_ctx.signing_context.network_byte = p2;
        add_chunk_data(input_buffer, input_length);

        switch (parse_signature_request())
        {
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
        case SIGN_FLOW_NEP_413:
            sign_message_nep_413_ux_flow_init();
            break;
        case SIGN_PARSING_ERROR:
            THROW(SW_BUFFER_OVERFLOW);
        default:
            THROW(SW_CONDITIONS_NOT_SATISFIED);
        }
    }
    else
    {
        add_chunk_data(input_buffer, input_length);
        THROW(SW_OK);
    }

    *flags |= IO_ASYNCH_REPLY;
}
