# If tools directory exists, we've already ran so just exit

# Check to see if u-boot submodules have been initialized, and if not, initialize them
if [ ! -f ".gitmodules" ]; then
    echo "git submodules not initialized, initializing"
    git submodule init
    git submodule update
fi

if [ -d "tools" ]; then
    echo "tools directory already exists, exiting"
    exit 0
fi

# MacOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    ARM_COMPILER_PATH="https://developer.arm.com/-/media/Files/downloads/gnu/12.3.rel1/binrel/arm-gnu-toolchain-12.3.rel1-darwin-x86_64-aarch64-none-elf.tar.xz?rev=78193d7740294ebe8dbaa671bb5011b2&hash=E7F05DCCE90B0833CD1E1818AD6E3CF1D4FBFEDD"
    ARM_COMPILER_SHA256="a3b5795cbf6ad4c6beacbacd0d7b4b98c9ea8c6b91f40c9a40a20753e749712b"
# Linux
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    ARM_COMPILER_PATH="https://developer.arm.com/-/media/Files/downloads/gnu/12.3.rel1/binrel/arm-gnu-toolchain-12.3.rel1-x86_64-aarch64-none-elf.tar.xz?rev=a8bbb76353aa44a69ce6b11fd560142d&hash=8DC6C55310058C1594FD6EEFD60F0B2528265C64"
    ARM_COMPILER_SHA256="382c8c786285e415bc0ff4df463e101f76d6f69a894b03f132368147c37f0ba7"
# Windows
else
    echo "get lost"
    exit 1
fi

# ARM_COMPILER_PATH contains the filename of the archive, but then theres a ? and a bunch of other stuff
# Use sed to remove everything after the ? and use the last part of the path as the filename
ARM_COMPILER_ARCHIVE_FILENAME=$(echo $ARM_COMPILER_PATH | sed 's/\?.*//g' | sed 's/.*\///g')

# Make tools/tmp directory and then use curl to download the compiler but also use -L to follow redirects
mkdir -p tools/tmp
curl -L $ARM_COMPILER_PATH -o tools/tmp/$ARM_COMPILER_ARCHIVE_FILENAME

# Check the sha256sum of the downloaded file against ARM_COMPILER_SHA256
if [ $(shasum -a 256 tools/tmp/$ARM_COMPILER_ARCHIVE_FILENAME | awk '{print $1}') != $ARM_COMPILER_SHA256 ]; then
    echo "sha256sum of downloaded file does not match expected value"
    exit 1
fi

# Extract the archive
tar -xf tools/tmp/$ARM_COMPILER_ARCHIVE_FILENAME -C tools

# set ARM_COMPILER_BASE to the base directory of the compiler
ARM_COMPILER_BASE=tools/$(echo $ARM_COMPILER_ARCHIVE_FILENAME | sed 's/.tar.xz//g')

# Create tools/paths.sh and write the path to the compiler to it
echo "export ARM_COMPILER_BASE=$(pwd)/$ARM_COMPILER_BASE" > tools/paths.sh

# Cleanup tmp
rm -rf tools/tmp
