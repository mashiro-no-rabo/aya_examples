#!/usr/bin/env bash

set -ex

(cd kprob && cargo rustc --release -- \
  -C linker-plugin-lto \
  -C linker-flavor=wasm-ld -C linker=bpf-linker \
  -C link-arg=--target=bpf)

(cd deployable && cargo build)

sudo deployable/target/debug/deployable
