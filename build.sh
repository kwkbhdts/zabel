#!/bin/bash

# Move to the directory of this script.
SCRIPT_DIR=$(cd $(dirname $0); pwd)
pushd $SCRIPT_DIR

cross build --target x86_64-pc-windows-gnu --release
mkdir -p bin
cp -f target/x86_64-pc-windows-gnu/release/zabel.exe bin/zabel.exe
cp -rf resource bin/

# Back to the saved directory.
popd
