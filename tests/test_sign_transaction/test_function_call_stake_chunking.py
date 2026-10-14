from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
    generic_test_sign_single_review,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_unstake_method_name_ends_at_chunk_end(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose method name ends exactly at the end of the first
    APDU.

    The staking flow is detected from the first APDU and the combined review is
    shown.

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
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d08000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b65"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805742260000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("380be8e588e1e819026491728878f73f5cfa48714a348d3bd0f8cdaabc27aa681a46d65b7f5d5bb37c8da2bd60926da2d13f77f4114c26aefccd2dae3a5d600e"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_method_name_split_across_chunks(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose method name ends one byte after the first APDU.

    The staking flow cannot be detected from the first APDU, so the generic
    review is shown.

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
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057cf8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280574365260000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("380be8e588e1e819026491728878f73f5cfa48714a348d3bd0f8cdaabc27aa681a46d65b7f5d5bb37c8da2bd60926da2d13f77f4114c26aefccd2dae3a5d600e"),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
