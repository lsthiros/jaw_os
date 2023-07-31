CPU=cortex-a53

# If environment variable GCC_PATH is not set, use default path
ifeq ($(GCC_PATH),)
GCC_PATH=
endif

# Create a variable for the compiler, as, ld, and objcopy
CC=$(GCC_PATH)arm-none-eabi-gcc
AS=$(GCC_PATH)arm-none-eabi-as
LD=$(GCC_PATH)arm-none-eabi-ld
OBJCOPY=$(GCC_PATH)arm-none-eabi-objcopy

all: test.bin

startup.o: startup.s
	$(AS) -mcpu=$(CPU) -g startup.s -o startup.o

test.o: test.c
	$(CC) -c -mcpu=$(CPU) -g test.c -o test.o

test.elf: test.o startup.o
	$(LD) -T test.ld test.o startup.o -o test.elf

test.bin: test.elf
	$(OBJCOPY) -O binary test.elf test.bin

clean:
	rm -f test.o startup.o test.elf test.bin