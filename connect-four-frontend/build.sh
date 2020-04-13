#!/bin/bash

set -e
set -u

ROOT=./target/wasm32-unknown-unknown/debug/
DEST=../connect-four-backend/public/

echo 'Building WebAssembly Files...'
cargo web build --target=wasm32-unknown-unknown

echo ''
echo 'Removing Old WebAssembly Files...'
if [ -f ${DEST}connect-four-frontend.js ]; then
    rm ${DEST}connect-four-frontend.js;
fi
if [ -f ${DEST}connect-four-frontend.wasm ]; then
    rm ${DEST}connect-four-frontend.wasm
fi

echo ''
echo 'Copying WebAssembly Files...'
sed 's/S_/\$/g' ${ROOT}connect-four-frontend.js > ${DEST}connect-four-frontend.js
cp ${ROOT}connect-four-frontend.wasm ${DEST}

echo ''
echo 'Finished Successfully!'
