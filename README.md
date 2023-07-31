# just another worthless Operating System
writing this while drunk idk. Want to write this in Rust or something. Its
going to support IPv6 fuck you Gerrik.

## Target
I'm deciding to target the qemu arm "virt" target for the A53. I think this
makes it aarch64 instead of arm but whatever. If I need to get set up again I
need to go to arm.com's website and download gcc-arm for aarch64-none-elf.

For the time being, I got it from [here](https://developer.arm.com/downloads/-/gnu-a)

And the qemu target is described [here](https://qemu.readthedocs.io/en/latest/system/arm/virt.html)

## Bootloader
I'm too stupid to write a bootloader so I'm using u-boot. First I gotta run
```bash
CROSS_COMPILE=aarch64-none-elf-
export CROSS_COMPILE
make qemu_arm64_defconfig
make
```