use core::arch::asm;
use core::arch::global_asm;

use crate::kprintf;

#[derive(Debug)]
pub enum ExceptionLevel {
    EL0 = 0,
    EL1 = 1,
    EL2 = 2,
    EL3 = 3,
}

#[repr(C)]
pub struct ExceptionContext {
    regs: [u64; 31],
    elr_el1: u64,
    spsr_el1: u64,
}

global_asm!(include_str!("interrupt.s"));

pub fn init_exception_table() {
    extern "C" {
        fn _exception_vector_table();
    }
    let exception_vector_table_offset: u64 = _exception_vector_table as usize as u64;
    unsafe {
        asm!(
            "msr VBAR_EL1, {0}",
            in(reg) exception_vector_table_offset,
        );
    }
}

pub fn configure_groups() {
    // Enable SRE bypass for EL1. If we don't do this, we'll get a synchronous exception when we try to write ICC_GRPEN1_EL1
    let sre_el1_contents: u64;
    unsafe {
        asm!(
            "mrs {0}, ICC_SRE_EL1",
            out(reg) sre_el1_contents,
        );
    }
    const SRE_BYPASS: u64 = 0b1;
    let sre_el1_desired: u64 = sre_el1_contents | SRE_BYPASS;
    unsafe {
        asm!(
            "msr ICC_SRE_EL1, {0}",
            in(reg) sre_el1_desired,
        );
    }

    const GROUP_ENABLE: u64 = 0b1;
    unsafe {
        asm!(
            "msr ICC_IGRPEN1_EL1, {0}",
            in(reg) GROUP_ENABLE,
        );
    }
}

#[no_mangle]
pub extern "C" fn _timer_interrupt(_ctx: &ExceptionContext) {
    kprintf!("Timer interrupt!\n");
    let delta: u64 = 100_000;
    let next: u64;
    unsafe {
        asm!(
            "msr CNTP_TVAL_EL0, {0}",
            "mrs {1}, CNTP_CVAL_EL0",
            in(reg) delta,
            out(reg) next,
        )
    }
    kprintf!("next: {:#x}\n", next);
    return;
}

// Function to get the current exception level
pub fn get_current_el() -> ExceptionLevel {
    let current_el_reg: u64;
    unsafe {
        asm!(
            "mrs {0}, CurrentEL",
            out(reg) current_el_reg,
        );
    }
    let current_el: u64 = (current_el_reg >> 2) & 0b11;
    match current_el {
        0 => ExceptionLevel::EL0,
        1 => ExceptionLevel::EL1,
        2 => ExceptionLevel::EL2,
        3 => ExceptionLevel::EL3,
        _ => panic!("Invalid exception level!"),
    }
}
