from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator

def test_sign_add_key_fullaccess(firmware, backend, navigator: Navigator, test_name):
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
            AddKey(
                AddKeyAction {
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    access_key: AccessKey {
                        nonce: 127127127127,
                        permission: FullAccess,
                    },
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000050161dd29ada831ab894b465a656c86c5"
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
                "800280573a57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d00000001"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "29c5b77982cc05ec949c4d6f8fc795679eaf2830dee89a3d101901e133cca2f4ecfc7592b7a285960ecece9daee767fef1251c84e502a6dbce4b3b63808de804"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name)

def test_sign_add_key_functioncall(firmware, backend, navigator: Navigator, test_name):
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
            AddKey(
                AddKeyAction {
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    access_key: AccessKey {
                        nonce: 127127127127,
                        permission: FunctionCall(
                            FunctionCallPermission {
                                allowance: Some(
                                    150000000000000000000,
                                ),
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
                                method_names: [
                                    "first_method",
                                    "saturating_add_signed",
                                    "iterator_chain_to_do_multiple_instances_of_an_operation_that_can_fail",
                                    "from_residual",
                                    "from_output",
                                    "unwrap_err_unchecked",
                                    "try_reserve_exact",
                                    "first_method",
                                    "saturating_add_signed",
                                    "iterator_chain_to_do_multiple_instances_of_an_operation_that_can_fail",
                                ],
                            },
                        ),
                    },
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000050161dd29ada831ab894b465a656c86c5"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex("80020057fa57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab2108000000000000008000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963316231316233623331363733303333393336616430376264646330316639646132376439373438313165343830666231393763373939653233343830613438390a0000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e"),
        bytes.fromhex("80020057fa6564450000006974657261746f725f636861696e5f746f5f646f5f6d756c7469706c655f696e7374616e6365735f6f665f616e5f6f7065726174696f6e5f746861745f63616e5f6661696c0d00000066726f6d5f726573696475616c0b00000066726f6d5f6f757470757414000000756e777261705f6572725f756e636865636b6564110000007472795f726573657276655f65786163740c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e6564450000006974657261746f725f636861696e5f746f5f646f5f6d756c7469706c655f696e7374616e6365735f6f665f616e5f6f706572617469"),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057106f6e5f746861745f63616e5f6661696c"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "437f513b357b513f3e4eadb17a7273da9aa51af0de8faa9e4a51b93b91aaa55db6a836b5ffd6986d85f6ef6d114274b0f8fce440464399f563ff7bf44d7fcc08"
                ),
            ),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name)
