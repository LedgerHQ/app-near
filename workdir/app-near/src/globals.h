#pragma once 
#include "os.h"
#include "ux.h"
#include "os_io_seproxyhal.h"


#define P1_CONFIRM 0x01
#define P1_NON_CONFIRM 0x00

#define FULL_ADDRESS_LENGTH 60
#define BIP32_PATH 5

// display stepped screens
extern unsigned int ux_step;
extern unsigned int ux_step_count;

typedef struct internalStorage_t {
    unsigned char dummy_setting_1;
    unsigned char dummy_setting_2;
    uint8_t initialized;
} internalStorage_t;

extern const internalStorage_t N_storage_real;
#define N_storage (*(volatile internalStorage_t*) PIC(&N_storage_real))
