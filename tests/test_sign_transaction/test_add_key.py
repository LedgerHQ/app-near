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
            "80020057fa57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab210800000000000000800000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396331623131623362333136373330333339333661643037626464633031663964613237643937343831316534383066623139376337393965323334383061343839060000000c00000066697273745f6d6574686f641500000073617475726174696e675f6164645f7369676e"
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
                    "e8735cdfda4b287e4364508eb120831da083df3466c445799a0b6e1bc64500005c16dab5080104021e19f2f264de2d4296d5690755a9edd55d1e474d2a1b8f05"
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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
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
            "80020057fa57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab210800000000000000800000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396331623131623362333136373330333339333661643037626464633031663964613237643937343831316534383066623139376337393965323334383061343839020000000c00000066697273745f6d6574686f642c00000061616161616161616161616161616161616161"
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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
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
            "80020057fa57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab2108000000000000008000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963316231316233623331363733303333393336616430376264646330316639646132376439373438313165343830666231393763373939653233343830613438390b000000140000006d6574686f645f6e616d655f6e756d6265723030140000006d6574686f645f6e616d65"
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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
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
            "80020057e757c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab210800000000000000800000006463376533346565636563333039366134613636316531303933323833346638303131343963343964626139623933333232663664396465313830343766396331623131623362333136373330333339333661643037626464633031663964613237643937343831316534383066623139376337393965323334383061343839010000001100000066697273745f6d6574686f643b657669"
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
                                receiver_id: "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
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
            "80020057de57c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b57045e991d000000000100009814440dab21080000000000000080000000646337653334656563656333303936613461363631653130393332383334663830313134396334396462613962393333323266366439646531383034376639633162313162336233313637333033333933366164303762646463303166396461323764393734383131653438306662313937633739396532333438306134383901000000080000006dc3a974686f64"
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
