// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod kprint;
mod reg;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("IPv6 Only Network Stack running version {}\n", 23);
    let el = reg::aarch64_mrs("CurrentEL");
    loop {}
}
