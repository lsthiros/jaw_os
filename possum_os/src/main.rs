// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

mod kprint;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintf!("Possum Panic! {}\n", _info);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    kprintf!("IPv6 Only Network Stack running version {}\n", 23);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
