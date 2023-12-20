#include "io.h"

#include <string.h>
#include "context.h"
#include "os.h"
#include "base58.h"

#define COPY_LITERAL(dst, src) \
    memcpy(dst, src, sizeof(src))

int blind_sign_init_ui_context() {
      memset(&ui_context, 0, sizeof(uiContext_t));
      COPY_LITERAL(ui_context.line1, "blind sign SHA256");
      if (base58_encode(tmp_ctx.signing_context.buffer, SHA256_SIZE, ui_context.line2, _32_BYTES_HEX_STR_LEN - 1) < 0) {
        // Unreachable: _32_BYTES_HEX_STR_LEN - 1 is always greater than 45, which is max len of base58 of 32 bytes input 
        return io_send_sw(EXCEPTION);
      }
      return 0;
}

