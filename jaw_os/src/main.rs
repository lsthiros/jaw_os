// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod exception;
mod gic;

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use gic::CpuId;
use gic::Gic;
use gic::InterruptType;

mod kprint;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("jaw_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");

    const TIMER_IRQ: u32 = 30;

    // Enable interrupts in DAIF
    kprintf!("DAIFClr\n");
    unsafe {
        asm!("msr DAIFClr, 0x2",);
    }

    exception::init_exception_table();
    let current_el: exception::ExceptionLevel = exception::get_current_el();
    kprintf!("Current exception level: {:?}\n", current_el);

    kprintf!("Group enable\n");
    exception::configure_groups();

    let gic = Gic::new(0x0800_0000 as usize, 0x0801_0000 as usize);
    kprintf!("Init GIC\n");
    gic.init_gic();
    kprintf!("done\n");
    // Set the timer interrupt to be level sensitive with set_cfg
    kprintf!("Set cfg\n");

    gic.set_cfg(TIMER_IRQ, InterruptType::LevelSensitive);
    gic.set_priority(TIMER_IRQ, 0);
    gic.set_target(TIMER_IRQ, CpuId::Cpu0);
    gic.clear_pending(TIMER_IRQ);
    gic.set_enable(TIMER_IRQ);
    gic.set_group(TIMER_IRQ);

    kprintf!("Set timer\n");

    let freq_val: u64;
    let ctl_val: u64 = 1;
    let next: u64;
    let delta: u64 = 100_000_000;
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
        let timer_ctl: u64;
        let remaining: u64;
        unsafe {
            asm!(
                "mrs {0}, CNTPCT_EL0",
                "mrs {1}, CNTP_CTL_EL0",
                "mrs {2}, CNTP_TVAL_EL0",
                out(reg) cntpct_val,
                out(reg) timer_ctl,
                out(reg) remaining,
            );
        }
        kprintf!(
            "cntpct_val: {:#x} ctl: {:#x} remain:{:#x}",
            cntpct_val,
            timer_ctl,
            remaining
        );
        let pending: u64 = gic.get_pending(TIMER_IRQ) as u64;
        kprintf!(" pending: {:#x}\n", pending);
        if (pending != 0) {
            loop {}
        }
    }
}
