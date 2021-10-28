#!/bin/bash
set -eu

# ./configure.sh # <- call this first! (one time only)

FOLDER_NAME=${PWD##*/}
CRATE_NAME=$FOLDER_NAME # assume crate name is the same as the folder name
OUT_DIR="$(dirname $(dirname $0))"

# This is required to enable the web_sys clipboard API which egui_web uses
# https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Clipboard.html
# https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html
export RUSTFLAGS=--cfg=web_sys_unstable_apis

echo "Building rust…"
BUILD=release
cargo build --release -p ${CRATE_NAME} --lib --target wasm32-unknown-unknown

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME}.wasm"
wasm-bindgen "target/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}" \
  --out-dir $OUT_DIR --no-modules --no-typescript

# to get wasm-opt:  apt/brew/dnf install binaryen
#echo "Optimizing wasm…"
#wasm-opt $OUT_DIR/${CRATE_NAME}_bg.wasm -O2 --fast-math -o $OUT_DIR/${CRATE_NAME}_bg.wasm # add -g to get debug symbols

echo "Finished: $OUT_DIR/${CRATE_NAME}.wasm"
