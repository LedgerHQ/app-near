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
import pytest


def test_sign_batch_transaction_all_actions(firmware, backend, navigator: Navigator, test_name):
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
            CreateAccount(
                CreateAccountAction,
            ),
            DeleteAccount(
                DeleteAccountAction {
                    beneficiary_id: AccountId(
                        "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
                    ),
                },
            ),
            DeleteKey(
                DeleteKeyAction {
                    public_key: ed25519:AVHQN9NRrHGeagz7RiVEUyhB9aiSGZCJbXKbJbW8z63E,
                },
            ),
            DeleteKey(
                DeleteKeyAction {
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                },
            ),
            Stake(
                StakeAction {
                    stake: 1157130000000000000000000,
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                },
            ),
            AddKey(
                AddKeyAction {
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    access_key: AccessKey {
                        nonce: 127127127127,
                        permission: FullAccess,
                    },
                },
            ),
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
                                    "from_residual",
                                    "from_output",
                                    "unwrap_err_unchecked",
                                    "try_reserve_exact",
                                ],
                            },
                        ),
                    },
                },
            ),
            Transfer(
                TransferAction {
                    deposit: 150000000000000000000000,
                },
            ),
            DeployContract(
                DeployContractAction {
                    code: KioqKioqKioqKioqKioqKioqKioqKioqKioqKioq,
                },
            ),
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: eyJ2ZXN0aW5nX3NjaGVkdWxlX3dpdGhfc2FsdCI6eyJ2ZXN0aW5nX3NjaGVkdWxlIjp7InN0YXJ0X3RpbWVzdGFtcCI6IjE1Nzc5MTk2MDAwMDAwMDAwMDAiLCJjbGlmZl90aW1lc3RhbXAiOiIxNjA5NDU1NjAwMDAwMDAwMDAwIiwiZW5kX3RpbWVzdGFtcCI6IjE3MDQxNTAwMDAwMDAwMDAwMDAifSwic2FsdCI6IjdiYzcwOWMyMjgwMTExOGIifX0=,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: IE9uIG9sZGVyIHRlcm1pbmFscywgdGhlIHVuZGVyc2NvcmUgY29kZSBpcyBkaXNwbGF5ZWQgYXMgYSBsZWZ0CiAgICAgICBhcnJvdywgY2FsbGVkIGJhY2thcnJvdy4K,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: ewABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4fICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj+AgYKDhIWGh4iJiouMjY6P,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310c0000000007800000006463376533346565636563"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions", "Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa33303936613461363631653130393332383334663830313134396334396462613962393333323266366439646531383034376639633162313162336233313637333033333933366164303762646463303166396461323764393734383131653438306662313937633739396532333438306134383906008cf7fde291819ba810ce4ec49dff5ade081df2c5c3af03260cfd2c81dc1d18ad060161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b040000e82982269b2408f50000000000000161dd29ada831ab894b465a656c86c5"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Action", "Next Action", "Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b050161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d00000001050161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab21080000000000000080000000646337653334656563656333303936613461363631653130393332383334"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Action", "Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057db6638303131343963343964626139623933333232663664396465313830343766396331623131623362333136373330333339333661643037626464633031663964613237643937343831316534383066623139376337393965323334383061343839060000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e65640d00000066726f6d5f726573696475616c0b00000066726f6d5f6f757470757414000000756e777261705f6572725f756e636865636b6564110000007472795f726573657276655f65786163"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800200575e74030000c071f0d12b84c31f000000000000011e0000002a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a021500000073617475726174696e675f6164645f7369676e6564c50000007b2276657374696e675f73"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Action", "Next Action", "Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057ba63686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a2237626337303963323238303131313862227d7d"
        ),
        bytes.fromhex(
            "8002005706c9f05d991d00"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800200579000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e656460000000204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772e0a"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002005787c9f05d991d0000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e6564510000007b000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f808182838485868788898a8b8c8d8e8f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Action"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805718c9f05d991d0000000000c071f0d12b84c31f000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "f6e4ce8d6cd190984080892a9c49d5d3c0daff536c7b3a87f43f3522e42ffa88bbb74b287e3ee77249caf775020af54f87c442e70d6cc104864537a62e4d1408"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
