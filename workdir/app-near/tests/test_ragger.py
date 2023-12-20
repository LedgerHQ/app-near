from pathlib import Path
from typing import Optional, Generator
from dataclasses import dataclass
from contextlib import contextmanager
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU, BackendInterface
from ragger.navigator import NavInsID, NavIns
from utils import get_version_from_makefile

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

DEFAULT_KEY = bytes.fromhex("c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f")
DEFAULT_PUBKEY = "ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U"

CLA = 0x80
INS_SIGN = 0x02
INS_GET_PUBKEY = 0x04
INS_GET_WALLET_ID = 0x05
INS_GET_APP_CONFIGURATION = 0x06


# Parameter 1 = not last APDU (INS_SIGN Sign instruction)
P1_MORE = 0x00
# Parameter 1 for screen confirmation for GET_PUBKEY.
P1_CONFIRM = 0x00
# Parameter 1 for NO screen confirmation for GET_PUBKEY.
P1_NO_CONFIRM = 0x01
# Parameter 1 = last APDU (INS_SIGN Sign instruction)
P1_LAST = 0x80
# Parameter 1 = Blind Signature (52 bytes, INS_SIGN Sign instruction)
P1_BLIND = 0x01           
# Parameter not used for this APDU
P1_P2_NOT_USED = 0x57

# Return codes
SW_OK                       = 0x9000
SW_CONDITIONS_NOT_SATISFIED = 0x6985
# `Blind Sign` disabled in settings
SW_SETTING_BLIND_DISABLED = 0x6192         
# buffer not equal to 52 bytes (20 bip32, 32 sha256) 
SW_BUFFER_WRONG_BLIND = 0x6191

# m/44'/397'/0'/0'/1
DERIV_PATH_DATA = bytes.fromhex('8000002c8000018d800000008000000080000001')


@dataclass
class Nearbackend():
    backend: BackendInterface

    def get_version(self) -> RAPDU:
        return self.backend.exchange(CLA, INS_GET_APP_CONFIGURATION, P1_P2_NOT_USED, P1_P2_NOT_USED, bytes())

    @contextmanager
    def sign_message(self, path: bytes, near_payload: bytes, blind=False) -> Generator[None, None, None]:
        if blind:
            data = path + near_payload    # payload is tx hash
            with self.backend.exchange_async(CLA,
                                             INS_SIGN,
                                             P1_BLIND,
                                             P1_P2_NOT_USED,
                                             data) as response:
                yield response
        else:
            cnt = 0
            data = path + near_payload    # payload is tx itself
            while cnt < len(data):
                to_send = min(len(data) - cnt, 255)

                if (cnt + to_send) >= len(data):
                    # Last APDU
                    with self.backend.exchange_async(CLA,
                                                     INS_SIGN,
                                                     P1_LAST,
                                                     P1_P2_NOT_USED,
                                                     bytes(data[cnt:(cnt + to_send)])) as response:
                        yield response
                else:
                    # Not last APDU
                    rapdu = self.backend.exchange(CLA, INS_SIGN, P1_MORE, P1_P2_NOT_USED, bytes(data[cnt:(cnt + to_send)]))
                    if rapdu.status != SW_OK:
                        return rapdu

                cnt += to_send


    @contextmanager
    def get_public_key_with_confirmation(self, path: bytes) -> Generator[None, None, None]:
        with self.backend.exchange_async(CLA,
                                         INS_GET_PUBKEY,
                                         P1_CONFIRM,
                                         P1_P2_NOT_USED,
                                         path) as response:
            yield response

    def get_public_key(self, path: str) -> RAPDU:
        return self.backend.exchange(CLA,
                                     INS_GET_PUBKEY,
                                     P1_NO_CONFIRM,
                                     P1_P2_NOT_USED,
                                     path)

    @contextmanager
    def get_wallet_id(self, path: bytes) -> Generator[None, None, None]:
        with self.backend.exchange_async(CLA,
                                         INS_GET_WALLET_ID,
                                         P1_P2_NOT_USED,
                                         P1_P2_NOT_USED,
                                         path) as response:
            yield response

    def get_async_response(self) -> Optional[RAPDU]:
        return self.backend.last_async_response


###################################################################
############################## TESTS ##############################
###################################################################

####################### APP VERSION TEST ##########################

# In this test we check that the get_version replies the right application version
def test_app_configuration(backend):
    # Use the app interface instead of raw interface
    client = Nearbackend(backend)
    # Send the get_version instruction to the app
    version = client.get_version().data
    assert len(version) == 3
    # Assert that we have received the correct app version compared as Makefile data
    assert (version[0], version[1], version[2]) == get_version_from_makefile()


####################### INFO MENU TEST ##########################
# In this test we check the behavior of the device info menu
def test_app_info_menu(firmware, navigator, test_name):
    # Navigate in the info menu
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK
        ]
    else:
        instructions = [
            NavInsID.USE_CASE_HOME_INFO,
            NavInsID.USE_CASE_SETTINGS_NEXT,
            NavInsID.USE_CASE_SETTINGS_MULTI_PAGE_EXIT
        ]
    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, instructions,
                                   screen_change_before_first_instruction=False)    


####################### PUBLIC KEY TESTS ##########################
def test_get_public_key_and_confirm_screen(firmware, backend, navigator, test_name):
    client = Nearbackend(backend)

    # Send the get pub key instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.get_public_key_with_confirmation(DERIV_PATH_DATA):

        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavIns(NavInsID.TOUCH, (200, 335)),
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_EXIT_QR,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)
    response = client.get_async_response()

    assert response.status == SW_OK
    assert response.data == DEFAULT_KEY

def test_get_public_key_no_confirm_screen(backend: BackendInterface):
    near = Nearbackend(backend)
    # Send the get pub key instruction (no confirmation).
    rapdu = near.get_public_key(DERIV_PATH_DATA)
    assert rapdu.status == SW_OK
    assert rapdu.data == DEFAULT_KEY

# In this test we check that the GET_PUBLIC_KEY in confirmation mode replies an error if the user refuses
def test_get_public_key_confirm_refused(firmware, backend, navigator, test_name):
    client = Nearbackend(backend)

    # Send the get pub key instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.get_public_key_with_confirmation(DERIV_PATH_DATA):
        # Disable raising when trying to unpack an error APDU
        backend.raise_policy = RaisePolicy.RAISE_NOTHING

        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Reject",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_STATUS_DISMISS]

            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)

    response = client.get_async_response()

    # Assert that we have received a refusal
    assert response.status == SW_CONDITIONS_NOT_SATISFIED
    assert len(response.data) == 0


# In this test we check that the GET_PUBLIC_KEY in confirmation mode replies an error if the user refuses
def test_get_public_key_confirm_refused_2(firmware, backend, navigator, test_name):
    if firmware.device.startswith("stax"):
        client = Nearbackend(backend)

        # Send the get pub key instruction.
        # As it requires on-screen validation, the function is asynchronous.
        # It will yield the result when the navigation is done
        with client.get_public_key_with_confirmation(DERIV_PATH_DATA):
            # Disable raising when trying to unpack an error APDU
            backend.raise_policy = RaisePolicy.RAISE_NOTHING

            # Validate the on-screen request by performing the navigation appropriate for this device
            instructions = [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CANCEL,
                NavInsID.USE_CASE_STATUS_DISMISS]

            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)
        response = client.get_async_response()

        # Assert that we have received a refusal
        assert response.status == SW_CONDITIONS_NOT_SATISFIED
        assert len(response.data) == 0



####################### WALLET ID TESTS ##########################
def test_get_wallet_id(firmware, backend, navigator, test_name):
    client = Nearbackend(backend)

    # Send the get wallet idinstruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.get_wallet_id(DERIV_PATH_DATA):

        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavIns(NavInsID.TOUCH, (200, 335)),
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_EXIT_QR,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)
    response = client.get_async_response()

    assert response.status == SW_OK
    assert response.data == DEFAULT_KEY

# In this test we check that the GET_WALLET_ID in confirmation mode replies an error if the user refuses
def test_get_wallet_id_refused(firmware, backend, navigator, test_name):
    client = Nearbackend(backend)

    # Send the get wallet id instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.get_wallet_id(DERIV_PATH_DATA):
        # Disable raising when trying to unpack an error APDU
        backend.raise_policy = RaisePolicy.RAISE_NOTHING

        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Reject",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_STATUS_DISMISS]

            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)

    response = client.get_async_response()

    # Assert that we have received a refusal
    assert response.status == SW_CONDITIONS_NOT_SATISFIED
    assert len(response.data) == 0


# In this test we check that the GET_WALLET_ID in confirmation mode replies an error if the user refuses
def test_get_wallet_id_refused_2(firmware, backend, navigator, test_name):
    if firmware.device.startswith("stax"):
        client = Nearbackend(backend)

        # Send the get wallet id instruction.
        # As it requires on-screen validation, the function is asynchronous.
        # It will yield the result when the navigation is done
        with client.get_wallet_id(DERIV_PATH_DATA):
            # Disable raising when trying to unpack an error APDU
            backend.raise_policy = RaisePolicy.RAISE_NOTHING

            # Validate the on-screen request by performing the navigation appropriate for this device
            instructions = [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CANCEL,
                NavInsID.USE_CASE_STATUS_DISMISS]

            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                        test_name,
                                        instructions)
        response = client.get_async_response()

        # Assert that we have received a refusal
        assert response.status == SW_CONDITIONS_NOT_SATISFIED
        assert len(response.data) == 0


####################### TRANSACTION TESTS ##########################
def navigate_sign_flow(firmware, navigator, test_name):
    """
    Validate the on-screen request by performing the navigation appropriate for this device
    """
    if firmware.device.startswith("nano"):
        navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                  [NavInsID.BOTH_CLICK],
                                                  "Approve",
                                                  ROOT_SCREENSHOT_PATH,
                                                  test_name,
                                                  screen_change_after_last_instruction = False)
    else:
        navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                  [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                  NavInsID.USE_CASE_STATUS_DISMISS,
                                                  NavInsID.WAIT_FOR_HOME_SCREEN],
                                                  "Hold to sign",
                                                  ROOT_SCREENSHOT_PATH,
                                                  test_name,
                                                  screen_change_after_last_instruction = False)

def generic_test_sign(backend, firmware, navigator, test_name, near_payload: bytes, expected_signature: bytes, blind=False):
    """
    Generic function to tests NEAR signature mechanism 
    """
    client = Nearbackend(backend)

    # Reset cx context
    client.get_version()

    with client.sign_message(DERIV_PATH_DATA, near_payload, blind):
        navigate_sign_flow(firmware, navigator, test_name)
    response = client.get_async_response()
    assert response.status == SW_OK
    assert response.data == expected_signature

# NOTE
# All the transaction payloads and the expected signatures used in the tests are generated with
# the rust file 'main.rs' located in folder tests/near_payload_generator/src

def test_sign_transfer(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "blablatest.testnet",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "speculos.testnet",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            Transfer(
                TransferAction {
                    deposit: 123400000000000000000000,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "12000000626c61626c61746573742e746573746e657400c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f0f7ac5e5c85700001000000073706563756c6f732e746573746e6574a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a010000000300002083c7f60387211a000000000000")
    expected_signature = bytes.fromhex(
        "e22eea0ee27a2d8e0bdfc72fb6337492d10a78aec15ff3cb6126b2944af920863e0907d5462bf2822a6bb0a62f1bb594e899ac96db7e95386895e91f325c460c")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)


def test_sign_function_call(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "blablatest.testnet",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "speculos.testnet",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: function_name,
                    args: `Dza`,
                    gas: 9999,
                    deposit: 1122334455,
                },
            ),
        ],
    }
    """

    near_payload = bytes.fromhex(
        "12000000626c61626c61746573742e746573746e657400c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f0f7ac5e5c85700001000000073706563756c6f732e746573746e6574a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a01000000020d00000066756e6374696f6e5f6e616d6502000000aabb0f27000000000000f776e542000000000000000000000000")
    expected_signature = bytes.fromhex(
        "e3329e9411101f0d0556a7106dc55ae10c04e234761e77ef9d78a53652ee8ed3f43c13761e0ea2edd4de21a534930932addb9d3e996b434aa47a3696f1ac2200")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)


def test_sign_stake(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "signer.near",
        ),
        public_key: ed25519:4c2pNM4aqrdTgaeRQyJnP9UwAFvHstDzZ1SCQAB7HnEc,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "receiver.near",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            Stake(
                StakeAction {
                    stake: 55556666,
                    public_key: ed25519:J6DmXMFwt894ZXED1BCGiK3y1aRhophVob5VwL8JBTm1,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "0b0000007369676e65722e6e65617200358c7177d702ee102a3cae18aa84b005bbd03b9188d5312e7d6df8f78d2a6a490f7ac5e5c85700000d00000072656365697665722e6e656172a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a01000000043aba4f0300000000000000000000000000fded04a996ebf5e25e7d6dd4c82edbbb544a397517edea03eadb39fb5211e460")
    expected_signature = bytes.fromhex(
        "e9a74d73b80de013ca90a2794855fa74d35c33697597905e03a3e1a483f68e5ed8e23dacd0b9ff62b881d7b0543c5dfb453fc2bd4e5dd16c8103e6ec72e3bc07")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)



def test_sign_add_key(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "arthur",
        ),
        public_key: ed25519:JCuJVU1tbr2tmYGX8b6f3YpvuN2zBZd2MZAYh16cNqGr,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            AddKey(
                AddKeyAction {
                    public_key: ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT,
                    access_key: AccessKey {
                        nonce: 12345,
                        permission: FullAccess,
                    },
                },
            ),
        ],
    }
    """

    near_payload = bytes.fromhex(
        "060000006172746875720053f9afa67ef91539ff38e2b36bbbed2d1dce6e18d06337cf6647389b5477359b0f7ac5e5c85700004000000039383739336364393161336638373066623132366636363238353830386337653039346166636663346564613861393730663636343863646630646264366465a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a0100000005002ffe256fd9a6e815abc3f220163413ac62871ecc5875d87625a35ce7ea65ee2f393000000000000001")
    expected_signature = bytes.fromhex(
        "003ee4ba2a305db02da9d929c95010e045f1a838dd9550c8e433398f6c8f6f2d71064ccc2b856a0949e2e9d678511a13679b35e787f47b28b68557969ee40b0a")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)


def test_sign_delete_key(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "speculosaccount",
        ),
        public_key: ed25519:JCuJVU1tbr2tmYGX8b6f3YpvuN2zBZd2MZAYh16cNqGr,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            DeleteKey(
                DeleteKeyAction {
                    public_key: ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "0f00000073706563756c6f736163636f756e7400ffa334478481a4a779c54ee30912f37ac23a323261f431f89d2652c277ca51ef0f7ac5e5c85700004000000039383739336364393161336638373066623132366636363238353830386337653039346166636663346564613861393730663636343863646630646264366465a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a0100000006005b4cf697ce3c6ded94c7adfa3c2d8310cc1dda88828a238b48513df3fdec7ab8")
    expected_signature = bytes.fromhex(
        "8c6d615c8b4afc0bfeaab1eab643d5b12ea0d8e017636b37b59433786c11fdd29b91432af4acdb116b88211e4bf4fdb0baaa50bc0243d47dcfed9d5121a98e04")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)


def test_sign_delete_account(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "speculosaccount",
        ),
        public_key: ed25519:7aE719urLcxUn81B9RkvXwDTgnXM7DEAy1eGWU2nDNd9,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "receiver.near",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            DeleteAccount(
                DeleteAccountAction {
                    beneficiary_id: AccountId(
                        "beneficiaryid",
                    ),
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "0f00000073706563756c6f736163636f756e740061a91abba0099d3ef23923645b37f19e6ebfeb220b238ee9abef3eeb32f851b40f7ac5e5c85700000d00000072656365697665722e6e656172a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a01000000070d00000062656e65666963696172796964")
    expected_signature = bytes.fromhex(
        "f518bcebfc57c3bb10d07511a5819f0f048868657661fb84a05f09297d3cadeb9e6e5995addbb6f9cd6c28a31474306adf0a41914a83161f380cd8e6a0a1a705")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)

def test_sign_multiple_actions_2_apdu_exchanges(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "blablatest.testnet",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "speculos.testnet",
        ),
        block_hash: C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK,
        actions: [
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "12000000626c61626c61746573742e746573746e657400c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f0f7ac5e5c85700001000000073706563756c6f732e746573746e6574a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a09000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000030000c071f0d12b84c31f000000000000")
    expected_signature = bytes.fromhex(
        "259308406966fd6f3d307c21e28dee16c42dca2806f88a03c96850f2c3eec4087f08d663123512b27fe387fda5dd9d65968a865db82c8fa305b48fa485a98601")
    generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)

# test fail --> code to improve
# def test_sign_add_key_function_call(firmware, backend, navigator, test_name):
#     """
#     Transaction {
#         signer_id: AccountId(
#             "b282b6f9a571a27d3ccde60f7fe194cdd498da775342a1587c662c44bf214fb2",
#         ),
#         public_key: ed25519:8MufT2vbLVH1tQxvZxNTbg1pamM3F6qSMv14X53wmdtC,
#         nonce: 96520360000015,
#         receiver_id: AccountId(
#             "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
#         ),
#         block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
#         actions: [
#             AddKey(
#                 AddKeyAction {
#                     public_key: ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT,
#                     access_key: AccessKey {
#                         nonce: 12345,
#                         permission: FunctionCall(
#                             FunctionCallPermission {
#                                 allowance: Some(
#                                     9999999999,
#                                 ),
#                                 receiver_id: "Receiver id",
#                                 method_names: [
#                                     "Method 0",
#                                     "Method 1",
#                                 ],
#                             },
#                         ),
#                     },
#                 },
#             ),
#         ],
#     }
#     """
#     near_payload = bytes.fromhex(
#         "4000000062323832623666396135373161323764336363646536306637666531393463646434393864613737353334326131353837633636326334346266323134666232006d5cf886f80bba79e1f0efb939002b2908e8947042b8dc3014fbea645e632b290f7ac5e5c85700004000000039383739336364393161336638373066623132366636363238353830386337653039346166636663346564613861393730663636343863646630646264366465a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a0100000005005b4cf697ce3c6ded94c7adfa3c2d8310cc1dda88828a238b48513df3fdec7ab839300000000000000001ffe30b540200000000000000000000000b000000526563656976657220696402000000080000004d6574686f642030080000004d6574686f642031")
#     expected_signature = bytes.fromhex(
#         "927cad5aee066055a3042de85c92a2cccee77df6582a7218b6ffe5129c8151729db553e9aec1a138a40bc90e487eb7e2f68c58b9aa71e3e2d3ade6ecc7e9270e")
#     generic_test_sign(backend, firmware, navigator, test_name, near_payload, expected_signature)


# In this test we send to the device a message to sign and cancel it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def generic_sign_message_cancel(backend, firmware, navigator, test_name, near_payload: bytes, blind=False):
    # Use the app interface instead of raw interface
    client = Nearbackend(backend)

    # Disable raising when trying to unpack an error APDU
    backend.raise_policy = RaisePolicy.RAISE_NOTHING

    if firmware.device.startswith("nano"):

        # Send the sign device instruction.
        # As it requires on-screen validation, the function is asynchronous.
        # It will yield the result when the navigation is done
        with client.sign_message(DERIV_PATH_DATA, near_payload, blind):
            # Validate the on-screen request by performing the navigation appropriate for this device
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Reject",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        # The device as yielded the result, parse it and ensure that the signature is correct
        response = client.get_async_response()

        # Assert that we have received a refusal
        assert response.status == SW_CONDITIONS_NOT_SATISFIED
        assert len(response.data) == 0
    else:
        instructions_list = [
            [
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS,
                NavInsID.WAIT_FOR_HOME_SCREEN
            ],
            [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS,
                NavInsID.WAIT_FOR_HOME_SCREEN
            ],
            [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS,
                NavInsID.WAIT_FOR_HOME_SCREEN
            ],
            [
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_CHOICE_REJECT,
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS,
                NavInsID.WAIT_FOR_HOME_SCREEN
            ]
        ]

        for i, instructions in enumerate(instructions_list):
            # Reset cx context
            client.get_version()
            
            # Send the sign device instruction.
            # As it requires on-screen validation, the function is asynchronous.
            # It will yield the result when the navigation is done
            with client.sign_message(DERIV_PATH_DATA, near_payload, blind):
                # Validate the on-screen request by performing the navigation appropriate for this device
                navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                               test_name + f"/part{i}",
                                               instructions,
                                               screen_change_after_last_instruction = False)

            # The device as yielded the result, parse it and ensure that the signature is correct
            response = client.get_async_response()

            # Assert that we have received a refusal
            assert response.status == SW_CONDITIONS_NOT_SATISFIED
            assert len(response.data) == 0

def test_sign_transfer_cancel(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "blablatest.testnet",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "speculos.testnet",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            Transfer(
                TransferAction {
                    deposit: 123400000000000000000000,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "12000000626c61626c61746573742e746573746e657400c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f0f7ac5e5c85700001000000073706563756c6f732e746573746e6574a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a010000000300002083c7f60387211a000000000000")
    generic_sign_message_cancel(backend, firmware, navigator, test_name, near_payload)



def test_sign_function_call_cancel(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "blablatest.testnet",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "speculos.testnet",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: function_name,
                    args: `Dza`,
                    gas: 9999,
                    deposit: 1122334455,
                },
            ),
        ],
    }
    """

    near_payload = bytes.fromhex(
        "12000000626c61626c61746573742e746573746e657400c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f0f7ac5e5c85700001000000073706563756c6f732e746573746e6574a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a01000000020d00000066756e6374696f6e5f6e616d6502000000aabb0f27000000000000f776e542000000000000000000000000")

    generic_sign_message_cancel(backend, firmware, navigator, test_name, near_payload)


def test_sign_stake_cancel(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "signer.near",
        ),
        public_key: ed25519:4c2pNM4aqrdTgaeRQyJnP9UwAFvHstDzZ1SCQAB7HnEc,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "receiver.near",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            Stake(
                StakeAction {
                    stake: 55556666,
                    public_key: ed25519:J6DmXMFwt894ZXED1BCGiK3y1aRhophVob5VwL8JBTm1,
                },
            ),
        ],
    }
    """
    near_payload = bytes.fromhex(
        "0b0000007369676e65722e6e65617200358c7177d702ee102a3cae18aa84b005bbd03b9188d5312e7d6df8f78d2a6a490f7ac5e5c85700000d00000072656365697665722e6e656172a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a01000000043aba4f0300000000000000000000000000fded04a996ebf5e25e7d6dd4c82edbbb544a397517edea03eadb39fb5211e460")
    generic_sign_message_cancel(backend, firmware, navigator, test_name, near_payload)



def test_sign_add_key_cancel(firmware, backend, navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "arthur",
        ),
        public_key: ed25519:JCuJVU1tbr2tmYGX8b6f3YpvuN2zBZd2MZAYh16cNqGr,
        nonce: 96520360000015,
        receiver_id: AccountId(
            "98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de",
        ),
        block_hash: `C32rfeBkSMT1xnsrArkV9Mu81ww9qK7n6Kw17NhEbVuK`,
        actions: [
            AddKey(
                AddKeyAction {
                    public_key: ed25519:79QAhRR464JQGr5ZXZe6jvXGM8NNgnR5KzRoQhaJyTYT,
                    access_key: AccessKey {
                        nonce: 12345,
                        permission: FullAccess,
                    },
                },
            ),
        ],
    }
    """

    near_payload = bytes.fromhex(
        "060000006172746875720053f9afa67ef91539ff38e2b36bbbed2d1dce6e18d06337cf6647389b5477359b0f7ac5e5c85700004000000039383739336364393161336638373066623132366636363238353830386337653039346166636663346564613861393730663636343863646630646264366465a3f5d1167a5c605fed71fc78d4381bef47a5acb3aba6fc9c07d7b8b912fc1e2a0100000005002ffe256fd9a6e815abc3f220163413ac62871ecc5875d87625a35ce7ea65ee2f393000000000000001")
    generic_sign_message_cancel(backend, firmware, navigator, test_name, near_payload)

def record_settings_screens_bs(firmware, navigator, test_name, reject_confirm_blindsign_popup=False):
    """
    test section to reflect blind sign is disabled in settings (with snapshots)
    """
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
        ]
    else:
        if not reject_confirm_blindsign_popup:
            instructions = [
                NavInsID.USE_CASE_HOME_INFO,
                NavInsID.USE_CASE_SETTINGS_NEXT,
                NavInsID.USE_CASE_SETTINGS_MULTI_PAGE_EXIT
            ]
            test_name += '_no_popup'
        else:
            instructions = [
                NavInsID.USE_CASE_HOME_INFO,
                NavInsID.USE_CASE_SETTINGS_NEXT,
                NavIns(NavInsID.TOUCH, (300, 116)), # toggle switch of setting
                NavInsID.USE_CASE_CHOICE_REJECT,
                NavInsID.USE_CASE_SETTINGS_NEXT,
                NavInsID.USE_CASE_SETTINGS_MULTI_PAGE_EXIT
            ]
            test_name += '_reject_popup'
            
    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, instructions,
                                   screen_change_before_first_instruction=False)    
    
def test_sign_blind_not_enabled_error(firmware, backend, navigator, test_name):
    """
    records settings' screens and tests for expected SW_SETTING_BLIND_DISABLED APDU
    returned status on signature attempt
    """
    # Disable raising when trying to unpack an error APDU
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)

    near_tx_hash = bytes.fromhex("f7e4d5d16fb1329282414b0820a6137d11fd6ec6e6718c1d369f1b0becccb49b")

    for click_confirm_popup in [False, True]:
        record_settings_screens_bs(firmware, navigator, test_name, click_confirm_popup)
        with client.sign_message(DERIV_PATH_DATA, near_tx_hash, blind=True):
            pass

        response = client.get_async_response()
        assert response.status == SW_SETTING_BLIND_DISABLED
        assert len(response.data) == 0

def enable_blind_sign_in_settings(firmware, navigator, test_name, record=False):
    """
    test section to reflect blind sign is disabled in settings (with snapshots)
    """
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
        ]
    else:
        instructions = [
            NavInsID.USE_CASE_HOME_INFO,
            NavInsID.USE_CASE_SETTINGS_NEXT,
            NavIns(NavInsID.TOUCH, (300, 116)), # toggle switch of setting
            NavInsID.USE_CASE_CHOICE_CONFIRM,
            NavInsID.USE_CASE_SETTINGS_NEXT,
            NavInsID.USE_CASE_SETTINGS_MULTI_PAGE_EXIT
        ]
    if record:
        navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH, test_name, instructions,
                                       screen_change_before_first_instruction=False)    
    else:
        navigator.navigate(instructions, screen_change_before_first_instruction=False)    

def test_sign_blind_too_short_payload(firmware, backend, navigator, test_name):
    """
    enables blind sign in settings and tests for expected SW_BUFFER_WRONG_BLIND APDU
    returned status on signature attempt
    """
    # Disable raising when trying to unpack an error APDU
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)

    near_tx_hash = bytes.fromhex("f7e4")
    enable_blind_sign_in_settings(firmware, navigator, test_name)

    with client.sign_message(DERIV_PATH_DATA, near_tx_hash, blind=True):
        pass

    response = client.get_async_response()
    assert response.status == SW_BUFFER_WRONG_BLIND
    assert len(response.data) == 0

def test_sign_blind_ok(firmware, backend, navigator, test_name):
    """
    enables blind sign in settings and tests for expected SW_OK APDU
    returned status on signature success
    """
    client = Nearbackend(backend)

    enable_blind_sign_in_settings(firmware, navigator, test_name + '_switch_setting', True)

    near_tx_hash = bytes.fromhex("f7e4d5d16fb1329282414b0820a6137d11fd6ec6e6718c1d369f1b0becccb49b")
    expected_signature = bytes.fromhex(
        "e0043ac06ad798f9631c1862e73fdd1b26ea2e7a3eda070ac940a8baaae7e247f2eb3b2d8ae0f87ab3979858614be7f869da207854a753b05a67adf58a9d7c08")

    generic_test_sign(backend, firmware, navigator, test_name + "_sign_flow", near_tx_hash, expected_signature, blind=True)

def test_sign_blind_cancel(firmware, backend, navigator, test_name):
    """
    enables blind sign in settings and tests for expected SW_CONDITIONS_NOT_SATISFIED APDU
    returned status on aborting signing process in-between
    """
    client = Nearbackend(backend)

    enable_blind_sign_in_settings(firmware, navigator, test_name, False)

    near_tx_hash = bytes.fromhex("f7e4d5d16fb1329282414b0820a6137d11fd6ec6e6718c1d369f1b0becccb49b")

    generic_sign_message_cancel(backend, firmware, navigator, test_name, near_tx_hash, blind=True)
