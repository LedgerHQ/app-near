from application_client.client import AsyncAPDU, SW_OK, NavigableConditions, Nearbackend
from ragger.backend.interface import RAPDU
from ragger.navigator import NavInsID, Navigator
from common import ROOT_SCREENSHOT_PATH


def test_sign_transfer(firmware, backend, navigator: Navigator, test_name):
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
            Transfer(
                TransferAction {
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
                "80028057fa8000002c8000018d800000008000000080000001400000006334663539343165383165303731633266643164616532653731666433643835396434363234383433393164396139306266323139323131646362623332306600c4f5941e81e071c2fd1dae2e71fd3d859d462484391d9a90bf219211dcbb320f85aae733385e00004000000064633765333465656365633330393661346136363165313039333238333466383031313439633439646261396239333332326636643964653138303437663963ac299ac1376e375cd39338d8b29225613ef947424b74a3207c1226863a72583101000000030000c071f0d12b84c31f000000000000"
            ),
            navigable_conditions=NavigableConditions(
                value=["Continue to actions", "Sign"],
            ),
            expected_response=RAPDU(
                SW_OK,
                # signature
                bytes.fromhex(
                    "f735265a9b6f2653d223705c79ab0354ee6606f6e1ccb44dff3cdea1c553fb62925d717c97d128e954403cd99aafb1108d4fe96cf425fcbb3d11a3ccc5da0108"
                ),
            ),
        )
    ]
    numbered_chunks = enumerate(client.sign_message_chunks(chunks))

    for index, chunk_event in numbered_chunks:
        if isinstance(chunk_event, NavigableConditions):
            for condition in chunk_event.value:
                condition_folder = test_name + "_" + str(index) + "_" + condition.lower().replace(" ", "_")
                navigator.navigate_until_text_and_compare(
                    NavInsID.RIGHT_CLICK,
                    [NavInsID.BOTH_CLICK],
                    condition,
                    ROOT_SCREENSHOT_PATH,
                    condition_folder,
                    screen_change_after_last_instruction=False,
                )
        elif isinstance(chunk_event, RAPDU):
            response = client.get_async_response()

            assert response.status == chunk_event.status
            assert response.data == chunk_event.data
