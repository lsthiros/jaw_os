// SPDX-License-Identifier: GPL-3.0-only
use core::fmt::{self, Write};
use core::ptr;

struct SimpleUart {
    base: *mut u8,
}

// impl new for SimpleUart
impl SimpleUart {
    pub fn new(base: *mut u8) -> Self {
        Self { base }
    }
}

// Implementations of the `core::fmt::Write` trait.
impl Write for SimpleUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                ptr::write_volatile(self.base, byte);
            }
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        unsafe {
            ptr::write_volatile(self.base, c as u8);
        }
        Ok(())
    }
}

pub fn _kprintf(args: fmt::Arguments<'_>) {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let mut writer = SimpleUart::new(UART0);
    write!(&mut writer, "{}", args).unwrap();
}

#[macro_export]
macro_rules! kprintf {
    ($($arg:tt)*) => ({
        use crate::kprint::_kprintf;
        _kprintf(format_args!($($arg)*));
    })
}
