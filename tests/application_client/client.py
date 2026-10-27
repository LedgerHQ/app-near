from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import Generator, Optional, List, Union
from ragger.navigator import NavIns, NavInsID, Navigator

from ragger.backend.interface import RAPDU, BackendInterface
from common import ROOT_SCREENSHOT_PATH


CLA = 0x80
INS_GET_APP_CONFIGURATION = 0x06
INS_GET_PUBKEY = 0x04

# Parameter not used for this APDU
P1_P2_NOT_USED = 0x57
# Parameter 1 for screen confirmation for GET_PUBKEY.
P1_CONFIRM = 0x00

# Return codes
SW_OK = 0x9000
SW_DENY = 0x6985
SW_TX_PARSING_FAIL = 0xB005
SW_BLIND_SIGNING_DISABLED = 0xB00E

# Navigable conditions of the screens shown when a value cannot be displayed in full.
BLIND_SIGNING_DISABLED = "Blind signing disabled"
BLIND_SIGNING_AHEAD = "Blind signing ahead"
BLIND_SIGNING_BACK_TO_SAFETY = "Back to safety"

# Where the blind-signing switch sits on the first settings page of touchscreen devices.
BLIND_SIGNING_SWITCH = {
    "stax": (200, 113),
    "flex": (200, 113),
    "apex_p": (150, 90),
}

FINISH_STUB_APDU = RAPDU(0xFFFF, bytes())


@dataclass(frozen=True)
class NavigableConditions:
    value: List[str]


@dataclass(frozen=True)
class AsyncAPDU:
    data: bytes
    navigable_conditions: NavigableConditions
    expected_response: RAPDU


@dataclass
class Nearbackend:
    backend: BackendInterface

    def get_version(self) -> RAPDU:
        return self.backend.exchange(
            CLA, INS_GET_APP_CONFIGURATION, P1_P2_NOT_USED, P1_P2_NOT_USED, bytes()
        )

    @contextmanager
    def get_public_key_with_confirmation(self, path: bytes) -> Generator[None, None, None]:
        with self.backend.exchange_async(
            CLA, INS_GET_PUBKEY, P1_CONFIRM, P1_P2_NOT_USED, path
        ) as response:
            yield response

    def get_async_response(self) -> Optional[RAPDU]:
        return self.backend.last_async_response

    def sign_message_chunks(
        self, chunks: List[Union[bytes, AsyncAPDU]]
    ) -> Generator[Union[NavigableConditions, RAPDU], None, RAPDU]:
        for chunk in chunks:
            if isinstance(chunk, AsyncAPDU):
                with self.backend.exchange_async_raw(chunk.data):
                    yield chunk.navigable_conditions
                yield chunk.expected_response
            elif isinstance(chunk, bytes):
                rapdu = self.backend.exchange_raw(chunk)
                if rapdu.status != SW_OK:
                    return rapdu
            else:
                raise TypeError("bytes or AsyncAPDU expected")
        return FINISH_STUB_APDU

def condition_folder_name(event_index: int, additional_index: bool, condition_index: int):
    if additional_index:
        return str(event_index) + "_" + str(condition_index)
    return str(event_index)


def enable_blind_signing(navigator: Navigator, firmware):
    """Turns the blind-signing setting on from the home screen, and goes back to it."""
    instructions: List[Union[NavIns, NavInsID]]
    if firmware.device.startswith("nano"):
        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
        ]
    else:
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, BLIND_SIGNING_SWITCH[firmware.device]),
            NavInsID.USE_CASE_SETTINGS_MULTI_PAGE_EXIT,
        ]
    navigator.navigate(instructions, screen_change_before_first_instruction=False)


def navigate_blind_signing(
    navigator: Navigator,
    condition: str,
    condition_folder: Path,
    firmware,
):
    """Answers the screens shown before a value that cannot be displayed in full."""
    if firmware.device.startswith("nano"):
        # The "disabled" page is a single page, which any button dismisses.
        # The texts are regex patterns anchored at the start of each screen event. When the
        # page is drawn right after "Receiving transaction" (single-chunk NEP-413 message),
        # speculos reports both pages' texts in one event, so this one must match anywhere.
        text = {
            BLIND_SIGNING_DISABLED: ".*Blind signing must",
            BLIND_SIGNING_AHEAD: "Continue",
            BLIND_SIGNING_BACK_TO_SAFETY: "Reject",
        }[condition]
        navigator.navigate_until_text_and_compare(
            NavInsID.RIGHT_CLICK,
            [NavInsID.BOTH_CLICK],
            text,
            ROOT_SCREENSHOT_PATH,
            condition_folder,
            screen_change_after_last_instruction=False,
        )
    else:
        # "Go to settings" and "Back to safety" are the top buttons of these choice pages,
        # "Reject transaction" and "Continue anyway" the footer ones.
        text, validation = {
            BLIND_SIGNING_DISABLED: (
                "Reject transaction",
                [NavInsID.USE_CASE_CHOICE_REJECT],
            ),
            BLIND_SIGNING_AHEAD: ("Continue anyway", [NavInsID.USE_CASE_CHOICE_REJECT]),
            BLIND_SIGNING_BACK_TO_SAFETY: ("Back to safety", [NavInsID.USE_CASE_CHOICE_CONFIRM]),
        }[condition]
        navigator.navigate_until_text_and_compare(
            NavInsID.USE_CASE_REVIEW_TAP,
            validation,
            text,
            ROOT_SCREENSHOT_PATH,
            condition_folder,
            # After "Continue anyway" the review follows in the same APDU: its own
            # navigation waits for it to appear.
            screen_change_after_last_instruction=condition != BLIND_SIGNING_AHEAD,
        )


def navigate_conditions(
    navigator: Navigator,
    conditions: NavigableConditions,
    event_index: int,
    test_name,
    firmware,
):
    reject_labels = {"Reject", "Cancel"}
    blind_signing_conditions = {
        BLIND_SIGNING_DISABLED,
        BLIND_SIGNING_AHEAD,
        BLIND_SIGNING_BACK_TO_SAFETY,
    }
    for cond_index, condition in enumerate(conditions.value):
        str_index = condition_folder_name(event_index, len(conditions.value) > 1, cond_index)
        condition_folder = Path(test_name) / (
            str_index + "_" + condition.lower().replace(" ", "_").replace("!", "_bang")
        )
        if condition in blind_signing_conditions:
            navigate_blind_signing(navigator, condition, condition_folder, firmware)
        elif firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(
                NavInsID.RIGHT_CLICK,
                [NavInsID.BOTH_CLICK],
                condition,
                ROOT_SCREENSHOT_PATH,
                condition_folder,
                screen_change_after_last_instruction=False,
            )
        else:
            if condition == "Sign":
                navigator.navigate_until_text_and_compare(
                    NavInsID.USE_CASE_REVIEW_TAP,
                    [
                        NavInsID.USE_CASE_REVIEW_CONFIRM
                    ],
                    condition,
                    ROOT_SCREENSHOT_PATH,
                    condition_folder,
                    screen_change_after_last_instruction=True,
                )
            elif condition in reject_labels:
                navigator.navigate_until_text_and_compare(
                    NavInsID.USE_CASE_REVIEW_TAP,
                    [NavInsID.USE_CASE_REVIEW_REJECT, NavInsID.USE_CASE_CHOICE_CONFIRM],
                    condition,
                    ROOT_SCREENSHOT_PATH,
                    condition_folder,
                    screen_change_after_last_instruction=True,
                )
            else:
                navigator.navigate_until_text_and_compare(
                    NavInsID.USE_CASE_REVIEW_TAP,
                    [
                        NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CONFIRM,
                    ],
                    condition,
                    ROOT_SCREENSHOT_PATH,
                    condition_folder,
                    screen_change_after_last_instruction=True,
                )


def assert_async_response(client: Nearbackend, expected_response: RAPDU):
    response = client.get_async_response()

    assert response.status == expected_response.status, hex(response.status)  # type: ignore
    assert response.data == expected_response.data, response.data.hex()  # type: ignore


def generic_test_sign(
    client: Nearbackend,
    chunks: List[Union[bytes, AsyncAPDU]],
    navigator: Navigator,
    test_name,
    firmware,
):
    numbered_chunks = enumerate(client.sign_message_chunks(chunks))

    try:
        while True:
            index, chunk_event = next(numbered_chunks)
            if isinstance(chunk_event, NavigableConditions):
                navigate_conditions(navigator, chunk_event, index, test_name, firmware)
            elif isinstance(chunk_event, RAPDU):
                assert_async_response(client, chunk_event)
    except StopIteration as e:
        if e.value != FINISH_STUB_APDU:
            raise AssertionError(e.value) from e


def generic_test_sign_single_review(
    client: Nearbackend,
    chunks: List[Union[bytes, AsyncAPDU]],
    navigator: Navigator,
    test_name,
    firmware,
):
    numbered_chunks = enumerate(client.sign_message_chunks(chunks))
    saw_review = False

    try:
        while True:
            index, chunk_event = next(numbered_chunks)
            if isinstance(chunk_event, NavigableConditions):
                assert not saw_review
                assert len(chunk_event.value) == 1

                saw_review = True
                navigate_conditions(navigator, chunk_event, index, test_name, firmware)
            elif isinstance(chunk_event, RAPDU):
                assert_async_response(client, chunk_event)
    except StopIteration as e:
        assert saw_review
        if e.value != FINISH_STUB_APDU:
            raise AssertionError(e.value) from e
