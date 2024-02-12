from typing import Generator, Optional, List, Union

from dataclasses import dataclass
from contextlib import contextmanager
from ragger.backend.interface import RAPDU, BackendInterface


CLA = 0x80
INS_GET_APP_CONFIGURATION = 0x06
INS_GET_PUBKEY = 0x04

# Parameter not used for this APDU
P1_P2_NOT_USED = 0x57
# Parameter 1 for screen confirmation for GET_PUBKEY.
P1_CONFIRM = 0x00

# Return codes
SW_OK = 0x9000

FINISH_STUB_APDU = RAPDU(0xFFFF, bytes())


@dataclass(frozen=True)
class AsyncAPDU:
    data: bytes
    expected_response: RAPDU


@dataclass
class Nearbackend:
    backend: BackendInterface

    def get_version(self) -> RAPDU:
        return self.backend.exchange(
            CLA, INS_GET_APP_CONFIGURATION, P1_P2_NOT_USED, P1_P2_NOT_USED, bytes()
        )

    @contextmanager
    def get_public_key_with_confirmation(self, path: bytes) -> Generator[None, None, None]:
        with self.backend.exchange_async(
            CLA, INS_GET_PUBKEY, P1_CONFIRM, P1_P2_NOT_USED, path
        ) as response:
            yield response

    def get_async_response(self) -> Optional[RAPDU]:
        return self.backend.last_async_response

    def sign_message_chunks(
        self, chunks: List[Union[bytes, AsyncAPDU]]
    ) -> Generator[RAPDU, None, RAPDU]:
        for chunk in chunks:
            if isinstance(chunk, AsyncAPDU):
                with self.backend.exchange_async_raw(chunk.data):
                    yield chunk.expected_response
            elif isinstance(chunk, bytes):
                rapdu = self.backend.exchange_raw(chunk)
                if rapdu.status != SW_OK:
                    return rapdu
            else:
                raise TypeError("bytes or AsyncAPDU expected")
        return FINISH_STUB_APDU
