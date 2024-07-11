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
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_delegate_action_batch(firmware, backend, navigator: Navigator, test_name):
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
                CreateAccount(
                    CreateAccountAction,
                ),
            ),
            NonDelegateAction(
                DeleteAccount(
                    DeleteAccountAction {
                        beneficiary_id: AccountId(
                            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
                        ),
                    },
                ),
            ),
            NonDelegateAction(
                DeleteKey(
                    DeleteKeyAction {
                        public_key: ed25519:AVHQN9NRrHGeagz7RiVEUyhB9aiSGZCJbXKbJbW8z63E,
                    },
                ),
            ),
            NonDelegateAction(
                DeleteKey(
                    DeleteKeyAction {
                        public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    },
                ),
            ),
            NonDelegateAction(
                Stake(
                    StakeAction {
                        stake: 1157130000000000000000000,
                        public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    },
                ),
            ),
            NonDelegateAction(
                AddKey(
                    AddKeyAction {
                        public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                        access_key: AccessKey {
                            nonce: 127127127127,
                            permission: FullAccess,
                        },
                    },
                ),
            ),
            NonDelegateAction(
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
            ),
            NonDelegateAction(
                Transfer(
                    TransferAction {
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
            NonDelegateAction(
                DeployContract(
                    DeployContractAction {
                        code: KioqKioqKioqKioqKioqKioqKioqKioqKioqKioq,
                    },
                ),
            ),
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: eyJwcmV2aW91c192ZXN0aW5nX3NjaGVkdWxlX3dpdGhfc2FsdCI6eyJ2ZXN0aW5nX3NjaGVkdWxlIjp7InN0YXJ0X3RpbWVzdGFtcCI6IjE1Nzc5MTk2MDAwMDAwMDAwMDAiLCJjbGlmZl90aW1lc3RhbXAiOiIxNjA5NDU1NjAwMDAwMDAwMDAwIiwiZW5kX3RpbWVzdGFtcCI6IjE3MDQxNTAwMDAwMDAwMDAwMDAifSwic2FsdCI6IjdiYzcwOWMyMjgwMTExOGI3NDNmYWUzODY2ZWRiNGRlYTE2MzBhOTdhYjljZDY3ZTk5MzQyOGI5NGEwZjM5N2EifSwgInZlc3Rpbmdfc2NoZWR1bGVfd2l0aF9zYWx0Ijp7InZlc3Rpbmdfc2NoZWR1bGUiOnsic3RhcnRfdGltZXN0YW1wIjoiMTU3NzkxOTYwMDAwMDAwMDAwMCIsImNsaWZmX3RpbWVzdGFtcCI6IjE2MDk0NTU2MDAwMDAwMDAwMDAiLCJlbmRfdGltZXN0YW1wIjoiMTcwNDE1MDAwMDAwMDAwMDAwMCJ9LCJzYWx0IjoiN2JjNzA5YzIyODAxMTE4Yjc0M2ZhZTM4NjZlZGI0ZGVhMTYzMGE5N2FiOWNkNjdlOTkzNDI4Yjk0YTBmMzk3YWFiYWJhYiJ9fQ==,
                        gas: 127127122121,
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: IE9uIG9sZGVyIHRlcm1pbmFscywgdGhlIHVuZGVyc2NvcmUgY29kZSBpcyBkaXNwbGF5ZWQgYXMgYSBsZWZ0CiAgICAgICBhcnJvdywgY2FsbGVkIGJhY2thcnJvdywgdGhlIGNhcmV0IGlzIGRpc3BsYXllZCBhcyBhbiB1cC1hcnJvdwogICAgICAgYW5kIHRoZSB2ZXJ0aWNhbCBiYXIgaGFzIGEgaG9sZSBpbiB0aGUgbWlkZGxlLgoKICAgICAgIFVwcGVyY2FzZSBhbmQgbG93ZXJjYXNlIGNoYXJhY3RlcnMgZGlmZmVyIGJ5IGp1c3Qgb25lIGJpdCBhbmQgdGhlCiAgICAgICBBU0NJSSBjaGFyYWN0ZXIgMiBkaWZmZXJzIGZyb20gdGhlIGRvdWJsZSBxdW90ZSBieSBqdXN0IG9uZSBiaXQsCiAgICAgICB0b28uICBUaGF0IG1hZGUgaXQgbXVjaCBlYXNpZXIgdG8gZW5jb2RlIGNoYXJhY3RlcnMgbWVjaGFuaWNhbGx5CiAgICAgICBvciB3aXRoIGEgbm9uLW1pY3JvY29udHJvbGxlci1iYXNlZCBlbGVjdHJvbmljIGtleWJvYXJkIGFuZCB0aGF0CiAgICAgICBwYWlyaW5nIHdhcyBmb3VuZCBvbiBvbGQgdGVsZXR5cGVzLgo=,
                        gas: 127127122121,
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: ewABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4fICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj9AQUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVpbXF1eX2BhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5ent8fX5/gIGCg4SFhoeIiYqLjI2Oj5CRkpOUlZaXmJmam5ydnp+goaKjpKWmp6ipqqusra6vsLGys7S1tre4ubq7vL2+v8DBwsPExcbHyMnKy8zNzs/Q0dLT1NXW19jZ2tvc3d7f4OHi4+Tl5ufo6err7O3u7/Dx8vP09fb3+Pn6+/z9/g==,
                        gas: 127127122121,
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
                "80080057fa8000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e6561720c000000000780000000646337653334656563656333303936613461363631653130393332383334663830313134396334396462613962393333323266366439646531383034376639633162313162336233313637333033333933366164303762646463303166396461323764393734383131653438306662313937633739396532333438306134383906008cf7fde291819ba810ce4ec49dff5ade081df2c5c3af03260cfd2c81dc1d18ad060161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee"
            ),
            navigable_conditions=NavigableConditions(
                value=[
                    "Proceed to subactions",
                    "Next Subaction",
                    "Next Subaction",
                    "Next Subaction",
                ],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b040000e82982269b2408f50000000000000161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b050161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d00000001050161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Subaction", "Next Subaction", "Next Subaction"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80080057facfb47a4c96509b57045e991d000000000100009814440dab2108000000000000008000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963316231316233623331363733303333393336616430376264646330316639646132376439373438313165343830666231393763373939653233343830613438390a0000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e6564450000006974657261746f725f636861696e5f746f5f646f5f6d756c7469706c655f696e7374616e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa6365735f6f665f616e5f6f7065726174696f6e5f746861745f63616e5f6661696c0d00000066726f6d5f726573696475616c0b00000066726f6d5f6f757470757414000000756e777261705f6572725f756e636865636b6564110000007472795f726573657276655f65786163740c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e6564450000006974657261746f725f636861696e5f746f5f646f5f6d756c7469706c655f696e7374616e6365735f6f665f616e5f6f7065726174696f6e5f746861745f63616e5f6661696c030000c071f0d12b84c31f000000000000011e0000002a2a2a2a"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Subaction", "Next Subaction"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a021500000073617475726174696e675f6164645f7369676e6564f90100007b2270726576696f75735f76657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a2237626337303963"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Subaction"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80080057fa323238303131313862373433666165333836366564623464656131363330613937616239636436376539393334323862393461306633393761227d2c202276657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a22376263373039633232383031"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa31313862373433666165333836366564623464656131363330613937616239636436376539393334323862393461306633393761616261626162227d7dc9f05d991d0000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e656409020000204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772c2074686520636172657420697320646973706c6179656420617320616e2075702d6172726f770a20"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Subaction"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80080057fa202020202020616e642074686520766572746963616c2062617220686173206120686f6c6520696e20746865206d6964646c652e0a0a2020202020202055707065726361736520616e64206c6f77657263617365206368617261637465727320646966666572206279206a757374206f6e652062697420616e64207468650a20202020202020415343494920636861726163746572203220646966666572732066726f6d2074686520646f75626c652071756f7465206279206a757374206f6e65206269742c0a20202020202020746f6f2e202054686174206d616465206974206d7563682065617369657220746f20656e636f646520636861"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa72616374657273206d656368616e6963616c6c790a202020202020206f7220776974682061206e6f6e2d6d6963726f636f6e74726f6c6c65722d626173656420656c656374726f6e6963206b6579626f61726420616e6420746861740a2020202020202070616972696e672077617320666f756e64206f6e206f6c642074656c6574797065732e0ac9f05d991d0000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e6564000100007b000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a"
            ),
            navigable_conditions=NavigableConditions(
                value=["Next Subaction"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057fa3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfec9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71"
            ),
            navigable_conditions=NavigableConditions(
                value=["To NEP366 suffix"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex("8008805713fd3d859d462484391d9a90bf219211dcbb320f"),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "3f671b1d2ba42132e78c39dad35848ef0ee67858d85bd63cb1ce9e03d629c74cec9529add083aa0de6dbd45d372baa67bd8f8e49e76297a87cec1dc7084ae80d"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
