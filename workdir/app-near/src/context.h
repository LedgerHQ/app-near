#pragma once

#include <stdint.h>

#include "constants.h"

#define SHA256_SIZE 32
#define _32_BYTES_HEX_STR_LEN 65

// A place to store information about the transaction
// for displaying to the user when requesting approval
// 64 bytes for addresses and 44 bytes for other data (+1 byte for \0)
typedef struct uiContext_t {
    char line1[45];
    char line2[_32_BYTES_HEX_STR_LEN];
    char line3[_32_BYTES_HEX_STR_LEN];
    char line5[45];
    char amount[45];
    char long_line[250];
} uiContext_t;

// A place to store data during the signing
typedef struct signingContext_t {
    // bip32 path
    uint32_t bip32[5];
    uint8_t buffer[MAX_DATA_SIZE];
    uint32_t buffer_used;
    unsigned char network_byte;
} signingContext_t;

// A place to store data during the confirming the address
typedef struct addressesContext_t {
    uint8_t public_key[32];
} addressesContext_t;

typedef union {
    signingContext_t signing_context;
    addressesContext_t address_context;
} tmpContext_t;

extern uiContext_t ui_context;

extern tmpContext_t tmp_ctx; // Temporary area to store stuff

