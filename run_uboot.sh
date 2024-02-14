#! /usr/bin/env sh

# If KERNEL_DEBUG is defined, set QEMU_ARGS to -s -S and set RELEASE_DIR to debug
if [ -n "$KERNEL_DEBUG" ]; then
  QEMU_ARGS="-s -S"
  RELEASE_DIR=debug
else
  RELEASE_DIR=release
  QEMU_ARGS=""
fi

if [ -z "$KERNEL_DIR" ]; then
  KERNEL_DIR=./possum_os/target/aarch64-unknown-none-softfloat/${RELEASE_DIR}/possum_os
fi

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -machine gic-version=3,secure=off \
  -kernel $KERNEL_DIR \
  ${QEMU_ARGS}
