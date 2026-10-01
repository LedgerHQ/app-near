from application_client.client import (
    AsyncAPDU,
    SW_DENY,
    SW_OK,
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
