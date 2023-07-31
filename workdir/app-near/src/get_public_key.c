#include "get_public_key.h"
#include "base58.h"
#include "utils.h"
#include "main.h"
#include "os.h"
#include "ux.h"
#include "glyphs.h"
#include "io.h"
#include "ledger_crypto.h"

#define ADDRESS_PREFIX "ed25519:"
#define ADDRESS_PREFIX_SIZE strlen(ADDRESS_PREFIX)

static char address[FULL_ADDRESS_LENGTH];

static uint32_t set_result_get_public_key()
{
    memcpy(G_io_apdu_buffer, tmp_ctx.address_context.public_key, 32);
    return 32;
}

//////////////////////////////////////////////////////////////////////

#ifdef HAVE_BAGL

UX_STEP_NOCB(
    ux_display_public_flow_5_step,
    bnnn_paging,
    {
        .title = "Public Key",
        .text = address,
    });
UX_STEP_VALID(
    ux_display_public_flow_6_step,
    pb,
    send_response(set_result_get_public_key(), true),
    {
        &C_icon_validate_14,
        "Approve",
    });
UX_STEP_VALID(
    ux_display_public_flow_7_step,
    pb,
    send_response(0, false),
    {
        &C_icon_crossmark,
        "Reject",
    });

UX_FLOW(
    ux_display_public_flow,
    &ux_display_public_flow_5_step,
    &ux_display_public_flow_6_step,
    &ux_display_public_flow_7_step);

void display_public_key(void)
{
    ux_flow_init(0, ux_display_public_flow, NULL);
}

#endif

#ifdef HAVE_NBGL

#include "nbgl_use_case.h"
#include "menu.h"

static void display_public_key_done(bool validated)
{
    if (validated) {
        nbgl_useCaseStatus("ADDRESS\nVERIFIED", true, ui_idle);
    } else {
        nbgl_useCaseStatus("Address verification\ncancelled", false, ui_idle);
    }
}

static void address_verification_cancelled(void) {
    send_response(0, false);
    // Display "cancelled" screen
    display_public_key_done(false);
}

static void display_address_callback(bool confirm)
{
    if (confirm)
    {
        send_response(set_result_get_public_key(), true);
        // Display "verified" screen
        display_public_key_done(true);
    }
    else
    {
        address_verification_cancelled();
    }
}

// called when tapping on review start page to actually display address
static void display_addr(void) 
{
    
    nbgl_useCaseAddressConfirmation(address, &display_address_callback);
}

static void display_public_key(void)
{
    nbgl_useCaseReviewStart(
        &C_stax_app_near_64px,
        "Verify " APPNAME "\naddress",
        NULL,
        "Cancel",
        display_addr,
        address_verification_cancelled
    );
}

#endif

int handle_get_public_key(uint8_t p1, uint8_t p2, const uint8_t *input_buffer, uint16_t input_length)
{
    UNUSED(p2);

    init_context();

    // Get the public key and return it.
    cx_ecfp_public_key_t public_key;

    uint32_t path[5];
    if (input_length < sizeof(path))
    {
        return io_send_sw(INVALID_PARAMETER);
    }
    read_path_from_bytes(input_buffer, path);

    if (!get_ed25519_public_key_for_path(path, public_key.W))
    {
        return io_send_sw(INVALID_PARAMETER);
    }

    memcpy(tmp_ctx.address_context.public_key, public_key.W, 32);

    memset(address, 0, sizeof(address));
    strcpy(address, ADDRESS_PREFIX);
    if (base58_encode(tmp_ctx.address_context.public_key, sizeof(tmp_ctx.address_context.public_key),
                      address + ADDRESS_PREFIX_SIZE, sizeof(address) - ADDRESS_PREFIX_SIZE - 1) < 0)
    {
        return io_send_sw(INVALID_PARAMETER);
    }

    if (p1 == RETURN_ONLY)
    {
        send_response(set_result_get_public_key(), true);
    }
    else if (p1 == DISPLAY_AND_CONFIRM)
    {
        display_public_key();
    }
    else
    {
        return io_send_sw(SW_INCORRECT_P1_P2);
    }
    return 0;
}
