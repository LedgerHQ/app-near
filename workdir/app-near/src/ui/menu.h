#pragma once

#include "os.h"

#ifdef HAVE_BAGL
#include "menu_bagl.h"
#endif

#ifdef HAVE_NBGL
#include "menu_nbgl.h"
#endif

extern volatile uint8_t blind_sign_enabled;
