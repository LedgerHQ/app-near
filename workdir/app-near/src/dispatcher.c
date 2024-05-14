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

#include <stdint.h>
#include <stdbool.h>

#include "buffer.h"
#include "io.h"

#include "dispatcher.h"

#include "get_public_key.h"
#include "get_wallet_id.h"
#include "sign_transaction.h"
#include "constants.h"
#include "app_main.h"


// Called by both the U2F and the standard communications channel
int apdu_dispatcher(const command_t *cmd) {
    if (cmd->cla != CLA) {
        return io_send_sw(SW_CLA_NOT_SUPPORTED);
    }

    switch (cmd->ins) {
        case INS_SIGN: 
            return handle_sign_transaction(cmd->p1, cmd->p2, cmd->data, cmd->lc);

        case INS_GET_PUBLIC_KEY:
            if (cmd->lc != 20) {
                return io_send_sw(SW_CONDITIONS_NOT_SATISFIED);
            }
            return handle_get_public_key(cmd->p1, cmd->p2, cmd->data, cmd->lc);

        case INS_GET_WALLET_ID:
            if (cmd->lc != 20) {
                return io_send_sw(SW_CONDITIONS_NOT_SATISFIED);
            }
            return handle_get_wallet_id(cmd->p1, cmd->p2, cmd->data, cmd->lc);

        case INS_GET_APP_CONFIGURATION:
            // NOTE: This allows using INS_GET_APP_CONFIGURATION as "reset state" command
            init_context();

            G_io_apdu_buffer[0] = MAJOR_VERSION;
            G_io_apdu_buffer[1] = MINOR_VERSION;
            G_io_apdu_buffer[2] = PATCH_VERSION;
            return io_send_response_pointer(G_io_apdu_buffer, 3, SW_OK);

        default:
            return io_send_sw(SW_INS_NOT_SUPPORTED);
    }
    return 0;
}
