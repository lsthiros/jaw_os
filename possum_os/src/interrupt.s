.globl _exception_vector_table
.section ".text.exceptions"

.equ CONTEXT_SIZE, 264

.macro saveframe label
    // Store the corrptable registers on the stack
    stp x0, x1, [SP, #-16]!
    stp x2, x3, [SP, #-16]!
    stp x4, x5, [SP, #-16]!
    stp x6, x7, [SP, #-16]!
    stp x8, x9, [SP, #-16]!
    stp x10, x11, [SP, #-16]!
    stp x12, x13, [SP, #-16]!
    stp x14, x15, [SP, #-16]!

    bl \label

    // Unstack the corrptable registers
    ldp x14, x15, [SP], #16
    ldp x12, x13, [SP], #16
    ldp x10, x11, [SP], #16
    ldp x8, x9, [SP], #16
    ldp x6, x7, [SP], #16
    ldp x4, x5, [SP], #16
    ldp x2, x3, [SP], #16
    ldp x0, x1, [SP], #16
    eret
.endm


_exception_vector_table:
    saveframe _timer_interrupt

_irq_exception:
    .org 0x0080
    saveframe _timer_interrupt

_fiq_exception:
    .org 0x0100
    saveframe _timer_interrupt

_serror_exception:
    .org 0x0180
    saveframe _timer_interrupt

_syncronous_spx_exception:
    .org 0x0200
    saveframe _timer_interrupt

_irq_spx_exception:
    .org 0x0280
    saveframe _timer_interrupt

_fiq_spx_exception:
    .org 0x0300
    saveframe _timer_interrupt

_serror_spx_exception:
    .org 0x0380
    saveframe _timer_interrupt
