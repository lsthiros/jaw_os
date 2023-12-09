#! /usr/bin/env sh

if [ -z "$QEMU_BIN" ]; then
  QEMU_BIN=qemu-system-aarch64
fi

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

# Run QEMU for aarch64 with the gicv3 and no security extensions
MACHINE_ARGS=gic-version=3,secure=off

$QEMU_BIN                 \
  -machine virt           \
  -cpu cortex-a53         \
  -nographic              \
  -machine  $MACHINE_ARGS \
  -kernel $KERNEL_DIR     \
  ${QEMU_ARGS}
