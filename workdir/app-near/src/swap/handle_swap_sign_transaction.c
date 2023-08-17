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
#ifdef HAVE_SWAP

#include "handle_swap_sign_transaction.h"
#include "swap.h"
#include "utils.h"
#include "format.h"

#define BASE58CHECK_ADDRESS_SIZE 34

typedef struct swap_validated_s {
    bool initialized;
    uint64_t amount;
    uint64_t fee;
    char recipient[BASE58CHECK_ADDRESS_SIZE + 1];
} swap_validated_t;

static swap_validated_t G_swap_validated;

// Save the BSS address where we will write the return value when finished
static uint8_t* G_swap_sign_return_value_address;

// Save the data validated during the Exchange app flow
bool swap_copy_transaction_parameters(create_transaction_parameters_t* params) {
    PRINTF("Inside NEAR swap_copy_transaction_parameters\n");

    // Ensure no extraid
    if (params->destination_address_extra_id == NULL) {
        PRINTF("destination_address_extra_id expected\n");
        return false;
    } else if (params->destination_address_extra_id[0] != '\0') {
        PRINTF("destination_address_extra_id expected empty, not '%s'\n",
               params->destination_address_extra_id);
        return false;
    }

    // first copy parameters to stack, and then to global data.
    // We need this "trick" as the input data position can overlap with app globals
    // and also because we want to memset the whole bss segment as it is not done
    // when an app is called as a lib.
    // This is necessary as many part of the code expect bss variables to
    // initialized at 0.
    swap_validated_t swap_validated;
    memset(&swap_validated, 0, sizeof(swap_validated));

    // Save recipient
    strlcpy(swap_validated.recipient,
            params->destination_address,
            sizeof(swap_validated.recipient));
    if (swap_validated.recipient[sizeof(swap_validated.recipient) - 1] != '\0') {
        PRINTF("Address copy error\n");
        return false;
    }

    // Save amount and fees
    swap_str_to_u64(params->amount, params->amount_length, &swap_validated.amount);
    swap_str_to_u64(params->fee_amount, params->fee_amount_length, &swap_validated.fee);

    swap_validated.initialized = true;

    // Full reset the global variables
    os_explicit_zero_BSS_segment();

    // Keep the address at which we'll reply the signing status
    G_swap_sign_return_value_address = &params->result;

    // Commit from stack to global data, params becomes tainted but we won't access it anymore
    memcpy(&G_swap_validated, &swap_validated, sizeof(swap_validated));
    swap_validated.initialized = true;
    return true;
}

// Check that the amount in parameter is the same as the previously saved amount
static bool check_swap_amount(const char* amount) {
    char validated_amount[MAX_PRINTABLE_AMOUNT_SIZE];
    uint8_t decimals = 0;

    memset(validated_amount, 0, sizeof(validated_amount));
    if (!format_fpu64_trimmed(validated_amount, sizeof(validated_amount), G_swap_validated.amount, decimals)) {
        return false;
    }

    if (strcmp(amount, validated_amount) != 0) {
        PRINTF("Amount requested in this transaction = %s\n", amount);
        PRINTF("Amount validated in swap = %s\n", validated_amount);
        return false;
    }

    return true;
}

// Check that the fee in parameter is zero
static bool check_swap_fee(void) {
    if (G_swap_validated.fee != 0) {
        PRINTF("Fee requested in this transaction = %d\n, should be 0", G_swap_validated.fee);
        return false;
    }

    return true;
}

bool swap_check_validity(const char* amount,
                         const char* toAddress) {
    PRINTF("Inside Near swap_check_validity\n");

    if (!G_swap_validated.initialized) {
        return false;
    }

    if (!check_swap_amount(amount)) {
        return false;
    }

    if (!check_swap_fee()) {
        return false;
    }

    if (strcmp(G_swap_validated.recipient, toAddress) != 0) {
        PRINTF("Recipient requested in this transaction = %s\n", toAddress);
        PRINTF("Recipient validated in swap = %s\n", G_swap_validated.recipient);
        return false;
    }

    PRINTF("VALID!\n");

    return true;
}

void __attribute__((noreturn)) swap_finalize_exchange_sign_transaction(bool is_success) {
    *G_swap_sign_return_value_address = is_success;
    os_lib_end();
}

#endif  // HAVE_SWAP
