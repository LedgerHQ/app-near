from application_client.client import (
    AsyncAPDU,
    SW_OK,
    SW_TX_PARSING_FAIL,
    NavigableConditions,
    Nearbackend,
    FINISH_STUB_APDU,
    generic_test_sign,
)
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_nep413_msg(firmware, backend, navigator: Navigator, test_name):
    """
    NEP413Payload {
        message: "Makes it possible to authenticate users without having to add new access keys. This wil\
l improve UX, save money and will not increase the on-chain storage of the users' accou\
nts.",
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
    The 536-byte message exceeds what the review can display (400 bytes): it is
    rejected with a parsing error before any review.

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
                value=[],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_nep413_msg_callback_url_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    NEP-413 message whose 105-byte callback url exceeds what the review can display
    (100 bytes): it is rejected with a parsing error before any review.
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
                value=[],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
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
