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
import json
from enum import IntEnum


def test_sign_transfer(firmware, backend, navigator: Navigator, test_name):
    client = Nearbackend(backend)
    path: str = "m/44'/1'/0'/0/0"

    with client.sign_tx(path=path, transaction=json.dumps({}).encode('utf-8')):
        pass
