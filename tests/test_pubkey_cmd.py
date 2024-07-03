from application_client.client import Nearbackend, SW_OK
from ragger.navigator import NavInsID, Navigator
from common import ROOT_SCREENSHOT_PATH, DERIV_PATH_DATA, DEFAULT_KEY


def test_get_public_key_and_confirm_screen(firmware, backend, navigator: Navigator, test_name):
    client = Nearbackend(backend)

    # Send the get pub key instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.get_public_key_with_confirmation(DERIV_PATH_DATA):
        # Validate the on-screen request by performing the navigation appropriate for this device
        navigator.navigate_until_text_and_compare(
            NavInsID.RIGHT_CLICK, [NavInsID.BOTH_CLICK], "Approve", ROOT_SCREENSHOT_PATH, test_name
        )
    response = client.get_async_response()

    assert response.status == SW_OK
    assert response.data == DEFAULT_KEY
