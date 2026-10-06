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


def test_sign_transfer(firmware, backend, navigator: Navigator, test_name):
    """
    1234.56 NEAR
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            Transfer(
                TransferAction {
                    deposit: 1234560000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000030000001049f203b43f34fd0300000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "a8cc807ff4e83df0a5834af232550ccd15f09f829e0e73a88b6175e41d001085f2087d0b03fb36faf07d3b261d2d11a0e7b92a244c0dfd57e6daa3b91ae42205"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_signer_id_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    Transfer whose signer_id is 65 bytes, one over the longest valid account ID ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.near").

    The transaction is rejected with a parsing error before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057ea8000002c8000018d800000008000000080000001410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e65617200c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
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


def test_reject_receiver_id_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    Transfer whose receiver_id is 65 bytes, one over the longest valid account ID ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.near").

    The transaction is rejected with a parsing error before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057ea8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e0000410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
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
