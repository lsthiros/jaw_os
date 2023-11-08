// SPDX-License-Identifier: GPL-3.0-only
use crate::simple_uart::SimpleUart;
use core::fmt;
use core::fmt::Write;

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
