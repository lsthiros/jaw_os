#! /usr/bin/env sh

# If KERNEL_DIR is not defined, use ./jaw_os/jaw_os/target/aarch64-unknown-none-softfloat/release/jaw_os
if [ -z "$KERNEL_DIR" ]; then
  KERNEL_DIR=./jaw_os/target/aarch64-unknown-none-softfloat/release/jaw_os
fi

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -kernel $KERNEL_DIR
