/*******************************************************************************
*   Waves platform Wallet App for Nano Ledger S. Updated By Waves community.
*   Copyright (c) 2017-2018 Sergey Tolmachev (Tolsi) <tolsi.ru@gmail.com>
* 
*   Based on Sample code provided and (c) 2016 Ledger and 2017-2018 Jake B. (Burstcoin)
*
*   Based on Ledger app boilerplate (c) 2016 Ledger
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/

#include "utils.h"
#include "ui.h"
#include "get_public_key.h"
#include "get_wallet_id.h"
#include "sign_transaction.h"
#include "menu.h"
#include "main.h"
#include "near.h"
#include "crypto/ledger_crypto.h"
#include "dispatcher.h"
#include "constants.h"
#include "io.h"

// Temporary area to sore stuff and reuse the same memory
tmpContext_t tmp_ctx;
uiContext_t ui_context;

uint32_t deserialize_uint32_t(const uint8_t *buffer)
{
    uint32_t value = 0;

    value |= buffer[0] << 24;
    value |= buffer[1] << 16;
    value |= buffer[2] << 8;
    value |= buffer[3];
    return value;
}

// 20 bytes total
void read_path_from_bytes(const uint8_t *buffer, uint32_t *path) {
    path[0] = deserialize_uint32_t(buffer);
    path[1] = deserialize_uint32_t(buffer + 4);
    path[2] = deserialize_uint32_t(buffer + 8);
    path[3] = deserialize_uint32_t(buffer + 12);
    path[4] = deserialize_uint32_t(buffer + 16);
}

// like https://github.com/lenondupe/ledger-app-stellar/blob/master/src/main.c#L1784
uint32_t set_result_sign() {
    cx_ecfp_private_key_t private_key;
    get_private_key_for_path((uint32_t *) tmp_ctx.signing_context.bip32, &private_key);

    BEGIN_TRY {
        TRY {
            uint8_t signature[64];
            near_message_sign(&private_key, (unsigned char *)tmp_ctx.signing_context.buffer, tmp_ctx.signing_context.buffer_used, signature);

            memcpy(G_io_apdu_buffer, signature, sizeof(signature));
        } FINALLY {
            // reset all private stuff
            explicit_bzero(&private_key, sizeof(cx_ecfp_private_key_t));
        }
    }
    END_TRY;

    return 64;
}

void init_context() {
    memset(&tmp_ctx, 0, sizeof(tmp_ctx));
}

void app_main(void) {
    // Length of APDU command received in G_io_apdu_buffer
    int input_len = 0;
    // Structured APDU command
    command_t cmd;

    io_init();
    ui_idle();

    // DESIGN NOTE: the bootloader ignores the way APDU are fetched. The only
    // goal is to retrieve APDU.
    // When APDU are to be fetched from multiple IOs, like NFC+USB+BLE, make
    // sure the io_event is called with a
    // switch event, before the apdu is replied to the bootloader. This avoid
    // APDU injection faults.
    for (;;) {
        // Receive command bytes in G_io_apdu_buffer
        if ((input_len = io_recv_command()) < 0) {
            PRINTF("=> io_recv_command failure\n");
            return;
        }

        // Parse APDU command from G_io_apdu_buffer
        if (!apdu_parser(&cmd, G_io_apdu_buffer, input_len)) {
            PRINTF("=> /!\\ BAD LENGTH: %.*H\n", input_len, G_io_apdu_buffer);
            io_send_sw(SW_WRONG_DATA_LENGTH);
            continue;
        }

        PRINTF("=> CLA=%02X | INS=%02X | P1=%02X | P2=%02X | Lc=%02X | CData=%.*H\n",
                cmd.cla,
                cmd.ins,
                cmd.p1,
                cmd.p2,
                cmd.lc,
                cmd.lc,
                cmd.data);

        // Dispatch structured APDU command to handler
        if (apdu_dispatcher(&cmd) < 0) {
            PRINTF("=> apdu_dispatcher failure\n");
            return;
        }
    }
}
