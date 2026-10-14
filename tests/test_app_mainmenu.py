from ragger.navigator import NavIns, NavInsID

from application_client.client import BLIND_SIGNING_SWITCH

from common import ROOT_SCREENSHOT_PATH


# In this test we check the behavior of the device main menu
def test_app_mainmenu(firmware, navigator, test_name):
    # Navigate in the main menu
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
        ]
    else:
        instructions = [
            NavInsID.USE_CASE_HOME_INFO,
            NavInsID.LEFT_HEADER_TAP
        ]
    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, instructions,
                                   screen_change_before_first_instruction=False)


# In this test we check that the blind-signing setting is disabled by default, and toggles
def test_app_settings_blind_signing(firmware, navigator, test_name):
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK,
        ]
    else:
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, BLIND_SIGNING_SWITCH[firmware.device]),
            NavIns(NavInsID.TOUCH, BLIND_SIGNING_SWITCH[firmware.device]),
        ]
    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, instructions,
                                   screen_change_before_first_instruction=False)
