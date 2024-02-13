from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator

def test_sign_delete_account_short(firmware, backend, navigator: Navigator, test_name):
    """
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
            DeleteAccount(
                DeleteAccountAction {
                    beneficiary_id: AccountId(
                        "bob.near",
                    ),
                },
            ),
        ],
    }   
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057f68000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000708000000626f622e6e656172"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "4abe2be79a2120d66e2bd17b92e843cd94e1a661e115e728d5f0d317272590c2872b40b323d74642862ff4f0cd584e2d874c3a440e68dcb829cf60788c510905"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name)
