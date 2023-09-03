// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod gic;

use core::arch::asm;
use core::arch::global_asm;
use core::mem::size_of;
use core::panic::PanicInfo;
use core::ptr;
use gic::Gic;

mod kprint;

global_asm!(include_str!("start.s"));
global_asm!(include_str!("interrupt.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("jaw_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");

    let freq_val: u64;
    let tick_val: u64;
    let gic = Gic::new(0x0800_0000 as *mut u32, 0x0801_0000 as *mut u32);
    gic.init_gic();

    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "mrs {1}, CNTPCT_EL0",
            out(reg) freq_val,
            out(reg) tick_val,
        );
    }

    // kprintf frequency and tick values as hex
    kprintf!(
        "freq_val: {:#x} tick_val: {:#x}\n",
        freq_val,
        tick_val
    );

    loop {}
}
