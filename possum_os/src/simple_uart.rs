// SPDX-License-Identifier: GPL-3.0-only
use core::fmt::{self, Write};
use core::ptr;

pub struct SimpleUart {
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

impl SimpleUart {
    const UARTDR_OFFSET: usize = 0x00;

    const UARTFR_OFFSET: usize = 0x18;
    const UARTFR_BUSY_BIT_MASK: u8 = 0x08;
    const UARTFR_RXFE_BIT_MASK: u8 = 0x10;

    pub fn putc(&mut self, c: u8) {
        unsafe {
            ptr::write_volatile(self.base, c);
        }
        let mut busy: bool;
        loop {
            unsafe {
                busy = (ptr::read_volatile(self.base.add(Self::UARTFR_OFFSET)) & Self::UARTFR_BUSY_BIT_MASK) != 0;
            }
            if !busy {
                break;
            }
        }
    }
    
    // Empty: true if the receive FIFO is empty.
    pub fn empty(&mut self) -> bool {
        unsafe {
            (ptr::read_volatile(self.base.add(Self::UARTFR_OFFSET)) & Self::UARTFR_RXFE_BIT_MASK) != 0
        }
    }

    pub fn getc(&mut self) -> u8 {
        let mut empty: bool;
        loop {
            unsafe {
                empty = ptr::read_volatile(self.base.add(Self::UARTFR_OFFSET)) & Self::UARTFR_RXFE_BIT_MASK != 0;
            }
            if !empty {
                break;
            }
        }
        unsafe { ptr::read_volatile(self.base.add(Self::UARTDR_OFFSET)) }
    }
}