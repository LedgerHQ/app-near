from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    FINISH_STUB_APDU,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_nep413_msg(firmware, backend, navigator: Navigator, test_name):
    """
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
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "eb1200a990ba295ebd3b5a49729a30734179d2414cb43bd8af39b7103ac4dcdfd3174409a434a1f6a48d267e4f46492886129343076f8315afaf9e761183490e"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name)
