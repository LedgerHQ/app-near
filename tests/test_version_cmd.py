from pathlib import Path
from application_client.client import Nearbackend
import toml


# In this test we check that the get_version replies the right application version
def test_get_version_cmd(backend):
    # Use the app interface instead of raw interface
    client = Nearbackend(backend)
    # Send the get_version instruction to the app
    version = client.get_version().data
    assert len(version) == 3
    # Read version from Cargo.toml
    with open(Path(__file__).parent.parent / 'Cargo.toml', 'r') as f:
        config = toml.load(f)
        v = config['package']['version']
        major, minor, patch = v.split('.')
    assert (version[0], version[1], version[2]) == (int(major), int(minor), int(patch)) 