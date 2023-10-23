// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod gic;

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use gic::Gic;
use gic::InterruptType;
use gic::CpuId;

mod kprint;

global_asm!(include_str!("start.s"));
global_asm!(include_str!("interrupt.s"));

#[repr(C)]
pub struct ExceptionContext {
    regs: [u64; 31],
    elr_el1: u64,
    spsr_el1: u64,
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("jaw_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");


    const TIMER_IRQ: u32 = 30;

    init_exception_table();

    let gic = Gic::new(0x0800_0000 as usize, 0x0801_0000 as usize);
    gic.init_gic();
    // Set the timer interrupt to be level sensitive with set_cfg
    gic.set_cfg(TIMER_IRQ, InterruptType::LevelSensitive);
    gic.set_priority(TIMER_IRQ, 0);
    gic.set_target(TIMER_IRQ, CpuId::Cpu1);
    gic.clear_pending(TIMER_IRQ);
    gic.set_enable(TIMER_IRQ);

    let freq_val: u64;
    let ctl_val: u64 = 1;
    let next: u64;
    let delta: u64 = 100_000;
    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "msr CNTP_CTL_EL0, {1}",
            "msr CNTP_TVAL_EL0, {3}",
            "mrs {2}, CNTP_CVAL_EL0",
            out(reg) freq_val,
            in(reg) ctl_val,
            out(reg) next,
            in(reg) delta,
        );
    }

    // kprintf frequency and tick values as hex
    kprintf!("freq_val: {:#x}\n next: {:#x}\n", freq_val, next);

    loop {
        let mut nop_cnt: u64 = 0;
        while nop_cnt < 200000 {
            unsafe {
                asm!("nop");
            }
            nop_cnt += 1;
        }
        let cntpct_val: u64;
        unsafe {
            asm!(
                "mrs {0}, CNTPCT_EL0",
                out(reg) cntpct_val,
            );
        }
        kprintf!("cntpct_val: {:#x}\n", cntpct_val);
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