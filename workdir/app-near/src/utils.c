#include "os.h"
#include "cx.h"
#include <stdbool.h>
#include <stdlib.h>
#include "utils.h"
#include "menu.h"
#include "os_utils.h"

/*
 Adapted from https://en.wikipedia.org/wiki/Double_dabble#C_implementation
 Returns: length of resulting string or -1 for error
*/
static int format_long_int_amount(size_t input_size, char *input, size_t output_size, char *output) {
    // NOTE: Have to copy to have word-aligned array (otherwise crashing on read)
    // Lots of time has been lost debugging this, make sure to avoid unaligned RAM access (as compiler in BOLOS SDK won't)
    uint16_t aligned_amount[8];
    memcpy(aligned_amount, input, 16);
    // Convert size in bytes into words
    size_t n = input_size / 2;

    size_t nbits = 16 * n;       /* length of arr in bits */
    size_t nscratch = nbits / 3; /* length of scratch in bytes */
    if (nscratch >= output_size) {
        // Output buffer is too small
        output[0] = '\0';
        return -1;
    }

    char *scratch = output;

    size_t i, j, k;
    size_t smin = nscratch - 2; /* speed optimization */

    for (i = 0; i < n; ++i) {
        for (j = 0; j < 16; ++j) {
            /* This bit will be shifted in on the right. */
            int shifted_in = (aligned_amount[n - i - 1] & (1 << (15 - j))) ? 1 : 0;

            /* Add 3 everywhere that scratch[k] >= 5. */
            for (k = smin; k < nscratch; ++k) {
                scratch[k] += (scratch[k] >= 5) ? 3 : 0;
            }

            /* Shift scratch to the left by one position. */
            if (scratch[smin] >= 8) {
                smin -= 1;
            }
            for (k = smin; k < nscratch - 1; ++k) {
                scratch[k] <<= 1;
                scratch[k] &= 0xF;
                scratch[k] |= (scratch[k + 1] >= 8);
            }

            /* Shift in the new bit from arr. */
            scratch[nscratch - 1] <<= 1;
            scratch[nscratch - 1] &= 0xF;
            scratch[nscratch - 1] |= shifted_in;
        }
    }

    /* Remove leading zeros from the scratch space. */
    for (k = 0; k < nscratch && k < output_size - 1; ++k) { 
        if (scratch[k] != 0) {
            break;
        }
    }
    nscratch -= k;
    memmove(scratch, scratch + k, nscratch + 1);

    /* Convert the scratch space from BCD digits to ASCII. */
    for (k = 0; k < nscratch; ++k) {
        scratch[k] += '0';
    }

    /* Resize and return */
    memmove(output, scratch, nscratch + 1);
    return nscratch;
}

int format_long_decimal_amount(size_t input_size, char *input, size_t output_size, char *output, int nomination) {
    int len = format_long_int_amount(input_size, input, output_size, output);

    if (len < 0 || (size_t) len + 2 > output_size) {
        // Output buffer is too small
        output[0] = '\0';
        return -1;
    }

    if (len <= nomination) {
        // < 1.0
        memmove(output + 2 + (nomination - len), output, len);
        /* coverity[bad_memset] */
        memset(output + 2, '0', (nomination - len));
        output[0] = '0';
        output[1] = '.';
        len = nomination + 2;
    } else {
        // >= 1.0
        int int_len = len - nomination;
        memmove(output + int_len + 1, output + int_len, nomination);
        output[int_len] = '.';
        len = len + 1;
    }

    // Remove trailing zeros
    output[len] = '0';
    while (len > 0 && output[len] == '0') {
        output[len--] = 0;
    }

    // Remove trailing dot
    if (output[len] == '.') {
        output[len] = 0;
    }

    return len;
}

void bin_to_hex(char *out, const uint8_t *in, size_t len) {
    const unsigned char hex_digits[] = {'0', '1', '2', '3', '4', '5', '6', '7',
                                        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};

    while (len--) {
        *out++ = hex_digits[(*in >> 4) & 0xF];
        *out++ = hex_digits[(*in++) & 0xF];
    }
    *out = 0;
}

void send_response(uint8_t tx, bool approve) {
    G_io_apdu_buffer[tx++] = approve? 0x90 : 0x69;
    G_io_apdu_buffer[tx++] = approve? 0x00 : 0x85;
    // Send back the response, do not restart the event loop
    io_exchange(CHANNEL_APDU | IO_RETURN_AFTER_TX, tx);

    #ifdef HAVE_BAGL
    // Display back the original UX
    ui_idle();
    #endif
}

// 20 bytes total
void read_path_from_bytes(const uint8_t *buffer, uint32_t *path) {
    path[0] = U4BE(buffer, 0);
    path[1] = U4BE(buffer, 4);
    path[2] = U4BE(buffer, 8);
    path[3] = U4BE(buffer, 12);
    path[4] = U4BE(buffer, 16);
}

