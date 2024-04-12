cleanup:
	#!/usr/bin/env bash
	sudo rm -rf target
	check=$(ls target 2>/dev/null | wc -l)
	if [[ $check -ne 0 ]];
	then 
		echo 'target not cleaned';
		exit 3;
	fi

build_all:
	#!/usr/bin/env bash
	bash local_test_helper.sh -c build_all

rebuild_all: cleanup build_all 
		

test_all:
	PYTEST_LOG_LEVEL=ERROR bash local_test_helper.sh -c test_all

test_all_info_log:
	PYTEST_LOG_LEVEL=INFO bash local_test_helper.sh -c test_all

pull_dev_images:
	docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder:latest
	docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest

run_speculos_nanos:
	docker run --rm -p 5000:5000 -p 5001:5001 -v '/dev/bus/usb:/dev/bus/usb'  \
	-v "$(realpath ./):/app" -it --name  \
	app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest \
	bash -c '/usr/bin/python3 -m speculos --model nanos --api-port 5000 --apdu-port 5001 --display headless /app/target/nanos/release/app-near-rust'

run_speculos_nanosplus:
	docker run --rm -p 5000:5000 -p 5001:5001 -v '/dev/bus/usb:/dev/bus/usb'  \
	-v "$(realpath ./):/app" -it --name  \
	app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest \
	bash -c '/usr/bin/python3 -m speculos --model nanosp --api-port 5000 --apdu-port 5001 --display headless /app/target/nanosplus/release/app-near-rust'

run_speculos_nanox:
	docker run --rm -p 5000:5000 -p 5001:5001 -v '/dev/bus/usb:/dev/bus/usb'  \
	-v "$(realpath ./):/app" -it --name  \
	app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest \
	bash -c '/usr/bin/python3 -m speculos --model nanox --api-port 5000 --apdu-port 5001 --display headless /app/target/nanox/release/app-near-rust'
