from application_client.client import (
    BLIND_SIGNING_AHEAD,
    BLIND_SIGNING_BACK_TO_SAFETY,
    BLIND_SIGNING_DISABLED,
    AsyncAPDU,
    SW_BLIND_SIGNING_DISABLED,
    SW_DENY,
    SW_OK,
    SW_TX_PARSING_FAIL,
    NavigableConditions,
    Nearbackend,
    FINISH_STUB_APDU,
    enable_blind_signing,
    generic_test_sign,
)
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_nep413_msg(firmware, backend, navigator: Navigator, test_name):
    """
    NEP413Payload {
        message: "Makes it possible to authenticate users without having to add new access keys. \
This will improve UX, save money and will not increase the on-chain storage of the users' \
accounts.",
        nonce: [
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
        ],
        recipient: "alice.near",
        callback_url: Some(
            "myapp.com/callback",
        ),
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ca8000002c8000018d800000008000000080000001b20000004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057452a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "d9f0d5b4feea6e091f00c92ee6923c1b03a59c9fc71780f2dc7dbdfcf9da0da8f31b5e01298a1c629a92eaafe07dd8c78c056faf27baf7d8f88d53a100272508"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_message_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    The 536-byte message exceeds what the review can display (400 bytes): with blind
    signing disabled (the default) it is rejected before any review.

    NEP413Payload {
        message: "Makes it possible to authenticate users without having to add new ac\
cess keys. This will improve UX, save money and will not increase the on-chain storage \
of the users' accounts./Makes it possible to authenticate users without having to add n\
ew access keys. This will improve UX, save money and will not increase the on-chain sto\
rage of the users' accounts./Makes it possible to authenticate users without having to \
add new access keys. This will improve UX, save money and will not increase the on-chai\
n storage of the users' accounts.",
        nonce: [
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
            42,
        ],
        recipient: "alice.near",
        callback_url: Some(
            "myapp.com/callback",
        ),
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057fa8000002c8000018d800000008000000080000001180200004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f7574"
        ),
        bytes.fromhex(
            "80070057fa20686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e64207769"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057816c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_callback_url_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose 105-byte callback url exceeds what the review can display
    (100 bytes): with blind signing disabled (the default) it is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ca8000002c8000018d800000008000000080000001b20000004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780579c2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201690000006d796170702e636f6d2f6363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_recipient_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose 65-byte recipient exceeds what the review can display
    (64 bytes): it is rejected with a parsing error before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ca8000002c8000018d800000008000000080000001b20000004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780577c2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_nep413_msg_fields_at_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose fields are each exactly as long as the review can display:
    a 400-byte message, a 64-byte recipient and a 100-byte callback url.

    Every field is reviewed in full and signed.
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ff8000002c8000018d800000008000000080000001900100004c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e"
        ),
        bytes.fromhex(
            "80070057fa742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f202a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a40000000727272727272727272727272727272727272727272727272727272727272727272727272727272727272727272"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780577c72727272727272727272727272722e6e656172016400000068747470733a2f2f6d796170702e636f6d2f63616c6c6261636b3f63636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "e3ed4ebef60d09a8ec308af2a0e3c769bf85c21cdfa00d1b9079c200e7a7758b63160ddf28a5410e1fffe2b339d33494d16393313b743df22e106351e4c02001"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_message_one_over_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose message is 401 bytes, one over what the review can display.

    With blind signing disabled (the default) it is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ff8000002c8000018d800000008000000080000001910100004c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e"
        ),
        bytes.fromhex(
            "80070057fa742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a400000007272727272727272727272727272727272727272727272727272727272727272727272727272727272727272"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780577d7272727272727272727272727272722e6e656172016400000068747470733a2f2f6d796170702e636f6d2f63616c6c6261636b3f63636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_callback_url_one_over_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose callback url is 101 bytes, one over what the review can display.

    With blind signing disabled (the default) it is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80070057ff8000002c8000018d800000008000000080000001900100004c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e"
        ),
        bytes.fromhex(
            "80070057fa742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e204c6f6720696e20746f202a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a40000000727272727272727272727272727272727272727272727272727272727272727272727272727272727272727272"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780577d72727272727272727272727272722e6e656172016500000068747470733a2f2f6d796170702e636f6d2f63616c6c6261636b3f6363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_message_not_printable_ascii(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose message is valid UTF-8 but not printable ASCII (a line feed).

    The review would draw that character as a square glyph shared by every such byte,
    so with blind signing disabled (the default) the message is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057838000002c8000018d800000008000000080000001260000004c6f6720696e20746f206d796170702e636f6d0a776974682074686973206163636f756e742e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a0000006d796170702e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_recipient_not_printable_ascii(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose recipient is valid UTF-8 but not printable ASCII (an accented letter; a NEP-413 recipient is never validated on chain).

    The review would draw that character as a square glyph shared by every such byte,
    so with blind signing disabled (the default) the message is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057858000002c8000018d800000008000000080000001260000004c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0c0000006d79617070c3a92e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_callback_url_not_printable_ascii(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose callback url is valid UTF-8 but not printable ASCII (an escape control byte).

    The review would draw that character as a square glyph shared by every such byte,
    so with blind signing disabled (the default) the message is rejected before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057848000002c8000018d800000008000000080000001260000004c6f6720696e20746f206d796170702e636f6d20776974682074686973206163636f756e742e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a0000006d796170702e6e65617201130000006d796170702e636f6d2f63616c6c6261636b1b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)

def test_sign_nep413_msg_message_too_long_blind_signing(firmware, backend, navigator: Navigator, test_name):
    """
    The 536-byte message of test_reject_nep413_msg_message_too_long, with blind signing
    enabled: after the warning, the review shows the first 400 bytes and a "Message size"
    field ("536 bytes, first 400 shown"), and the message is signed.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    enable_blind_signing(navigator, firmware)
    chunks = [
        bytes.fromhex(
            "80070057fa8000002c8000018d800000008000000080000001180200004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f7574"
        ),
        bytes.fromhex(
            "80070057fa20686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e64207769"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057816c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_AHEAD, "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                "eb1200a990ba295ebd3b5a49729a30734179d2414cb43bd8af39b7103ac4dcdfd3174409a434a1f6a48d267e4f46492886129343076f8315afaf9e761183490e"
            ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)

def test_sign_nep413_msg_message_not_printable_ascii_blind_signing(firmware, backend, navigator: Navigator, test_name):
    """
    The line-feed message of test_reject_nep413_msg_message_not_printable_ascii, with blind
    signing enabled: after the warning, the review shows the message with the line feed drawn
    as a square glyph, and it is signed.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    enable_blind_signing(navigator, firmware)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057838000002c8000018d800000008000000080000001260000004c6f6720696e20746f206d796170702e636f6d0a776974682074686973206163636f756e742e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a0000006d796170702e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_AHEAD, "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                "db43c5a34f55b0468e08ece66df56f30aaa2484f18378a0bfd2c207430113ad2832f4f2459b0d3986e1ff94e6fb46ace33fbeef0f2a253a5633a45f12f53d508"
            ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)

def test_sign_nep413_msg_callback_url_too_long_blind_signing(firmware, backend, navigator: Navigator, test_name):
    """
    The 105-byte callback url of test_reject_nep413_msg_callback_url_too_long, with blind
    signing enabled: the review shows its first 100 bytes and a "Callback Url size" field,
    and the message is signed.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    enable_blind_signing(navigator, firmware)
    chunks = [
        bytes.fromhex(
            "80070057ca8000002c8000018d800000008000000080000001b20000004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800780579c2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201690000006d796170702e636f6d2f6363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363636363"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_AHEAD, "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                "db53ec658de6842d213c7eced8b54c9b5cee325f83dbe30d97e090d182a3c7c1335b3695f2df12f53ecf7416ad38e89bdb1656d4d81d5f79d4f69e55f838410b"
            ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)

def test_reject_nep413_msg_message_too_long_back_to_safety(firmware, backend, navigator: Navigator, test_name):
    """
    With blind signing enabled, the user turns back at the warning: the message is not
    signed.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    enable_blind_signing(navigator, firmware)
    chunks = [
        bytes.fromhex(
            "80070057fa8000002c8000018d800000008000000080000001180200004d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f7574"
        ),
        bytes.fromhex(
            "80070057fa20686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e642077696c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2f4d616b657320697420706f737369626c6520746f2061757468656e74696361746520757365727320776974686f757420686176696e6720746f20616464206e657720616363657373206b6579732e20546869732077696c6c20696d70726f76652055582c2073617665206d6f6e657920616e64207769"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80078057816c6c206e6f7420696e63726561736520746865206f6e2d636861696e2073746f72616765206f662074686520757365727327206163636f756e74732e2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a0a000000616c6963652e6e65617201120000006d796170702e636f6d2f63616c6c6261636b"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_BACK_TO_SAFETY],
            ),
            expected_response=RAPDU(
                SW_DENY,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
