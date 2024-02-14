#!/bin/sh

run_wazero(){
    wazero \
        run \
        ./target/wasm32-wasi/release-wasi/struct2csv.wasm
}

which wazero | fgrep -q wazero && run_wazero
