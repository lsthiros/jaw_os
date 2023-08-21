#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::mem::size_of;
use core::ptr;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const HEX: &[u8] = b"0123456789abcdef";
// print_hex<T> is a generic function that prints the hexadecimal representation of any integer type.
fn print_hex<T: Into<u64> + core::ops::Shr<i32, Output = T> + core::marker::Copy>(mut val: T) {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let mut nibble_len: usize = size_of::<T>() * 2;
    while nibble_len != 0 {
        nibble_len -= 1;
        let nibble = val.into() & 0xf;
        let char: u8 = HEX[nibble as usize];
        unsafe {
            ptr::write_volatile(UART0, char);
        }
        val = val >> 4;
    }
}


#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    const VAL: u64 = 0x1234_5678_9abc_def0;
    let out_str = b"IPv6 Only Network Stack\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    print_hex(VAL);
    loop {}
}
