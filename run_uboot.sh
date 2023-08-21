#! /usr/bin/env sh

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -kernel ./jaw_os/target/aarch64-unknown-none-softfloat/release/jaw_os
