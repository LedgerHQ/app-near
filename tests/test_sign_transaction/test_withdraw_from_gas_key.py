
from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_withdraw_from_gas_key(
    firmware, backend, navigator: Navigator, test_name
):
    """
    transaction length: 
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c"
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E
        actions: [
            WithdrawFromGasKey(
                WithdrawFromGasKeyAction(
                    public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
                    amount: NearToken::from_yoctonear(150000000000000000000),
                ),
            ),
        ],
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000d00c4f5941e81e071c2fd1dae2e71fd3d"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes()
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805721859d462484391d9a90bf219211dcbb320f00009814440dab210800000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "54631fdf4adb95e51c8623b3a6b88a50cb2ce1085b71a45d3414233d83efcc56b72bd7d2d51e88cf6f6545d5cc004c0b3adc9884464eaed614b10fb071d61b09"
                ),
            ),
        ),
    ]

    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_withdraw_from_gas_key_secp(
    firmware, backend, navigator: Navigator, test_name
):
    """
    transaction length: 
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c"
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E
        actions: [
            WithdrawFromGasKey(
                WithdrawFromGasKeyAction(
                    public_key: secp256k1:2xV3hzGShUE3X5jE9jmAyFC67GfgwAUo5FoBJ79Zh84Z5Ubdxy94Ka73EWwrFg5FbVYAvtdqJK77P6CAdyMkEnca,
                    amount: NearToken::from_yoctonear(150000000000000000000),
                ),
            ),
        ],
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000d0161dd29ada831ab894b465a656c86c5"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes()
            ),
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280574157c5008156da0909c4a281f5c8d9ee3de837534833badf7ad41a5e83071908af7d4f2ae835c9d9aceb48cfb47a4c96509b00009814440dab210800000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "ab5f0f3d9d8b5392e4197d33c68da8e3fc32fc546467d9efd5ed2f73f7e2752b91959b0801ae869965a37f5f32ad56cfcd9530d455d3ae8d12caff9b14bae20a"
                ),
            ),
        ),
    ]

    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_withdraw_from_gas_key_ml_dsa(
    firmware, backend, navigator: Navigator, test_name
):
    """
    transaction length: 
    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f"
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c"
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E
        actions: [
            WithdrawFromGasKey(
                WithdrawFromGasKeyAction(
                    public_key: ml-dsa-65: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,
                    amount: NearToken::from_yoctonear(150000000000000000000),
                ),
            ),
        ],
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000d025f784cf7f1a0ec5a8b3e6b4ffcd939"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes()
            ),
        ),
        bytes.fromhex(
            "80020057fa1085b5b9120cc24561a35d6114fe53f2d5a8678bd2279a7f2eab55c32b69315652322f100e1df9072dc994b566d1e77c46d3acb9c34890ed6445aca1d8c78e55aa2d76c8ecc6f043f53f20e657fee42867e808cde592b0aa4a3c78bc11da8cea8d7ec073fe875b6f9258ea962d081a0b2094d380e8d13ad2fe005af81f90a7e6ccbd82d1fdaa66a6e4f45e04144cf50c41b553f9a23e1c5d785991c3b72202dbe38980ca3eea30717f7a06b2da737dbd05646211a7ec53f7b83049907e64027bb6065f57e5c1601ddad8b4a7c1bd627f5f20ec994c6c35d5c34b5c0a927f8a728931a4a8fe547a0d82f40d8a16d3600f203e6b8961e6349728a4"
        ),
        bytes.fromhex(
            "80020057faab9dce40441b050ed70147aaae6d93381d5408a33f08141403cb49f279d1925d8add1dfedfff5c4b02e51dd0fbd437e907971570733ac66f35791c6d9e736908d01a860ba4351a8c90e2d587694824649f6df88bae5c1a984a495fcebcb61c3bcadee4a6a903b1978414b6f89fd69a378ab26be98bf6c733483eb738a1205a4d3b3f00c50afebe8818198bbc57bc1d90faed921fa8b204d392643e311b6b8749aaac9380a39aaed36a2859b92131f7291a248ce9e49d00fb1761e583b119ac8f9671e863d239adf3373eefc353fbfe9f315e90ab6614d2c5c02df1d169cf25e2e17283d127d1529d47c71e55f64b18f6bfb829a3ef7a38cba250"
        ),
        bytes.fromhex(
            "80020057fa1a71f1b3c82cb40722e741dc1183f06b2c4df32b4516bf43513aa003f5758dca01bbf77e81ac018f432a05d545af521a2ef71dcaf2d25a664b624097891b539fa2dc4212170882708e2a2d1bdd4128eee9d43b44f46842946a16eb346107dca96b81eae6c595a9a2bf2be3413029eda53d60cbb017a1e9f075d428387b527bad3ea2b0b2396934116d21ade586a841576667a00b02411a6c87ade89e98bc4b551955f7403a31e64d531bfa26564644bbae2df74216111e9d5792702a623edc7bfef00cb0a5074b3e36e91d74a1c73c6dca582097368289589a9aff216711b72ebc2ec1309b9365a8c69ea9e52fc093bd098ca0c537323004b89c"
        ),
        bytes.fromhex(
            "80020057fa5a12c7b3466e5224293d1bc40c46a197968d843c53e75fa453e1b952794758fc2cf9846fc9063f86a3a2a518dc4c06f432feae6dfd95e050a433388549bdcd4be85aad3f3459067d21c7acb61b83af3bf1b26783180011887a91b63b1b84cc43fc110fe2ed9abe4a95b5d7b05c5be70267b69c584e45a8a04ea6a316454ea6c378a9125d1b7905ae2c72dd1330724d50594529bc7436277c2d352dc9150a2d8186064d0e0d03e65a194645a9b93c3934a5d86b0aaaadc7dfcfea7980db694e647bbc3266540c14764ee090745a86d57cb6d918c14bc11546fc4ce3d82cd64452bb3d5abbb9ba76ce73fe05505ef811d8c980037fb7f61c516dd8"
        ),
        bytes.fromhex(
            "80020057fa0382b5abe58a0efad34816cf94ac25d18a23310facea75cff2a69b8719972a7bb52b76b1175e5e69ec28e0c5b81a02022bc93bd2fbd91db12a2cc31f6e13d3d6772d16dfbba001c3e95e95dd03e0720f09c63dd468b09c537ec1e96859203eebe3ec39496ecc1d05da3a061e65cea600dc77dc777d2ca04c17fdd82980495e0d73d63b12af87cfe4ea15dcf3e694bca5d2ffb4bfdfcea9aea96875ad55bde4786fcfb679cbe663848ef2708ba170afa57cc205d80481b4cf95d201c0a7852fab4c5e853db79ec31acc1c5182fbc3bad5aee663392278f38ab5fcfe2311506f7a5caca5bcdd1f335659d3febe4931480ddbcc5befb744253e264a"
        ),
        bytes.fromhex(
            "80020057fa9bf858b61ae52bd0da26bf0dd342ebb343ad3353e4697096672d0d6ccddb7e4763764f2ec9b28a9ff6f90b5986fdd1979178814e2e992b55dc1992c8bddbbf18cf914214665536e1259e5f1aa928a83c2a0144d85993f50adbe4d5450f19a933913f88e5accf193d42130350b73443f229158102eaa5a8cbd998075e14ec6617b8d522e13d35a0611045719df276d4a152b00b5a6e612d1b6d27c12f8ed7071c78e9351b98035797d4a8ca9720dd6dc3332e538da29ce08809f2563670120016e928ac797b5a8648d80c5a91a173d692a0d42def42d8e0848e9b7543cb625b4d02d9d772549947296241344a9f000a8b8841f84c96f2ac461cf6"
        ),
        bytes.fromhex(
            "80020057faa831f462e583ca58654567cea69ec7e55cd7567a49d36c5e6facccb92801f9be6c1b7dad9964ae6665ba93b4df0ee0c234c8ea35c8616089034d1fe52d9fd5a5e055e3cfcb545537a2331d966cfa34c8803bd11c436afcbcd27c08ba9f682433e02301adf6f86f50c6fe1e88cf91525cf42eb0125c5cf29c5630571f24ccedacd7ead63ddce266d04be02b78a10796d58c70c8f5de82b77bcec1b00ab4d36f6eb9af476ba7ae12c5459a3664af1c8873cc45ae11f0f1c5be9e6c41bd1a9e5e8384bf6fa05d9d7ba34dd911664a0f626ce4697da44ea17006a5e94c96e5ef6e3123d9dabeae58114063e363d6be57c5a9e69783378f27f8f1f20e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057cb53e9ae885520b7296b3614d70e605fde7c39fb7cf68f4d6551bea1653e0e61b9481022dd99efb053220a003f745d0168a7aafaab6ec2d864412f5919d7f35f58136785b6a800bc5497d41590b394bf6fbf6c75bcb086b2fe08d41e87cdbc7cc31ed621a88a3854dc7338c51a056df9f79af377589b3bd5da6e760a8f9d4b95a05931bf4c2995152a2b2fd4e5b218e9350959746d28c9e08364dd8b9e46712f8333c367e81039b34d6752e677c4751127c1976b4077ed25528293cd00009814440dab210800000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "ef87bfc3692553600d3c2191edbafe6c406a67e3f248837d7a2e8c2d534950d332c0038efb63dc39d4c618e55119d79ccec9f1583f0d6e5a86b1fdade691f10a"
                ),
            ),
        ),
    ]

    generic_test_sign(client, chunks, navigator, test_name, firmware)
