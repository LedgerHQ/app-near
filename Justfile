cleanup:
	#!/usr/bin/env bash
	sudo rm -rf target
	sudo rm -rf build
	check=$(ls target 2>/dev/null | wc -l)
	if [[ $check -ne 0 ]];
	then 
		echo 'target not cleaned';
		exit 3;
	fi
	check=$(ls build 2>/dev/null | wc -l)
	if [[ $check -ne 0 ]];
	then
		echo 'build not cleaned';
		exit 3;
	fi

build_all:
	bash local_test_helper.sh -c build_all

rebuild_all: cleanup build_all 
		

test_all:
	bash local_test_helper.sh -c test_all
