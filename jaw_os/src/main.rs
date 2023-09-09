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
// global_asm!(include_str!("interrupt.s"));

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

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("jaw_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");


    const TIMER_IRQ: u32 = 30;
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
    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "msr CNTP_CTL_EL0, {0}",
            "msr CNTP_TVAL_EL0, {1}",
            out(reg) freq_val,
            in(reg) ctl_val,
        );
    }

    // kprintf frequency and tick values as hex
    kprintf!("freq_val: {:#x}\n", freq_val);

    loop {}
}

#[no_mangle]
pub extern "C" fn _timer_interrupt(_ctx: &ExceptionContext) {
    kprintf!("Timer interrupt!\n");
    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "msr CNTP_TVAL_EL0, {0}",
            out(reg) _
        )
    }
    return;
}