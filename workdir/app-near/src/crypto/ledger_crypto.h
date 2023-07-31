#ifndef __LEDGER_CRYPTO_H__
#define __LEDGER_CRYPTO_H__

#include "os.h"
#include <stdbool.h>

void public_key_le_to_be(uint8_t raw_pubkey[static 65]);
bool get_ed25519_public_key_for_path(const uint32_t* path, uint8_t raw_pubkey[static 65]);

#endif