#!/bin/bash

rustc +nightly-2017-12-21-x86_64-unknown-linux-gnu \
    -C debuginfo=1 \
    --target wasm32-unknown-unknown -O double.rs \
    --crate-type=cdylib


