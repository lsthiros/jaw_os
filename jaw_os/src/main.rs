#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use core::ptr;

global_asm!(include_str!("start.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"IPv6 Only Network Stack\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    loop {}
}
