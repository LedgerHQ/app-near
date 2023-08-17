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

#include "swap.h"

#include "handle_swap_sign_transaction.h"
#include "utils.h"
#include "format.h"

/* Set empty printable_amount on error, printable amount otherwise */
void swap_handle_get_printable_amount(get_printable_amount_parameters_t* params) {
    uint8_t decimals = 0;
    uint64_t amount;

    PRINTF("Inside NEAR swap_handle_get_printable_amount\n");

    if (!swap_str_to_u64(params->amount, params->amount_length, &amount)) {
        PRINTF("Amount is too big\n");
        goto error;
    }

    memset(params->printable_amount, '\0', sizeof(params->printable_amount));
   
    if (!format_fpu64_trimmed(params->printable_amount, sizeof(params->printable_amount), amount, decimals)) {
        PRINTF("print_amount failed\n");
        goto error;
    }

    strlcat(params->printable_amount, " ", sizeof(params->printable_amount));
    strlcat(params->printable_amount, "NEAR", sizeof(params->printable_amount));

    PRINTF("Amount %s\n", params->printable_amount);
    return;

error:
    PRINTF("Error\n");
    memset(params->printable_amount, '\0', sizeof(params->printable_amount));
}

#endif  // HAVE_SWAP
