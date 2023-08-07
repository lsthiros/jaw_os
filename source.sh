CROSS_COMPILE=aarch64-none-elf-
export CROSS_COMPILE

# If tools/path.sh exists, source it, otherwise quit
if [ -f "tools/paths.sh" ]; then
    source tools/paths.sh
else
    echo "tools/paths.sh does not exist, run setup.sh"
    exit 1
fi

# If GCC_PATH is not set, set it
export GCC_PATH=$ARM_COMPILER_BASE/bin

# if GCC_PATH is not in PATH, add it
if [[ ! ":$PATH:" == *":$GCC_PATH:"* ]]; then
    export PATH=$PATH:$GCC_PATH
else
    echo "arm gcc already in PATH"
fi
