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


def test_sign_delegate_action_simple(firmware, backend, navigator: Navigator, test_name):
    """
    DelegateAction {
        sender_id: AccountId(
            "bob.near",
        ),
        receiver_id: AccountId(
            "alice.near",
        ),
        actions: [
            NonDelegateAction(
                Transfer(
                    TransferAction {
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
        ],
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057748000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e65617201000000030000c071f0d12b84c31f000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions", "To NEP366 suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "c6645407278a472641350472fc83eb8002ef961ecf67102df5976adb5a071208db7309975dc0a56f7c5b604ea45ccfdf3d0a78be221c4afcee6aae03d394690c"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name)
