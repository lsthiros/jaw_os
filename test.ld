ENTRY(_Reset)
SECTIONS
{
    . = 0x00;
    .text : {
        startup.o (INTERRUPT_VECTOR)
        *(.text)
    }
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }
    . = ALIGN(8);
    . = . + 0x1000; /* 4kB of stack */
    stack_top = .;
}