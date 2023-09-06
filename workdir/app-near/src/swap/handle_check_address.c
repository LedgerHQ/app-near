/*******************************************************************************
 *   NEAR Ledger Wallet
 *   (c) 2023 Ledger
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

#include <string.h>

#include "os.h"
#include "crypto_helpers.h"
#include "swap.h"
#include "bip32.h"
#include "main.h"
#include "ledger_crypto.h"
#include "base58.h"

#define FULL_ADDRESS_LENGTH 60
#define PUBLIC_KEY_SIZE  65
#define BASE58CHECK_ADDRESS_SIZE 44
#define PATH_LEN 5

typedef struct {
    uint32_t indices[MAX_BIP32_PATH];
    uint8_t length;
} bip32_path_t;

static int derive_public_key(const uint8_t *buffer,
                             uint16_t buffer_length,
                             uint8_t public_key[static PUBLIC_KEY_SIZE],
                             char address58[static BASE58CHECK_ADDRESS_SIZE + 1]) {
    char address[FULL_ADDRESS_LENGTH];
    uint32_t path[PATH_LEN];

    if (buffer_length < PATH_LEN) {
        PRINTF("read_bip32_path failed\n");
        return -1;
    }

    read_path_from_bytes(buffer, path);

    if (!get_ed25519_public_key_for_path(path, public_key))
    {
        PRINTF("get_ed25519_public_key_for_path failed\n");
        return -1;
    }

    if (base58_encode(public_key, 32,
                      address, FULL_ADDRESS_LENGTH) < 0)
    {
        PRINTF("base58_encode failed\n");
        return -1;
    }
    address[BASE58CHECK_ADDRESS_SIZE] = '\0';
    memmove(address58, address, BASE58CHECK_ADDRESS_SIZE + 1);
    return 0;
}

/* Set params.result to 0 on error, 1 otherwise */
void swap_handle_check_address(check_address_parameters_t *params) {
    PRINTF("Inside NEAR swap_handle_check_address\n");
    params->result = 0;

    if (params->address_parameters == NULL) {
        PRINTF("derivation path expected\n");
        return;
    }

    if (params->address_to_check == NULL) {
        PRINTF("Address to check expected\n");
        return;
    }
    PRINTF("Address to check %s\n", params->address_to_check);

    if (params->extra_id_to_check == NULL) {
        PRINTF("extra_id_to_check expected\n");
        return;
    } else if (params->extra_id_to_check[0] != '\0') {
        PRINTF("extra_id_to_check expected empty, not '%s'\n", params->extra_id_to_check);
        return;
    }

    uint8_t public_key[PUBLIC_KEY_SIZE];
    char address58[BASE58CHECK_ADDRESS_SIZE + 1];
    if (derive_public_key(params->address_parameters + 1,
                          params->address_parameters_length,
                          public_key,
                          address58) != 0) {
        PRINTF("Failed to derive public key\n");
        return;
    }
    // Only address58 is useful in this context
    UNUSED(public_key);

    if (strcmp(params->address_to_check, address58) != 0) {
        PRINTF("Address %s != %s\n", params->address_to_check, address58);
        return;
    }

    PRINTF("Addresses match\n");

    params->result = 1;
    return;
}
