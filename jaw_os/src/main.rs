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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn putc(c: u8) {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    unsafe {
        ptr::write_volatile(UART0, c);
    }
}

fn print_str(s: &[u8]) {
    for byte in s {
        putc(*byte);
    }
}

const HEX: &[u8] = b"0123456789abcdef";
// print_hex<T> is a generic function that prints the hexadecimal representation of any integer type.
fn print_hex<
    T: Into<u64>
        + core::ops::Shl<i32, Output = T>
        + core::ops::Shr<i32, Output = T>
        + core::marker::Copy,
>(
    mut val: T,
) {
    let nibble_len: usize = size_of::<T>() * 2;
    let bit_len: usize = nibble_len * 4;
    let mut nibble_count: usize = nibble_len;

    while nibble_count != 0 {
        nibble_count -= 1;
        // Let nibble: usize be the most significant nibble of val as it currently stands.
        let nibble: usize = (val.into() >> (bit_len - 4)) as usize;
        let char: u8 = HEX[nibble as usize];
        putc(char);
        val = val << 4;
    }
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    const VAL: u64 = 0x1234_5678_9abc_def0;
    let out_str = b"jaw_os: The best operating system because it supports IPv6 Exclusivley (tm)\n";
    print_str(out_str);
    kprintf!("IPv6 Only Network Stack running version {}\n", 23);

    let freq_val: u64;
    let tick_val: u64;
    let gic = Gic::new(0x0800_0000 as *mut u32, 0x0800_1000 as *mut u32);
    gic.init_gic();

    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "mrs {1}, CNTPCT_EL0",
            out(reg) freq_val,
            out(reg) tick_val,
        );
    }

    print_hex(freq_val);
    putc(b'\n');
    print_hex(tick_val);
    putc(b'\n');
    print_hex(VAL);
    putc(b'\n');
    loop {}
}
