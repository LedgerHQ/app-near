#!/bin/bash

function check_feature() {
  cat Cargo.toml | grep -E 'default = \[.*"speculos"'
  return $?
}

check_feature

if [ $? = 0 ];
then
   echo DISABLE SPECULOS FEAT
   exit 1
else
   cargo ledger build -l "$1"
fi
