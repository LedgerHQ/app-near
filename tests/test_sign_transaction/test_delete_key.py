from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_delete_key_ed25519(firmware, backend, navigator: Navigator, scenario_navigator, test_name):
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
            DeleteKey(
                DeleteKeyAction {
                    public_key: ed25519:AVHQN9NRrHGeagz7RiVEUyhB9aiSGZCJbXKbJbW8z63E,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000006008cf7fde291819ba810ce4ec49dff5a"
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
            data=bytes.fromhex("8002805711de081df2c5c3af03260cfd2c81dc1d18ad"),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "62ae46a48cbd3ac193cc6e5fed7a33fb877f4c6a775eb283eb8813b1a0168ee768252cd2d2208073ddde78974c0e84ceca7cde95254629ee2e3eed8671be170f"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, scenario_navigator, test_name, firmware)


def test_sign_delete_key_secp256k1(firmware, backend, navigator: Navigator, scenario_navigator, test_name):
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
            DeleteKey(
                DeleteKeyAction {
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000060161dd29ada831ab894b465a656c86c5"
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
                "800280573157c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "6b412a1b34e3e6c799e9d58db2acb99e1b38457157449824da5ad599a5ed96cd5388116058991d4539dc630513751ce8d10b126ee5d0a7a9c62bf54853b56502"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, scenario_navigator, test_name, firmware)
