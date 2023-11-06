// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod exception;
mod gic;

<<<<<<< HEAD
use core::arch::asm;
use core::arch::global_asm;
=======
use core::arch::{asm, global_asm};
>>>>>>> master
use core::panic::PanicInfo;
use gic::CpuId;
use gic::Gic;
use gic::InterruptType;

mod kprint;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
<<<<<<< HEAD
    kprintf!("Possum Panic: {}\n", _info);
=======
    kprintf!("Possum Panic! {}\n", _info);
>>>>>>> master
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
<<<<<<< HEAD
    kprintf!("possum_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");

    const TIMER_IRQ: u32 = 30;

    exception::init_exception_table();
    let current_el: exception::ExceptionLevel = exception::get_current_el();
    kprintf!("Current exception level: {:?}\n", current_el);

    let gic = Gic::new(0x0800_0000 as usize, 0x0801_0000 as usize, 0x080A_0000 as usize);
    kprintf!("Init GIC\n");
    gic.init_gic();
    // Set the timer interrupt to be level sensitive with set_cfg
    kprintf!("Set cfg\n");

    gic.set_redistributor_priority(TIMER_IRQ, 0);
    gic.set_group(TIMER_IRQ, true);
    gic.set_cfg(TIMER_IRQ, InterruptType::LevelSensitive);
    gic.clear_pending(TIMER_IRQ);
    gic.set_enable(TIMER_IRQ);

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
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
        }

        if (remaining > delta) {
            kprintf!("remaining > delta. Setting interrupt manually and hoping for the best\n");
            gic.set_pending(TIMER_IRQ);
            gic.set_redistributor_pending(TIMER_IRQ);
            kprintf!("good luck, us :)\n");
            let val: u32 = gic.get_pending(TIMER_IRQ) as u32;
            let other_val: u32 = gic.get_redistributor_pending(TIMER_IRQ) as u32;
            kprintf!("val: {:#x} other_val {:#x}\n", val, other_val);
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
=======
    kprintf!("IPv6 Only Network Stack running version {}\n", 23);
    loop {
        unsafe {
            asm!("wfi");
>>>>>>> master
        }
    }
}
