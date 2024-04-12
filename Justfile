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
