#pragma once

#include "os.h"
#include <stdbool.h>

bool get_ed25519_public_key_for_path(const uint32_t* path, uint8_t raw_pubkey[static 32]);
uint32_t set_result_sign(void);
