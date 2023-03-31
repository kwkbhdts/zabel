#!/bin/bash

# Move to the directory of this script.
SCRIPT_DIR=$(cd $(dirname $0); pwd)
pushd $SCRIPT_DIR

while getopts d OPT
do
  case $OPT in
    "d" ) DEBUG_FLAG=true ;;
  esac
done

mkdir -p bin

if "${DEBUG_FLAG}"; then
    cargo build
    cp -f target/debug/zabel.exe bin/zabel.exe
else
    cargo build --release
    cp -f target/release/zabel.exe bin/zabel.exe
fi

cp -rf resource bin/

# Back to the saved directory.
popd
