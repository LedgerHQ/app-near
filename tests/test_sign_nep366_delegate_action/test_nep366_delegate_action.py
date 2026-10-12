from application_client.client import (
    BLIND_SIGNING_AHEAD,
    BLIND_SIGNING_DISABLED,
    SW_BLIND_SIGNING_DISABLED,
    AsyncAPDU,
    SW_OK,
    SW_TX_PARSING_FAIL,
    NavigableConditions,
    Nearbackend,
    FINISH_STUB_APDU,
    enable_blind_signing,
    generic_test_sign,
)
from ragger.backend import RaisePolicy
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
                            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
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
                                    receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
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
                        args: eyJ2ZXN0aW5nX3NjaGVkdWxlX3dpdGhfc2FsdCI6eyJ2ZXN0aW5nX3NjaGVkdWxlIjp7InN0YXJ0X3RpbWVzdGFtcCI6IjE1Nzc5MTk2MDAwMDAwMDAwMDAiLCJjbGlmZl90aW1lc3RhbXAiOiIxNjA5NDU1NjAwMDAwMDAwMDAwIiwiZW5kX3RpbWVzdGFtcCI6IjE3MDQxNTAwMDAwMDAwMDAwMDAifSwic2FsdCI6IjdiYzcwOWMyMjgwMTExOGIifX0=,
                        gas: 127127122121,
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: IE9uIG9sZGVyIHRlcm1pbmFscywgdGhlIHVuZGVyc2NvcmUgY29kZSBpcyBkaXNwbGF5ZWQgYXMgYSBsZWZ0CiAgICAgICBhcnJvdywgY2FsbGVkIGJhY2thcnJvdy4K,
                        gas: 127127122121,
                        deposit: 150000000000000000000000,
                    },
                ),
            ),
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: ewABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4fICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj+AgYKDhIWGh4iJiouMjY6P,
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
                "80080057ba8000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e6561720c0000000007400000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396306008cf7fde291819ba810ce4ec49dff5ade081df2c5c3af03260cfd2c81dc1d18ad060161dd29ada831ab894b465a656c86c557c5008156da0909c4a281f5c8d9ee"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions", "Next Subaction", "Next Subaction", "Next Subaction"],
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
            "80080057bacfb47a4c96509b57045e991d000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963060000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e65640d00000066726f6d5f726573696475616c0b00000066726f6d5f6f757470757414000000756e7772"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800800573f61705f6572725f756e636865636b6564110000007472795f726573657276655f6578616374030000c071f0d12b84c31f000000000000011e0000002a2a2a2a"
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
                "80080057fa2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a021500000073617475726174696e675f6164645f7369676e6564c50000007b2276657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a2237626337303963323238303131313862"
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
            "8008005703227d7d"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8008005796c9f05d991d0000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e656460000000204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772e0a"
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
                "8008005772c9f05d991d0000000000c071f0d12b84c31f000000000000021500000073617475726174696e675f6164645f7369676e6564510000007b000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a"
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
                "800800574b3b3c3d3e3f808182838485868788898a8b8c8d8e8fc9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71"
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
            data=bytes.fromhex(
                "8008805713fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "d94a1288bcb50a753d6695949873e47a5ab7a17a2121940176d2cd3763d6d678c644f0eb6cbf76fe22e58924b41aa35ae35b9ce669bbe5f23d24fcf9fee59906"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_delegate_action_method_name_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    The 51-byte method name exceeds what the review can display: the delegate
    action is rejected with a parsing error before the subaction review.

    DelegateAction {
        sender_id: AccountId(
            "bob.near",
        ),
        receiver_id: AccountId(
            "alice.near",
        ),
        actions: [
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab,
                        args: {"x":1},
                        gas: 125000000000000,
                        deposit: 0,
                    },
                ),
            ),
        ],
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057be8000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e656172010000000233000000616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616162070000007b2278223a317d00d098d4af71000000000000000000000000000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions"],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_delegate_action_method_name_too_long_other_suffix(firmware, backend, navigator: Navigator, test_name):
    """
    Same as test_reject_delegate_action_method_name_too_long with a method name
    that differs only after the first 50 bytes: rejected too.

    DelegateAction {
        sender_id: AccountId(
            "bob.near",
        ),
        receiver_id: AccountId(
            "alice.near",
        ),
        actions: [
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac,
                        args: {"x":1},
                        gas: 125000000000000,
                        deposit: 0,
                    },
                ),
            ),
        ],
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057be8000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e656172010000000233000000616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616163070000007b2278223a317d00d098d4af71000000000000000000000000000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions"],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_delegate_action_method_name_50_bytes(firmware, backend, navigator: Navigator, test_name):
    """
    Delegate action with a FunctionCall whose method name is exactly 50 bytes:
    the whole name is shown and the delegate action is signed.

    DelegateAction {
        sender_id: AccountId(
            "bob.near",
        ),
        receiver_id: AccountId(
            "alice.near",
        ),
        actions: [
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                        args: {"x":1},
                        gas: 125000000000000,
                        deposit: 0,
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
                "80088057bd8000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e6561720100000002320000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161070000007b2278223a317d00d098d4af71000000000000000000000000000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions", "To NEP366 suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("b39259bebba4d23cbca456d8b384801c945f43b09f3b41d65441192bc5d71c0ebc1ee7d418e51d5201bf9546b6ba831503a51a60f6607364c233c803c43fd301"),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_delegate_action_add_key_method_name_too_long(firmware, backend, navigator: Navigator, test_name):
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
                AddKey(
                    AddKeyAction {
                        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
                        access_key: AccessKey {
                            nonce: 12345,
                            permission: FunctionCall(
                                FunctionCallPermission {
                                    allowance: Some(
                                        150000000000000000000,
                                    ),
                                    receiver_id: "alice.near",
                                    method_names: [
                                        "first_method",
                                        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaevil",
                                    ],
                                },
                            ),
                        },
                    },
                ),
            ),
        ],
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
    }

    The 44-byte method name exceeds what the review can display (40 bytes): the
    delegate action is rejected with a parsing error before the subaction review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057f18000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e656172010000000500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000000a000000616c6963652e6e656172020000000c00000066697273745f6d6574686f642c000000616161616161616161616161616161616161616161616161616161616161616161616161616161616576696cc9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions"],
            ),
            expected_response=RAPDU(
                SW_TX_PARSING_FAIL,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_delegate_action_sender_id_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    Delegate action whose sender_id is 65 bytes, one over the longest valid account ID
    ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.near").

    It is rejected with a parsing error before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057ad8000002c8000018d800000008000000080000001410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e6561720a000000616c6963652e6e65617201000000030000c071f0d12b84c31f000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
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


def test_reject_delegate_action_receiver_id_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    Delegate action whose receiver_id is 65 bytes, one over the longest valid account ID
    ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.near").

    It is rejected with a parsing error before any review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057ab8000002c8000018d80000000800000008000000108000000626f622e6e656172410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e65617201000000030000c071f0d12b84c31f000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
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


def test_reject_delegate_action_function_call_args_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    Delegate action with a FunctionCall subaction whose string args are 201 bytes, one over
    what the review can display ({"memo":"xxx...xxx"}).

    Blind signing is disabled by default, so it is rejected before the subaction review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057328000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e65617201000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80088057e7021500000073617475726174696e675f6164645f7369676e6564c90000007b226d656d6f223a2278787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878227d"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_DISABLED],
            ),
            expected_response=RAPDU(
                SW_BLIND_SIGNING_DISABLED,
                bytes(),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_delegate_action_function_call_args_too_long_blind_signing(
    firmware, backend, navigator: Navigator, test_name
):
    """
    Delegate action with a FunctionCall subaction whose string args are 201 bytes, one over
    what the review can display ({"memo":"xxx...xxx"}).

    With blind signing enabled, it is signed after the warning; the subaction review shows
    the first 200 bytes of the args and their size.

    DelegateAction {
        sender_id: AccountId(
            "bob.near",
        ),
        receiver_id: AccountId(
            "alice.near",
        ),
        actions: [
            NonDelegateAction(
                FunctionCall(
                    FunctionCallAction {
                        method_name: saturating_add_signed,
                        args: {"memo":"xxx...xxx"} (201 bytes),
                        gas: 125000000000000,
                        deposit: 0,
                    },
                ),
            ),
        ],
        nonce: 127127122121,
        max_block_height: 100500,
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
    }
    """
    enable_blind_signing(navigator, firmware)
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057328000002c8000018d80000000800000008000000108000000626f622e6e6561720a000000616c6963652e6e65617201000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Proceed to subactions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80080057e7021500000073617475726174696e675f6164645f7369676e6564c90000007b226d656d6f223a2278787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878787878227d"
            ),
            navigable_conditions=NavigableConditions(
                value=[BLIND_SIGNING_AHEAD],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800880574900d098d4af71000000000000000000000000000000000000c9f05d991d000000948801000000000000c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
            ),
            navigable_conditions=NavigableConditions(
                value=["To NEP366 suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex(
                    "c99c41379e936b206f760ca58d10eb0ddb2c585ca5ce64ca6732392d246790c88ee780a58084bb8d65cc91ade0bb2d9d039c5d2cb14c75ef5f1292ee96a68c0e"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
