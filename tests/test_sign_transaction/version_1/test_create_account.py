from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_create_account(firmware, backend, navigator: Navigator, test_name):
    """
     TransactionV1 {
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
            CreateAccount(
                CreateAccountAction,
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057f38000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions", "To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "a51f87f83c6ee724eaf0108188dd5e6f7b12f18bfefbe9ac6e2754c3aee578c65634c442e245bbaa9856a26011a0b85eadbc092cecf3f956190b3bbe4b04fa01"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
