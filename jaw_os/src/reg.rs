// SPDX-License-Identifier: GPL-3.0-only

use core::arch::asm;

pub fn aarch64_mrs(reg: u64) -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs $0, $1" : "=r"(val) : "i"(reg) :: "volatile");
    }
    val
}

pub fn aarch64_msr(reg: u64, val: u64) {
    unsafe {
        asm!("msr $0, $1" :: "i"(reg), "r"(val) :: "volatile");
    }
}
