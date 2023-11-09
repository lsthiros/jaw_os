// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

mod console;
mod kprint;
mod simple_uart;
mod ring_buffer;

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
    let mut console = console::Console::new();
    loop {
        console.service();
    }
}
