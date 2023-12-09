// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod exception;
mod gic;

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use gic::CpuId;
use gic::Gic;
use gic::InterruptType;

mod console;
mod device_tree;
mod kprint;
mod ring_buffer;
mod simple_uart;

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
    kprintf!("possum_os: The best operating system because it supports IPv6 Exclusivley (tm)\n");
    kprintf!("IPv6 Only Network Stack running version {}\n", 23);
    // Running into some very odd problems here. See my notes
    // in the README.md file.
    let magic: *const u32 = 0x4000_0000 as *const u32;
    kprintf!("Magic: {:#x}\n", unsafe { *magic });
    let mut console = console::Console::new();
    loop {
        console.service();
    }
}
