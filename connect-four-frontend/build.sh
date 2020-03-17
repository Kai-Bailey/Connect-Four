#!/bin/bash

set -e
set -u

ROOT=./target/wasm32-unknown-unknown/debug/
DEST=../connect-four/public/webassembly/

echo 'Building WebAssembly Files...'
cargo web build --target=wasm32-unknown-unknown

echo ''
echo 'Removing Old WebAssembly Files...'
if [ -f ${ROOT}connect-four-frontend.js ]; then
    rm ${ROOT}connect-four-frontend.js;
fi
if [ -f ${ROOT}connect-four-frontend.wasm ]; then
    rm ${ROOT}connect-four-frontend.wasm
fi

echo ''
echo 'Copying WebAssembly Files...'
cp ${ROOT}connect-four-frontend.js ${DEST}
cp ${ROOT}connect-four-frontend.wasm ${DEST}

echo ''
echo 'Finished Successfully'
