from application_client.client import (
    AsyncAPDU,
    SW_DENY,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign_single_review,
)
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_stake(firmware, backend, navigator: Navigator, test_name):
    """
    1.16 NEAR
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            Stake(
                StakeAction {
                    stake: 1157130000000000000000000,
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                },
            ),
        ],
    }

    New combined flow: first APDU (prefix) is processed silently, second APDU
    triggers the combined "Review stake" screen directly.

    NOTE: the expected signature bytes below are placeholders. Run this test
    once against speculos with --snapshot-dir tests/snapshots to capture the
    real snapshots and signature, then update the bytes.fromhex() value in
    expected_response with the actual signature returned by the device.
    """
    client = Nearbackend(backend)
    chunks = [
        # First APDU contains BIP32 path + tx prefix — no UI interaction needed.
        bytes.fromhex(
            "80020057d58000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000040000e82982269b2408f5000000000000"
        ),
        # Second APDU contains the Stake action — triggers combined review.
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                    "95eae42061262857b00a3294e82419bb910954e0f105999ba1df9c7d5e144a3c33bcb3c5225b3a2728f8d0e8252293befbe5d8233d2150bb2922191a548f6205"
                ),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_reject_stake(firmware, backend, navigator: Navigator, test_name):
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d58000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000040000e82982269b2408f5000000000000"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b"
            ),
            navigable_conditions=NavigableConditions(
                value=["Reject"],
            ),
            expected_response=RAPDU(SW_DENY, bytes()),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_stake_long_account_ids(firmware, backend, navigator: Navigator, test_name):
    """
    Transaction {
        signer_id: AccountId(
            "long-staking-account-ssssssssssssssssssssssssssssssssssssss.near",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "long-validator-pool-name-ppppppppppppppppppppppppppp.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            Stake(
                StakeAction {
                    stake: 1157130000000000000000000,
                    public_key: ed25519:5KporntzQkHiVMKxnFuUGcwPKhtVdhaFkneEkhnoZi1U,
                },
            ),
        ],
    }

    Signer and receiver are both 64 bytes, the longest a NEAR account ID can be;
    the combined review still shows them in full, with the stake amount, the
    pool and the validator public key.
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057e38000002c8000018d800000008000000080000001400000006c6f6e672d7374616b696e672d6163636f756e742d7373737373737373737373737373737373737373737373737373737373737373737373737373737373737300c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e0000400000006c6f6e672d76616c696461746f722d706f6f6c2d6e616d652d707070707070707070707070707070707070707070707070707070707070707070707070707070ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805738583101000000040000e82982269b2408f500000000000000404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                    "26d692721921dcbfe71e9dde891ee328304549d86dfa9baca0362db7d0e1dccb842856ed49dd40ebaa14b5f2a2198b54e2a9ab2379e345712c102d7b5503cd07"
                ),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)
