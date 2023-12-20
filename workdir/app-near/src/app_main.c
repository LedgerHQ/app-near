/*****************************************************************************
 *   Ledger App Near.
 *   (c) 2023 Ledger SAS.
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
 *****************************************************************************/

#include "menu.h"
#include "globals.h"
#include "app_main.h"
#include "dispatcher.h"
#include "io.h"
#include "utils.h"

// Temporary area to store stuff and reuse the same memory
tmpContext_t tmp_ctx;
uiContext_t ui_context;

void init_context() {
    memset(&tmp_ctx, 0, sizeof(tmp_ctx));
}

void nv_app_state_init()
{
    // Initialize the NVM data if required
    if (N_storage.initialized != 0x01)
    {
        internalStorage_t storage;
        storage.blind_sign_enabled = 0x00;
        storage.initialized = 0x01;
        nvm_write((void *)&N_storage, &storage, sizeof(internalStorage_t));
    }
    blind_sign_enabled = N_storage.blind_sign_enabled;
}

void app_main(void) {
    // Length of APDU command received in G_io_apdu_buffer
    int input_len = 0;
    // Structured APDU command
    command_t cmd;

    io_init();

    nv_app_state_init();
    ui_idle();

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
