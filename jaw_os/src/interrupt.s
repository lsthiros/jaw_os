.section .text.exceptions

.globl exception_vector_table
exception_vector_table:

.equ CONTEXT_SIZE, 256

// IRQ vector
.org 0x0080
    sub sp, sp, #CONTEXT_SIZE

    // Store x0 to x29 on the stack
    stp x0, x1, [sp, #0x00]
    stp x2, x3, [sp, #0x10]
    stp x4, x5, [sp, #0x20]
    stp x6, x7, [sp, #0x30]
    stp x8, x9, [sp, #0x40]
    stp x10, x11, [sp, #0x50]
    stp x12, x13, [sp, #0x60]
    stp x14, x15, [sp, #0x70]
    stp x16, x17, [sp, #0x80]
    stp x18, x19, [sp, #0x90]
    stp x20, x21, [sp, #0xA0]
    stp x22, x23, [sp, #0xB0]
    stp x24, x25, [sp, #0xC0]
    stp x26, x27, [sp, #0xD0]
    stp x28, x29, [sp, #0xE0]

    // Store x30 (LR) on the stack
    str x30, [sp, #0xF0]

    // Store exception link register in x0, processor state in x1
    // and then store the pair on the stack
    mrs x0, elr_el1
    mrs x1, spsr_el1
    stp x0, x1, [sp, #0xF8]

    // Call the exception handler
    bl exception_handler

    // Restore x30 (LR) from the stack
    ldr x30, [sp, #0xF0]

    // Restore x0 to x29 from the stack
    ldp x0, x1, [sp, #0x00]
    ldp x2, x3, [sp, #0x10]
    ldp x4, x5, [sp, #0x20]
    ldp x6, x7, [sp, #0x30]
    ldp x8, x9, [sp, #0x40]
    ldp x10, x11, [sp, #0x50]
    ldp x12, x13, [sp, #0x60]
    ldp x14, x15, [sp, #0x70]
    ldp x16, x17, [sp, #0x80]
    ldp x18, x19, [sp, #0x90]
    ldp x20, x21, [sp, #0xA0]
    ldp x22, x23, [sp, #0xB0]
    ldp x24, x25, [sp, #0xC0]
    ldp x26, x27, [sp, #0xD0]
    ldp x28, x29, [sp, #0xE0]

    // Restore the exception link register and processor state
    ldp x0, x1, [sp, #0xF8]
    msr elr_el1, x0
    msr spsr_el1, x1

    // Restore the stack pointer
    add sp, sp, #CONTEXT_SIZE
    eret