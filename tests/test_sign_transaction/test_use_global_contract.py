from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_deploy_contract_using_global_hash(
    firmware, backend, navigator: Navigator, test_name
):
    """
    transaction length: 267
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            UseGlobalContract(
                UseGlobalContractAction {
                    contract_identifier: CodeHash(9m1EzTMRTbSzqTNwyDDEW5nYX14XhnDQzn7L1BtGHQKc),
                },
            ),
        ],
    },
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000063346635393431653831653037316332666431646165326537316664336438353964343632343834333931643961393062663231393231316463626233323066ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000a00822352e19e76d52bfdee0810652d10"
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
            data=bytes.fromhex("80028057114e47695112adc814dc39b7eb78f30be143"),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "e030d1a0ae8f2da786a366489d640a740133b543ed446d44d2e439e5f3b494f5394f2e489b0ab03f729fd5d37ae247d8643767d6f003ed958ccbb0cbab160a09"
                ),
            ),
        ),
    ]

    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_deploy_contract_using_account_id(
    firmware, backend, navigator: Navigator, test_name
):
    """
    transaction length: 267
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            UseGlobalContract(
                UseGlobalContractAction {
                    contract_identifier: AccountId(AccountId("dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c")),
                },
            ),
        ],
    },
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000063346635393431653831653037316332666431646165326537316664336438353964343632343834333931643961393062663231393231316463626233323066ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000a01400000006463376533346565636563"
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
                "80028057353330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "683297e19dc0209b7103c2a8e5c409fe85c3b24555ef43e1b71d7a2a45c044cc2083666b527005876cb7a07837a72ed3066d31577c31e2cab6bf965049fe3808"
                ),
            ),
        ),
    ]

    generic_test_sign(client, chunks, navigator, test_name, firmware)
