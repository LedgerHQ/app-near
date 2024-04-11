#!/bin/bash

function build_all() {
  # docker command to build
  cat <<"EOF" | docker run --rm -i --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath ./):/app" ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder:latest
    rm -rf target
    cargo ledger build nanos
    cargo ledger build nanosplus
    cargo ledger build nanox
    exit
EOF
}

function test() {
  
  docker run --rm -tdi --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath .):/app" --name app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest 
  docker exec -it -u 0  app-near-container bash -c ' [ -f ./tests/requirements.txt ] && pip install -r ./tests/requirements.txt'

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

  docker exec -it  app-near-container bash -c "pytest ./tests --tb=short $filter -v --device $1 $golden_suffix"
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
  docker run --rm -tdi --privileged -v "/dev/bus/usb:/dev/bus/usb" -v "$(realpath .):/app" --name app-near-container ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest 
  docker exec -it -u 0  app-near-container bash -c ' [ -f ./tests/requirements.txt ] && pip install -r ./tests/requirements.txt'
  docker exec -it  app-near-container bash -c "pytest ./tests --tb=short $filter -v --device nanos" || exit
  docker exec -it  app-near-container bash -c "pytest ./tests --tb=short $filter -v --device nanosp" || exit
  docker exec -it  app-near-container bash -c "pytest ./tests --tb=short $filter -v --device nanox" || exit
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
