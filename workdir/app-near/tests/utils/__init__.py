
from typing import Tuple
from pathlib import Path
import os
from mnemonic import Mnemonic

DEFAULT_SPECULOS_MNEMONIC = "glory promote mansion idle axis finger extra february uncover one " \
                            "trip resource lawn turtle enact monster seven myth punch hobby " \
                            "comfort wild raise skin"

DEFAULT_SETTINGS = {
    # mnemonic to use when running speculos
    "mnemonic": DEFAULT_SPECULOS_MNEMONIC,
    # path of the automation file to use for speculos if used, or None
    "automation_file": None,
    "sdk": "2.0",
    "model": "nanos"
}


def automation(filename):
    """Decorator that adds the automation_file attribute to a test function.

    When present, this filename will be used as the --automation file when creating the
    Speculos fixture.
    """
    def decorator(func):
        func.automation_file = filename
        return func
    return decorator


class SpeculosGlobals:
    def __init__(self, mnemonic: str, network: str = "test"):
        if network not in ["main", "test"]:
            raise ValueError(f"Invalid network: {network}")

        self.mnemonic = mnemonic
        self.seed = Mnemonic("english").to_seed(mnemonic)



makefile_relative_path = "../../Makefile"
makefile_path = (Path(os.path.dirname(os.path.realpath(__file__))) / Path(makefile_relative_path)).resolve()

def get_version_from_makefile() -> Tuple[int, int, int]:
    '''
    Parse the app Makefile to automatically
    '''
    APPVERSION_M = -1
    APPVERSION_N = -1
    APPVERSION_P = -1
    with open(makefile_path) as myfile:
        for line in myfile:
            if line.startswith("APPVERSION_M"):
                _, var = line.partition("=")[::2]
                APPVERSION_M = int(var.strip())
            if line.startswith("APPVERSION_N"):
                _, var = line.partition("=")[::2]
                APPVERSION_N = int(var.strip())
            if line.startswith("APPVERSION_P"):
                _, var = line.partition("=")[::2]
                APPVERSION_P = int(var.strip())

    return APPVERSION_M, APPVERSION_N, APPVERSION_P
