#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)
pushd $SCRIPT_DIR

cargo build --release
mkdir -p bin
cp -f target/release/zabel.exe bin/zabel.exe
cp -rf resource bin/

popd
