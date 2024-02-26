set -xe
pushd fmt_buffer
cargo +stable test
popd
pushd near_token
cargo +stable test
popd
pushd near_gas
cargo +stable test
popd
