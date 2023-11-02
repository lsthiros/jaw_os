#! /usr/bin/env sh

# If KERNEL_DIR is not defined, use ./possum_os/possum_os/target/aarch64-unknown-none-softfloat/release/possum_os
if [ -z "$KERNEL_DIR" ]; then
  KERNEL_DIR=./possum_os/target/aarch64-unknown-none-softfloat/release/possum_os
fi

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -kernel $KERNEL_DIR
