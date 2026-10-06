from application_client.client import (
    AsyncAPDU,
    SW_DENY,
    SW_OK,
    SW_TX_PARSING_FAIL,
    NavigableConditions,
    Nearbackend,
    generic_test_sign_single_review,
)
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_deposit_and_stake(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall deposit_and_stake on a staking pool.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: deposit_and_stake,
                    args: {},
                    gas: 125000000000000,
                    deposit: 2006231634787054162110590,
                },
            ),
        ],
    }

    Combined stake UI: first APDU (prefix) is silent, second APDU triggers
    "Review stake" / "Sign stake" combined review screen.

    NOTE: the expected signature bytes below are placeholders.  Run this test
    once against speculos with --snapshot-dir tests/snapshots to capture the
    real snapshots and signature:
        pytest tests/ -k test_sign_deposit_and_stake --snapshot-dir tests/snapshots
    Then update the bytes.fromhex() value in expected_response below with the
    actual signature returned by the device.
    """
    client = Nearbackend(backend)
    chunks = [
        # First APDU: BIP32 path + tx prefix + FunctionCall tag/method_name.
        # This stays silent but lets the app identify the staking flow early.
        bytes.fromhex(
            "80020057db8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002110000006465706f7369745f616e645f7374616b6502"
        ),
        # Second APDU: remaining FunctionCall fields — triggers combined review.
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af7100007e6855ec10c0eb08d6a8010000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("b3ad9e69236d9de996d08148b7d4684bbe01d56be1dbb3c4fcc501a0d9f44c213670cca41a1985c0d2f8e1a404fe04e53cf32aef0f0ba6b8a3508b53f4b48007"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake on a staking pool, with amount in JSON args.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 1,
                },
            ),
        ],
    }

    Combined unstake UI: first APDU (prefix) is silent, second APDU triggers
    "Review unstake" / "Sign unstake" combined review screen showing the
    unstake amount parsed from JSON args.

    NOTE: the expected signature bytes below are placeholders.  Run this test
    once against speculos to capture real snapshots and signature, then update.
    """
    client = Nearbackend(backend)
    chunks = [
        # First APDU: BIP32 path + tx prefix + FunctionCall tag/method_name.
        # This stays silent but lets the app identify the staking flow early.
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6526"
        ),
        # Second APDU: remaining FunctionCall fields — triggers combined review.
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000001000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("5652c4648eec17f182bd575a640d7955924243a6fb3f869326e4b0e3a24d99cf2bf60228721d5aa9bcdd43530a520673c56c831b3108f5875c62ef9522cdb70e"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_reject_deposit_and_stake(firmware, backend, navigator: Navigator, test_name):
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057db8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002110000006465706f7369745f616e645f7374616b6502"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af7100007e6855ec10c0eb08d6a8010000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Reject"],
            ),
            expected_response=RAPDU(SW_DENY, bytes()),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_reject_unstake(firmware, backend, navigator: Navigator, test_name):
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6526"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000001000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Reject"],
            ),
            expected_response=RAPDU(SW_DENY, bytes()),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)

def test_reject_unstake_decimal_amount(firmware, backend, navigator: Navigator, test_name):
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6527"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057420000007b22616d6f756e74223a22302e313830363233363535363639373437343339343634343935227d00d098d4af71000001000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Reject"],
            ),
            expected_response=RAPDU(SW_DENY, bytes()),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_with_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake with a nonzero attached deposit.

    The review shows the unstake amount from the JSON args and, as a separate
    field, the attached deposit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 1000000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6526"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af710000000000a1edccce1bc2d3000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("6656946982e3f9f08f6cb0b674bcb24925ef2db6e41d738b75a85246a3bc7b5ddceac403006b508375c2b162526b3a5ca050fd8f09c23606f1f1049d5d81860f"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_zero_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake with a zero attached deposit.

    The review shows the unstake amount from the JSON args and no attached
    deposit field.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6526"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("380be8e588e1e819026491728878f73f5cfa48714a348d3bd0f8cdaabc27aa681a46d65b7f5d5bb37c8da2bd60926da2d13f77f4114c26aefccd2dae3a5d600e"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_all_with_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake_all with a nonzero attached deposit.

    The review shows "All staked NEAR" and, as a separate field, the attached
    deposit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake_all,
                    args: {},
                    gas: 125000000000000,
                    deposit: 1000000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d58000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000020b000000756e7374616b655f616c6c02"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af710000000000a1edccce1bc2d3000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("b72c719830caafbc0b43e5bf5e98bb2b51820fa4b698465f2fa9cb1007843770f629cb2dc4f065cda5bed03192dc6b96627a553dbe32987c9f5d3a31e74dbd04"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_all_zero_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake_all with a zero attached deposit.

    The review shows "All staked NEAR" and no attached deposit field.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake_all,
                    args: {},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d58000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000020b000000756e7374616b655f616c6c02"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("f00ce8bc81ac7d8a8db8d21ece2980c99a36e57834f939e782636bb9b75f70a1c60e730db33ddc07dea83af4f56aa87cc03faed2a3f5acebea2814ab0d5b5607"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_stake_fn_call_with_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall stake with a nonzero attached deposit.

    The review shows the stake amount from the JSON args and, as a separate
    field, the attached deposit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: stake,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 1000000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057cf8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002050000007374616b6526"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af710000000000a1edccce1bc2d3000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("127a9700f9092a52d9164f97c78874899f524e68854cb103cdba7bcc7ca750b27d67ed27ef45d1099c090b53f2a1a0b0497174e3f8dae5207d0441fff4f1f903"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_withdraw_with_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall withdraw with a nonzero attached deposit.

    The review shows the amount from the JSON args and, as a separate field,
    the attached deposit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: withdraw,
                    args: {"amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 1000000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d28000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000208000000776974686472617726"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057410000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af710000000000a1edccce1bc2d3000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("71d26508363a3415a09855dc33ba15110e1d3b8246ca00e6a71cd52bc29d561700ad5ddc06af407d0c9e5d12daccc02f41ffe3e8666829e437adfe0ef04a1108"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_withdraw_all_with_deposit(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall withdraw_all with a nonzero attached deposit.

    The review shows "All staked NEAR" and, as a separate field, the attached
    deposit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: withdraw_all,
                    args: {},
                    gas: 125000000000000,
                    deposit: 1000000000000000000000000,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d68000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000020c00000077697468647261775f616c6c02"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af710000000000a1edccce1bc2d3000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("4d7e7b59faa2f4fcc66a751e9becfb3dd9e110a4569da15d0f6bd27a7077df01a99c9086e529c1d6593f76aa5056b2b3aa3c02906de1f089a497c26f408d9801"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_reject_function_call_overflowing_method_length(firmware, backend, navigator: Navigator, test_name):
    """
    Single-action FunctionCall whose method name length is 0xffffffff.

    The app must answer with a parsing error, without any review screen and
    without exiting, and keep serving commands afterwards.
    """
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    rapdu = backend.exchange_raw(
        bytes.fromhex(
            "80028057d08000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002ffffffff756e7374616b65"
        )
    )
    assert rapdu.status == SW_TX_PARSING_FAIL
    assert client.get_version().status == SW_OK


def test_sign_unstake_extra_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args carry a field besides `amount`.

    The args do not match the expected shape, so the review shows the method
    name and the args as signed instead of an unstake amount.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000","beneficiary_id":"attacker.near"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6547"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057620000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030222c2262656e65666963696172795f6964223a2261747461636b65722e6e656172227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("2e36516285233aaa92a286d7d2b023d67a74ca80dcddc3d43ddcf62e0da3b31e29b51f415f67de269beb3b731905783364e1a0f9d8970c7ae12e807d74c46004"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_duplicate_amount_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args repeat the `amount` key.

    The review shows the method name and the args as signed.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1","amount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6533"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280574e0000007b22616d6f756e74223a2231222c22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("16e410b6045b97e44dd1cd23510176e8cf740111e303ffddf602f468ff3f6ab931518ad8795f0afe1b5351bf48d6da9e4920ca7646f0704385babc5e3d55e804"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_all_with_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake_all whose args are not empty.

    The review shows the method name and the args as signed instead of
    "All staked NEAR".

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake_all,
                    args: {"amount":"1"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d58000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000020b000000756e7374616b655f616c6c0e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057290000007b22616d6f756e74223a2231227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("bca91e6833725efc795ed4e826eb6ad30871c0297133b3c7b208322b3471d8eb69b914e41a8f0a07b58e7fc1e3ada144cc5fcab7823fa73219c1f8f2bd3cbc03"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_long_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args exceed the 200-byte display capacity.

    The args cannot be decoded strictly, so the review shows them as signed,
    with the number of bytes that do not fit.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000","memo":"mmmmmmmmmmmmmm... (248 bytes),
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b65f8"
        ),
        bytes.fromhex(
            "80020057fa0000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030222c226d656d6f223a226d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d22"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057197d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("291f3141f3b688b20e12f10ba24175e8d313a8dfc42c40eb779bda1c9cb18096858939ebb62a7a9b9085bf0a22eb38ebbdfe784e2721a0ef531229440bd95101"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_binary_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args are not valid UTF-8.

    The review shows the method name and the args in hexadecimal.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: <binary> 7b22616d6f756e74223a223122fffe7d,
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6510"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280572b0000007b22616d6f756e74223a223122fffe7d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("3c3206317173f209db6384b9380391edae0bd4c2b17de7e226d9fa1352e2c869e2728ed3baba2036399eb62e8c9cb43c71917dfefc91907e5867bf8f1b250a0f"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_deposit_and_stake_with_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall deposit_and_stake whose args are not empty.

    deposit_and_stake takes no args, so the review shows the method name, the
    args as signed and the attached deposit, instead of a stake amount.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: deposit_and_stake,
                    args: {"amount":"1"},
                    gas: 125000000000000,
                    deposit: 2006231634787054162110590,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057db8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002110000006465706f7369745f616e645f7374616b650e"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057290000007b22616d6f756e74223a2231227d00d098d4af7100007e6855ec10c0eb08d6a8010000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("27db253b79f98e3bd5590284d5305002d31eedef77fbfa4c7eeebfb459ac3fa15f3016102321ab93a43ba5f46a6fa5c15bd98f1f3bb84547307cec7a3324ab0f"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_stake_extra_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall stake whose args carry a field besides `amount`.

    The review shows the method name and the args as signed instead of a
    stake amount.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: stake,
                    args: {"amount":"1157130000000000000000000","account_id":"pool.near"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057cf8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a7258310100000002050000007374616b653f"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280575a0000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030222c226163636f756e745f6964223a22706f6f6c2e6e656172227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("1cc8fa535cfff2bacb00aed3a7ebbec9c94044c9106229c70208b33793188c70adabe1219457d6d989e109d756ef4e744fe4873ae0489032fa3ce4a33813370e"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_trailing_data_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args have data after the JSON object.

    The review shows the method name and the args as signed.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"amount":"1157130000000000000000000"}{"amount":"1"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6534"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280574f0000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030227d7b22616d6f756e74223a2231227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("1639c2f2ad7b7a1cebbb0786a49a152de5c0e113393f7f0416a378ce9dc4379f954b24aefc6bb07f929bf398d58baa0de4dc2f4ac492332e6acfe00b8fa6250d"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_escaped_key_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake whose args spell the `amount` key with a JSON escape.

    The review shows the method name and the args as signed.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {"\\u0061mount":"1157130000000000000000000"},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b652b"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057460000007b225c75303036316d6f756e74223a2231313537313330303030303030303030303030303030303030227d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("1fac9cf2cbed7ef28027252cc2eaddeae4f77b3c7b52e1bdb40d04f625ea24ad2d5d76868ec977b22a14004fee92637bac6eed53b2b6b15ec6751330197efe00"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_empty_args_shown_raw(firmware, backend, navigator: Navigator, test_name):
    """
    FunctionCall unstake with empty JSON args.

    unstake needs an `amount`, so the review shows the method name and the
    args as signed instead of an unstake amount.

    Transaction {
        signer_id: AccountId(
            "c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f",
        ),
        public_key: ed25519:EFr6nRvgKKeteKoEH7hudt8UHYiu94Liq2yMM7x2AU9U,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "ledgerbyfigment.poolv1.near",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: unstake,
                    args: {},
                    gas: 125000000000000,
                    deposit: 0,
                },
            ),
        ],
    }
    """
    client = Nearbackend(backend)
    chunks = [
        bytes.fromhex(
            "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b6502"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280571d0000007b7d00d098d4af71000000000000000000000000000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                bytes.fromhex("8431e6d1f0d0c925efe387e7e973b198b61cb3464dcc023205c126759f6b8876194c6fbd0017f216d1a4f82f507b379eb2f67aca7aefa071b8b145e3ec89ad01"),
            ),
        ),
    ]
    generic_test_sign_single_review(client, chunks, navigator, test_name, firmware)
