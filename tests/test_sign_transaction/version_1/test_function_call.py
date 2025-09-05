from application_client.client import (
    AsyncAPDU,
    SW_OK,
    NavigableConditions,
    Nearbackend,
    generic_test_sign,
)
from ragger.backend.interface import RAPDU
from ragger.navigator import Navigator


def test_sign_function_call_string(firmware, backend, navigator: Navigator, test_name):
    """
    Args String:
    {"previous_vesting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577\
919600000000000","cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000\
000000"},"salt":"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}, "v\
esting_schedule_with_salt":{"vesting_schedule":{"start_timestamp":"1577919600000000000"\
,"cliff_timestamp":"1609455600000000000","end_timestamp":"1704150000000000000"},"salt":\
"7bc709c22801118b743fae3866edb4dea1630a97ab9cd67e993428b94a0f397a"}}    

    TransactionV1 {
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
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: eyJwcmV2aW91c192ZXN0aW5nX3NjaGVkdWxlX3dpdGhfc2FsdCI6eyJ2ZXN0a\
W5nX3NjaGVkdWxlIjp7InN0YXJ0X3RpbWVzdGFtcCI6IjE1Nzc5MTk2MDAwMDAwMDAwMDAiLCJjbGlmZl90aW1l\
c3RhbXAiOiIxNjA5NDU1NjAwMDAwMDAwMDAwIiwiZW5kX3RpbWVzdGFtcCI6IjE3MDQxNTAwMDAwMDAwMDAwMDA\
ifSwic2FsdCI6IjdiYzcwOWMyMjgwMTExOGI3NDNmYWUzODY2ZWRiNGRlYTE2MzBhOTdhYjljZDY3ZTk5MzQyOG\
I5NGEwZjM5N2EifSwgInZlc3Rpbmdfc2NoZWR1bGVfd2l0aF9zYWx0Ijp7InZlc3Rpbmdfc2NoZWR1bGUiOnsic\
3RhcnRfdGltZXN0YW1wIjoiMTU3NzkxOTYwMDAwMDAwMDAwMCIsImNsaWZmX3RpbWVzdGFtcCI6IjE2MDk0NTU2\
MDAwMDAwMDAwMDAiLCJlbmRfdGltZXN0YW1wIjoiMTcwNDE1MDAwMDAwMDAwMDAwMCJ9LCJzYWx0IjoiN2JjNzA\
5YzIyODAxMTE4Yjc0M2ZhZTM4NjZlZGI0ZGVhMTYzMGE5N2FiOWNkNjdlOTkzNDI4Yjk0YTBmMzk3YSJ9fQ==,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
         priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021500000073617475726174696e675f"
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
            "80020057fa6164645f7369676e6564f30100007b2270726576696f75735f76657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a2237626337303963323238303131313862373433666165333836366564623464656131363330613937616239636436376539"
        ),
        bytes.fromhex(
            "80020057fa393334323862393461306633393761227d2c202276657374696e675f7363686564756c655f776974685f73616c74223a7b2276657374696e675f7363686564756c65223a7b2273746172745f74696d657374616d70223a2231353737393139363030303030303030303030222c22636c6966665f74696d657374616d70223a2231363039343535363030303030303030303030222c22656e645f74696d657374616d70223a2231373034313530303030303030303030303030227d2c2273616c74223a22376263373039633232383031313138623734336661653338363665646234646561313633306139376162396364363765393933343238"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "800280572d62393461306633393761227d7dc9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "fdd62160b65403f731687e550e4fd6db2f8b64748a4da429d7745b2b701e80807b0aa624e0fea18a2973f93fae55bb0a1938f7e116f3e91bf1456846a15b6a02"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_string_with_newline(
    firmware, backend, navigator: Navigator, test_name
):
    """
    Args String:
    "{\"test_key\": \"value\nhidden part of value 1 2 3 4 5 6 7 8 9\"}"

    TransactionV1 {
        signer_id: AccountId(
            "1b11b3b31673033936ad07bddc01f9da27d974811e480fb197c799e23480a489",
        ),
        public_key: ed25519:2pffV8fXgXUyuKdTS8Nqnvbkua16wEtTv8tdWZCpVtat,
        nonce: 103595482000005,
        receiver_id: AccountId(
            "dc7e34eecec3096a4a661e10932834f801149c49dba9b93322f6d9de18047f9c",
        ),
        block_hash: Cb3vKNiF3MUuVoqfjuEFCgSNPT79pbuVfXXd2RxDXc5E,
        actions: [
            FunctionCall(
                FunctionCallAction {
                    method_name: test_payload_with_newline,
                    args: eyJ0ZXN0X2tleSI6ICJ2YWx1ZQpoaWRkZW4gcGFydCBvZiB2YWx1ZSAxIDIgMyA0IDUgNiA3IDggOSJ9,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000219000000746573745f7061796c6f61"
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
                "800280576e645f776974685f6e65776c696e653c0000007b22746573745f6b6579223a202276616c75650a68696464656e2070617274206f662076616c7565203120322033203420352036203720382039227dc9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "ea8daedb6a7f330718303721f0a3843dee62282a60dcd6ea5df03a94a40d193544a826bc85f5bfcd52bf5e8755e7565ea0a7c7d22ab44a469a609e4f47530f0c"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_string_ascii_subrange(
    firmware, backend, navigator: Navigator, test_name
):
    r"""
    Args String:
    "{\" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}\"}"

    TransactionV1 {
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
            FunctionCall(
                FunctionCallAction {
                    method_name: test_payload_str_with_ascii_subrange,
                    args: eyIgISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn8ifQ==,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a725831010000000224000000746573745f7061796c6f61"
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
                "80028057a1645f7374725f776974685f61736369695f73756272616e6765640000007b22202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f227dc9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "b0acb312d884ce4e2fcab87acb205c6c2506c5f6f306a9db6b90bc400275e1a2186a18735df41c819e10548dc0f3c361a5376927e1da76ddc016f6a12e2e1405"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_string_with_multibyte_utf8(
    firmware, backend, navigator: Navigator, test_name
):
    r"""
    Args String:
    "{\"test_utf8_key\": \"2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©3ଔ4🝙2©\"}"

    TransactionV1 {
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
            FunctionCall(
                FunctionCallAction {
                    method_name: test_payload_with_utf8_text,
                    args: eyJ0ZXN0X3V0Zjhfa2V5IjogIjLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqTPgrJQ08J+dmTLCqSJ9,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021b000000746573745f7061796c6f61"
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
            "80020057fa645f776974685f757466385f74657874a40100007b22746573745f757466385f6b6579223a202232c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac94"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "80028057de34f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a933e0ac9434f09f9d9932c2a9227dc9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "e212462feb6a814aec3bfa83ba65bdcd12a52110bbf233cec8404f763566722322ff312b1dde1e9b15db262e64fd34ac9686797614e7f20ee83928351c382f08"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_binary_hexdump(
    firmware, backend, navigator: Navigator, test_name
):
    """
    Args Binary: bytes.fromhex("204f6e206f6c646572207465726d696e616c732c2074686520756e\
64657273636f726520636f646520697320646973706c617965642061732061206c6566740a2020202020202\
06172726f772c2063616c6c6564206261636b6172726f772c2074686520636172657420697320646973706c\
6179656420617320616e2075702d6172726f770a20202020202020616e642074686520766572746963616c2\
062617220686173206120686f6c6520696e20746865206d6964646c652e0a0a202020202020205570706572\
6361736520616e64206c6f77657263617365206368617261637465727320646966666572206279206a75737\
4206f6e652062697420616e64207468650a2020202020202041534349492063686172616374657220322064\
6966666572732066726f6d2074686520646f75626c652071756f7465206279206a757374206f6e652062697\
42c0a20202020202020746f6f2e202054686174206d616465206974206d7563682065617369657220746f20\
656e636f64652063686172616374657273206d656368616e6963616c6c790a202020202020206f722077697\
4682061206e6f6e2d6d6963726f636f6e74726f6c6c65722d626173656420656c656374726f6e6963206b65\
79626f61726420616e6420746861740a2020202020202070616972696e672077617320666f756e64206f6e2\
06f6c642074656c6574797065732e0a")

    TransactionV1 {
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
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: IE9uIG9sZGVyIHRlcm1pbmFscywgdGhlIHVuZGVyc2NvcmUgY29kZSBpcyBka\
XNwbGF5ZWQgYXMgYSBsZWZ0CiAgICAgICBhcnJvdywgY2FsbGVkIGJhY2thcnJvdywgdGhlIGNhcmV0IGlzIGRp\
c3BsYXllZCBhcyBhbiB1cC1hcnJvdwogICAgICAgYW5kIHRoZSB2ZXJ0aWNhbCBiYXIgaGFzIGEgaG9sZSBpbiB\
0aGUgbWlkZGxlLgoKICAgICAgIFVwcGVyY2FzZSBhbmQgbG93ZXJjYXNlIGNoYXJhY3RlcnMgZGlmZmVyIGJ5IG\
p1c3Qgb25lIGJpdCBhbmQgdGhlCiAgICAgICBBU0NJSSBjaGFyYWN0ZXIgMiBkaWZmZXJzIGZyb20gdGhlIGRvd\
WJsZSBxdW90ZSBieSBqdXN0IG9uZSBiaXQsCiAgICAgICB0b28uICBUaGF0IG1hZGUgaXQgbXVjaCBlYXNpZXIg\
dG8gZW5jb2RlIGNoYXJhY3RlcnMgbWVjaGFuaWNhbGx5CiAgICAgICBvciB3aXRoIGEgbm9uLW1pY3JvY29udHJ\
vbGxlci1iYXNlZCBlbGVjdHJvbmljIGtleWJvYXJkIGFuZCB0aGF0CiAgICAgICBwYWlyaW5nIHdhcyBmb3VuZC\
BvbiBvbGQgdGVsZXR5cGVzLgo=,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021500000073617475726174696e675f"
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
            "80020057fa6164645f7369676e656409020000204f6e206f6c646572207465726d696e616c732c2074686520756e64657273636f726520636f646520697320646973706c617965642061732061206c6566740a202020202020206172726f772c2063616c6c6564206261636b6172726f772c2074686520636172657420697320646973706c6179656420617320616e2075702d6172726f770a20202020202020616e642074686520766572746963616c2062617220686173206120686f6c6520696e20746865206d6964646c652e0a0a2020202020202055707065726361736520616e64206c6f776572636173652063686172616374657273206469666665"
        ),
        bytes.fromhex(
            "80020057fa72206279206a757374206f6e652062697420616e64207468650a20202020202020415343494920636861726163746572203220646966666572732066726f6d2074686520646f75626c652071756f7465206279206a757374206f6e65206269742c0a20202020202020746f6f2e202054686174206d616465206974206d7563682065617369657220746f20656e636f64652063686172616374657273206d656368616e6963616c6c790a202020202020206f7220776974682061206e6f6e2d6d6963726f636f6e74726f6c6c65722d626173656420656c656374726f6e6963206b6579626f61726420616e6420746861740a2020202020202070"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805743616972696e672077617320666f756e64206f6e206f6c642074656c6574797065732e0ac9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "4ee0806396575f4366996446ae98625c8b7cd7dc8af7d4c7609fda4e045f2644e42ef296d6b14cf015ae01632f71e0331bfec574f84f7944704c3d20e5859709"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)


def test_sign_function_call_binary_hexdump_after_utf8_error(
    firmware, backend, navigator: Navigator, test_name
):
    """
    Args Binary: bytes.fromhex("7b000102030405060708090a0b0c0d0e0f101112131415161718191\
a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445\
464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707\
172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c\
9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c\
8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3\
f4f5f6f7f8f9fafbfcfdfe")

    TransactionV1 {
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
            FunctionCall(
                FunctionCallAction {
                    method_name: saturating_add_signed,
                    args: ewABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4fICEiIyQlJicoKSorL\
C0uLzAxMjM0NTY3ODk6Ozw9Pj9AQUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVpbXF1eX2BhYmNkZWZnaGlqa2xt\
bm9wcXJzdHV2d3h5ent8fX5/gIGCg4SFhoeIiYqLjI2Oj5CRkpOUlZaXmJmam5ydnp+goaKjpKWmp6ipqqusra6\
vsLGys7S1tre4ubq7vL2+v8DBwsPExcbHyMnKy8zNzs/Q0dLT1NXW19jZ2tvc3d7f4OHi4+Tl5ufo6err7O3u7/\
Dx8vP09fb3+Pn6+/z9/g==,
                    gas: 127127122121,
                    deposit: 150000000000000000000000,
                },
            ),
        ],
        priority_fee: 127127122121,
    }
    """
    client = Nearbackend(backend)
    chunks = [
        AsyncAPDU(
            data=bytes.fromhex(
                "80020057fa8000002c8000018d80000000800000008000000101400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000021500000073617475726174696e675f"
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
            "80020057fa6164645f7369676e6564000100007b000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9ea"
        ),
        AsyncAPDU(
            data=bytes.fromhex(
                "8002805734ebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfec9f05d991d0000000000c071f0d12b84c31f000000000000c9f05d991d000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["To transaction suffix", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "bbb3b17d85a286e11fdb9e44b6a3fd5f238664feab92475112a544dd0c4f7de4da017441c3098e535ab4d2cc2fc04ddbc7c76e43ce4ac14ae70ca06a034c9b0c"
                ),
            ),
        ),
    ]
    generic_test_sign(client, chunks, navigator, test_name, firmware)
