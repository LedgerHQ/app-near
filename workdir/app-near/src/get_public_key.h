#include "os.h"
#include "cx.h"
#include "globals.h"

#ifndef _GET_PUBLIC_KEY_H_
#define _GET_PUBLIC_KEY_H_

enum p1_values_e {
    DISPLAY_AND_CONFIRM = 0,
    RETURN_ONLY = 1,
};

int handle_get_public_key(uint8_t p1, uint8_t p2, const uint8_t *input_buffer, uint16_t input_length);

#endif
