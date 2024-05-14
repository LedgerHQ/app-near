#pragma once 

#include "os.h"
#include "cx.h"
#include "globals.h"


typedef enum rlpTxType {
    TX_LENGTH = 0,
    TX_TYPE,
    TX_SENDER,
    TX_RECIPIENT,
    TX_AMOUNT,
    TX_FEE
} rlpTxType;

int format_long_decimal_amount(size_t input_size, char *input, size_t output_size, char *output, int nomination);
void bin_to_hex(char *out, const uint8_t *in, size_t len);
void send_response(uint8_t tx, bool approve);
void read_path_from_bytes(const uint8_t *buffer, uint32_t *path);
