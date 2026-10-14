"""
FunctionCall args that the review cannot display in full are signed only with blind signing
enabled in the settings, after a warning; the review then shows the part of the args that fits
and their size. With the setting disabled (the default) they are rejected, see the
`test_reject_*_too_long` tests.
"""
from application_client.client import (
    BLIND_SIGNING_AHEAD,
    BLIND_SIGNING_BACK_TO_SAFETY,
    AsyncAPDU,
    SW_DENY,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    enable_blind_signing,
    generic_test_sign,
)
from ragger.backend import RaisePolicy
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator

# The FunctionCall of test_reject_function_call_string_args_too_long: 499-byte string args,
# method saturating_add_signed, gas 127127122121, deposit 0.15 NEAR.
STRING_ARGS_TOO_LONG = [
    "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021500000073617475726174696e675f61",
    "80020057fa64645f7369676e6564f30100007b2270726576696f75735f76657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a223762633730396332323830313131386237343366616533383636656462346465613136333061393761623963643637653939",
    "80020057fa3334323862393461306633393761227d2c202276657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a2237626337303963323238303131313862373433666165333836366564623464656131363330613937616239636436376539393334323862",
    "8002805724393461306633393761227d7dc9f05d991d0000000000c071f0d12b84c31f000000000000",
]

# The FunctionCall of test_reject_function_call_binary_args_too_long: binary args over the
# 100 bytes the review can display in hex.
BINARY_ARGS_TOO_LONG = [
    "80020057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021500000073617475726174696e675f61",
    "80020057fa64645f7369676e656409020000204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772c2074686520636172657420697320646973706c6179656420617320616e2075702d6172726f770a20202020202020616e642074686520766572746963616c2062617220686173206120686f6c6520696e20746865206d6964646c652e0a0a2020202020202055707065726361736520616e64206c6f77657263617365206368617261637465727320646966666572",
    "80020057fa206279206a757374206f6e652062697420616e64207468650a20202020202020415343494920636861726163746572203220646966666572732066726f6d2074686520646f75626c652071756f7465206279206a757374206f6e65206269742c0a20202020202020746f6f2e202054686174206d616465206974206d7563682065617369657220746f20656e636f64652063686172616374657273206d656368616e6963616c6c790a202020202020206f7220776974682061206e6f6e2d6d6963726f636f6e74726f6c6c65722d626173656420656c656374726f6e6963206b6579626f61726420616e6420746861740a202020202020207061",
    "800280573a6972696e672077617320666f756e64206f6e206f6c642074656c6574797065732e0ac9f05d991d0000000000c071f0d12b84c31f000000000000",
]

# The unstake of test_reject_unstake_long_args, on ledgerbyfigment.poolv1.near: 248-byte args.
UNSTAKE_ARGS_TOO_LONG = [
    "80020057d18000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00001b0000006c656467657262796669676d656e742e706f6f6c76312e6e656172ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000207000000756e7374616b65f8",
    "80020057fa0000007b22616d6f756e74223a2231313537313330303030303030303030303030303030303030222c226d656d6f223a226d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d6d22",
    "80028057197d00d098d4af71000000000000000000000000000000000000",
]


def multi_chunk(chunks, last_conditions, last_response):
    """The first chunk shows the transaction prefix, the last one the given screens."""
    return (
        [
            AsyncAPDU(
                data=bytes.fromhex(chunks[0]),
                navigable_conditions=NavigableConditions(value=["Continue to actions"]),
                expected_response=RAPDU(SW_OK, bytes()),
            )
        ]
        + [bytes.fromhex(chunk) for chunk in chunks[1:-1]]
        + [
            AsyncAPDU(
                data=bytes.fromhex(chunks[-1]),
                navigable_conditions=NavigableConditions(value=last_conditions),
                expected_response=last_response,
            )
        ]
    )


def test_sign_function_call_string_args_too_long_blind_signing(
    firmware, backend, navigator: Navigator, test_name
):
    """
    With blind signing enabled, the 499-byte string args are signed after the warning; the
    review shows their first 200 bytes and "499 bytes, first 200 shown".
    """
    enable_blind_signing(navigator, firmware)
    client = Nearbackend(backend)
    chunks = multi_chunk(
        STRING_ARGS_TOO_LONG,
        [BLIND_SIGNING_AHEAD, "Sign"],
        RAPDU(SW_OK, bytes.fromhex(
                    "e53a9694b09b0470fe72eb0531793d70ac2d8f0bd54e12d353a91a70d1413b534bfc28feb5bb78ec57a7e13600442d3ef55ee9d0fc72de1519f3e7edc0eb5306"
                )),
    )
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_binary_args_too_long_blind_signing(
    firmware, backend, navigator: Navigator, test_name
):
    """
    With blind signing enabled, binary args over the display limit are signed after the
    warning; the review shows their first 100 bytes in hex and their size.
    """
    enable_blind_signing(navigator, firmware)
    client = Nearbackend(backend)
    chunks = multi_chunk(
        BINARY_ARGS_TOO_LONG,
        [BLIND_SIGNING_AHEAD, "Sign"],
        RAPDU(SW_OK, bytes.fromhex(
                    "013d90aac5466bdbb1344648822b2fc74772669c37c53b1d2b4d7ffb46a9ce912ca6e15dda47afcc2d399eba5e68762ea740f157b0ed6fd9d10cc0bbbc991c0d"
                )),
    )
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_reject_function_call_args_too_long_back_to_safety(
    firmware, backend, navigator: Navigator, test_name
):
    """
    With blind signing enabled, turning back at the warning rejects the transaction.
    """
    enable_blind_signing(navigator, firmware)
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    client = Nearbackend(backend)
    chunks = multi_chunk(
        STRING_ARGS_TOO_LONG,
        [BLIND_SIGNING_BACK_TO_SAFETY],
        RAPDU(SW_DENY, bytes()),
    )
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_unstake_long_args_blind_signing(firmware, backend, navigator: Navigator, test_name):
    """
    With blind signing enabled, a staking-pool call whose args cannot be displayed in full is
    signed after the warning; the staking review shows the method name, the first 200 bytes
    of the args and their size.
    """
    enable_blind_signing(navigator, firmware)
    client = Nearbackend(backend)
    chunks = [bytes.fromhex(chunk) for chunk in UNSTAKE_ARGS_TOO_LONG[:-1]] + [
        AsyncAPDU(
            data=bytes.fromhex(UNSTAKE_ARGS_TOO_LONG[-1]),
            navigable_conditions=NavigableConditions(value=[BLIND_SIGNING_AHEAD, "Sign"]),
            expected_response=RAPDU(SW_OK, bytes.fromhex(
                    "291f3141f3b688b20e12f10ba24175e8d313a8dfc42c40eb779bda1c9cb18096858939ebb62a7a9b9085bf0a22eb38ebbdfe784e2721a0ef531229440bd95101"
                )),
        )
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
