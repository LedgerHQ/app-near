#*******************************************************************************
#   Ledger App
#   (c) 2023 Ledger
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#*******************************************************************************

ifeq ($(BOLOS_SDK),)
$(error Environment variable BOLOS_SDK is not set)
endif

include $(BOLOS_SDK)/Makefile.defines

# Application name
APPNAME = "NEAR"

#Application version
APPVERSION_M = 1
APPVERSION_N = 2
APPVERSION_P = 4
APPVERSION = $(APPVERSION_M).$(APPVERSION_N).$(APPVERSION_P)

# Application source files
APP_SOURCE_PATH += src

# Application icons following guidelines:
ICON_NANOS = icons/app_near_16px.gif
ICON_NANOX = icons/app_near_14px.gif
ICON_NANOSP = icons/app_near_14px.gif
ICON_STAX = icons/app_near_32px.gif

CURVE_APP_LOAD_PARAMS = ed25519
PATH_APP_LOAD_PARAMS = "44'/397'" 

VARIANT_PARAM = COIN
VARIANT_VALUES = near

ENABLE_BLUETOOTH = 1
ENABLE_NBGL_QRCODE = 1

APP_WEBUSB_URL = wallet.near.org

# U2F
DEFINES   += HAVE_U2F HAVE_IO_U2F
DEFINES   += U2F_PROXY_MAGIC=\"NEAR\"
SDK_SOURCE_PATH += lib_u2f 

include $(BOLOS_SDK)/Makefile.standard_app
