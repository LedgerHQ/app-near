from application_client.client import (
    AsyncAPDU,
    SW_OK,
    SW_TX_PARSING_FAIL,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend import RaisePolicy
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
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


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
        bytes.fromhex(
            "80020057ba57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963060000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e"
        ),
        bytes.fromhex(
            "800200574e65640d00000066726f6d5f726573696475616c0b00000066726f6d5f6f757470757414000000756e777261705f6572725f756e636865636b6564110000007472795f726573657276655f65786163"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280570174"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "0c621b325f5ee71006234dd81babd8d837730632bbd00965d186dfcba136c7fb146d8fec67f9efc81fd6128432f36c8d6fb7f5f5d87e709ae04fba5538eb4203"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_add_key_method_name_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey FunctionCall permission whose 44-byte method name exceeds what the review can
    display (40 bytes): it is rejected with a parsing error before the action review.

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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [
                                    "first_method",
                                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaevil",
                                ],
                            },
                        ),
                    },
                },
            ),
        ],
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
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
        bytes.fromhex(
            "80020057ba57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963020000000c00000066697273745f6d6574686f642c00000061616161616161616161616161616161616161"
        ),
        bytes.fromhex(
            "8002005718616161616161616161616161616161616161616161657669"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057016c"
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


def test_reject_add_key_method_names_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey FunctionCall permission whose 11 method names take 231 bytes in the review,
    over its 210-byte capacity: it is rejected with a parsing error before the action
    review, instead of leaving methods out of the list.

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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [
                                    "method_name_number00",
                                    "method_name_number01",
                                    "method_name_number02",
                                    "method_name_number03",
                                    "method_name_number04",
                                    "method_name_number05",
                                    "method_name_number06",
                                    "method_name_number07",
                                    "method_name_number08",
                                    "method_name_number09",
                                    "method_name_number10",
                                ],
                            },
                        ),
                    },
                },
            ),
        ],
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
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
        bytes.fromhex(
            "80020057ba57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab21080000000000000040000000646337653334656563656333303936613461363631653130393332383334663830313134396334396462613962393333323266366439646531383034376639630b000000140000006d6574686f645f6e616d655f6e756d6265723030140000006d6574686f645f6e616d65"
        ),
        bytes.fromhex(
            "800200574e5f6e756d6265723031140000006d6574686f645f6e616d655f6e756d6265723032140000006d6574686f645f6e616d655f6e756d6265723033140000006d6574686f645f6e616d655f6e756d6265"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805793723034140000006d6574686f645f6e616d655f6e756d6265723035140000006d6574686f645f6e616d655f6e756d6265723036140000006d6574686f645f6e616d655f6e756d6265723037140000006d6574686f645f6e616d655f6e756d6265723038140000006d6574686f645f6e616d655f6e756d6265723039140000006d6574686f645f6e616d655f6e756d6265723130"
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


def test_reject_add_key_method_name_with_separator(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey FunctionCall permission whose method name contains ';', the separator of the
    reviewed list: it is rejected with a parsing error before the action review.

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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [
                                    "first_method;evil",
                                ],
                            },
                        ),
                    },
                },
            ),
        ],
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
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
        bytes.fromhex(
            "80020057a757c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963010000001100000066697273745f6d6574686f643b657669"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057016c"
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


def test_reject_add_key_method_name_not_ascii(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey FunctionCall permission whose method name is not printable ASCII, which the
    review cannot show unambiguously: it is rejected with a parsing error before the
    action review.

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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [
                                    "méthode",
                                ],
                            },
                        ),
                    },
                },
            ),
        ],
    }
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
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
        bytes.fromhex(
            "800200579e57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab210800000000000000400000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396301000000080000006dc3a974686f64"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280570165"
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


def test_sign_add_key_functioncall_any_method(firmware, backend, navigator: Navigator, test_name):
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
                    public_key: ed25519:3ARMH9zfVCnU2TKiphU4xcEyWdA45fc1sjKEtYMdf3gr,
                    access_key: AccessKey {
                        nonce: 0,
                        permission: FunctionCall(
                            FunctionCallPermission {
                                allowance: Some(
                                    150000000000000000000,
                                ),
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [],
                            },
                        ),
                    },
                },
            ),
        ],
    }

    An empty method list allows calling any method of the receiver: the review
    shows "Any method" instead of an empty method list.
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000500202122232425262728292a2b2c2d2e"
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
                "80028057732f303132333435363738393a3b3c3d3e3f0000000000000000000100009814440dab210800000000000000400000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396300000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "0d760b42814ca9919958ea2e5ea436e2be386b4a518e1a302d598152b2a756d4219301e09adab950ea109076925c74c9c50148380f015105eebdcf4315a4c702"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_add_key_functioncall_any_method_unlimited(firmware, backend, navigator: Navigator, test_name):
    """
    An empty method list with an unlimited allowance: the review shows
    "Any method" and the unlimited allowance.

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
                    public_key: ed25519:3ARMH9zfVCnU2TKiphU4xcEyWdA45fc1sjKEtYMdf3gr,
                    access_key: AccessKey {
                        nonce: 0,
                        permission: FunctionCall(
                            FunctionCallPermission {
                                allowance: None,
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
                                method_names: [],
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
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000500202122232425262728292a2b2c2d2e"
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
                "80028057632f303132333435363738393a3b3c3d3e3f00000000000000000000400000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396300000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("db9fdc260ca1356f91535443052dcc9c9d4a295f53e8d225d0bf2b77d4b5ef7564438ca1dd4e4bfcf135f659a5ea6bccb359f9238ec614e506682c47b4021d00"),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_add_key_receiver_id_too_long(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose receiver_id is 65 bytes, one over the longest
    valid account ID ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.near").

    The transaction is rejected with a parsing error before the action review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
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
                "80028057840500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f39300000000000000000410000006161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161612e6e656172010000000b0000006765745f62616c616e6365"
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


def test_sign_add_key_method_names_at_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose method names fill the review exactly:
    five 40-byte names (the longest accepted) and "ping", 210 bytes with separators.

    The full list is reviewed and signed.
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057e806000000280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303030280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303031280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303032280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303033280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f3030340400000070696e67"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "19fc81b1388afd5726d0e470b0dd3af56cef1042f986beb11faea611d81cbd14e8d1ae562c08a718939d52a3199c6de544499791490ec9ab7f3bf854359f1006"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_add_key_method_name_one_over_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose only method name is 41 bytes, one over the
    longest the review can display.

    The transaction is rejected with a parsing error before the action review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280573101000000290000006d6574686f645f6e616d655f65786163746c795f666f7274795f6f6e655f62797465735f6c6f6e675f"
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


def test_reject_add_key_method_names_one_over_display_limit(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose method names are each short enough but add up
    to 211 bytes with separators, one over the review capacity (five 40-byte names and "pings").

    The transaction is rejected with a parsing error before the action review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057e906000000280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303030280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303031280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303032280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f303033280000006d6574686f645f6e616d655f65786163746c795f666f7274795f62797465735f6c6f6e675f3030340500000070696e6773"
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


def test_reject_add_key_method_name_with_del_byte(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose method name contains a raw 0x7f byte
    ("get\x7fbalance"), which the review would show as the same glyph as any non-printable byte.

    The transaction is rejected with a parsing error before the action review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805713010000000b0000006765747f62616c616e6365"
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


def test_reject_add_key_method_name_with_control_byte(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission whose method name contains a control byte
    ("get\nbalance").

    The transaction is rejected with a parsing error before the action review.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805713010000000b0000006765740a62616c616e6365"
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


def test_sign_add_key_empty_method_name(firmware, backend, navigator: Navigator, test_name):
    """
    AddKey with a FunctionCall permission listing an empty method name before "get_balance".

    The empty name is reviewed as an empty entry (";get_balance;") and signed.
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057e98000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes(),
            ),
        ),
        bytes.fromhex(
            "80020057800500c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f3930000000000000000100009814440dab2108000000000000004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571702000000000000000b0000006765745f62616c616e6365"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "b2a6fd212db45604b5c7fd09beaa13d905811c54b841dc0b624ed83883ff03c0b46a45c7379539575d53f09d6be117eda1bca974d0afb5ba073620888ee1a209"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
