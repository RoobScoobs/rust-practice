#!/bin/bash

WABT_BIN=$HOME/wasm-tools/wabt/bin
TARGET=wasm32-unknown-unknown
NAME=do_addition
BINARY=target/$TARGET/release/$NAME.wasm

cargo build --target $TARGET --release
$WABT_BIN/wasm-strip $BINARY