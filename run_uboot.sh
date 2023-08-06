#! /usr/bin/env zsh

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -bios u-boot/u-boot.bin
