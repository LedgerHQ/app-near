#!/usr/bin/env bash

function build_all() {
  # docker command to build
  cat <<"EOF" | docker run --rm -i --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath ./):/app" ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder:latest
    rm -rf target
    cargo ledger build nanos
    cargo ledger build nanosplus
    cargo ledger build nanox
    cargo ledger build stax
    cargo ledger build flex
    exit
EOF
}

function test() {
  if [[ -n "$GOLDEN" ]] ;
  then
    golden_suffix="--golden_run"
  else
    golden_suffix=""
  fi

  if [[ -n "$2" ]] ;
  then
    filter="-k $2"
  else
    filter=""
  fi

  docker run --rm -i --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath .):/app" --name app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest  /bin/bash -s <<EOF
  [ -f ./tests/requirements.txt ] && pip install -r ./tests/requirements.txt
  pytest ./tests --tb=short --log-cli-level=$PYTEST_LOG_LEVEL $filter -v --device $1 $golden_suffix
EOF
}

function test_all_nano() {
  echo $1

  if [[ -n "$1" ]] ;
  then
    filter="-k $1"
  else
    filter=""
  fi
  echo "$filter"
  # docker commands to test with Ragger (for ALL)

  docker run --rm -i --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath .):/app" --name app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest  /bin/bash -s <<EOF
  set -e
  [ -f ./tests/requirements.txt ] && pip install -r ./tests/requirements.txt
  pytest ./tests --log-cli-level=$PYTEST_LOG_LEVEL --tb=short $filter -v --device all_nano
EOF
}

function test_all() {
  echo $1
  
  if [[ -n "$1" ]] ;
  then 
    filter="-k $1"
  else
    filter=""
  fi
  echo "$filter"
  # docker commands to test with Ragger (for ALL)

  docker run --rm -i --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath .):/app" --name app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest  /bin/bash -s <<EOF
  set -e
  [ -f ./tests/requirements.txt ] && pip install -r ./tests/requirements.txt
  pytest ./tests --log-cli-level=$PYTEST_LOG_LEVEL --tb=short $filter -v --device all
EOF
}
while getopts ":c:t:f:g" opt; do
  case $opt in
    c) command="$OPTARG"
    ;;
    t) target="$OPTARG"
    ;;

    f) filter="$OPTARG"
    ;;

    g)
      echo "Golden run was triggered"
      export GOLDEN=run
    ;;

    \?) echo "Invalid option -$OPTARG" >&2
    exit 1
    ;;
  esac

  case $OPTARG in
    -*) echo "Option $opt needs a valid argument"
    exit 1
    ;;
  esac
done

case $command in

  build_all)
    build_all
    ;;

  test)
    test $target $filter
    ;;

  test_all)
    test_all $filter
    ;;

  *)
    echo -n "unknown command" $command
    exit 2
    ;;
esac
