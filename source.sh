CROSS_COMPILE=aarch64-none-elf-
export CROSS_COMPILE
GCC_PATH=$(pwd)/tools/arm-gnu-toolchain-12.3.rel1-darwin-x86_64-aarch64-none-elf/bin

# if GCC_PATH is not in PATH, add it
if [[ ! ":$PATH:" == *":$GCC_PATH:"* ]]; then
    export PATH=$PATH:$GCC_PATH
else
    echo "arm gcc already in PATH"
fi
