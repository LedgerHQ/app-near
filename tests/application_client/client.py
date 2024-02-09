from dataclasses import dataclass
from ragger.backend.interface import RAPDU, BackendInterface

CLA = 0x80
INS_GET_APP_CONFIGURATION = 0x06

# Parameter not used for this APDU
P1_P2_NOT_USED = 0x57


@dataclass
class Nearbackend:
    backend: BackendInterface

    def get_version(self) -> RAPDU:
        return self.backend.exchange(
            CLA, INS_GET_APP_CONFIGURATION, P1_P2_NOT_USED, P1_P2_NOT_USED, bytes()
        )
