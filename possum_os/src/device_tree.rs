// SPDX-License-Identifier: GPL-3.0-only
use core::convert::From;
use core::fmt::LowerHex;
use core::ops::{Shl, Shr, BitAnd, BitOr};

use crate::kprintf;

pub const QEMU_DEVICE_TREE_OFSET: usize = 0x4000_0000;

#[derive(Debug)]
pub struct DeviceTree {
    base: *const u8,
    structure_offset: u32,
    strings_offset: u32,
    mem_offset: u32,
    version: u32,
    last_comp_version: u32,
    structure_size: u32,
    strings_size: u32,
}

const FDT_BEGIN_NODE: u32 = 0x0000_0001;
const FDT_END_NODE: u32 = 0x0000_0002;
const FDT_PROP: u32 = 0x0000_0003;
const FDT_NOP: u32 = 0x0000_0004;
const FDT_END: u32 = 0x0000_0009;

pub fn device_tree_from_ram_ptr(ram_ptr: *const u8) -> DeviceTree {
    let mut dt = DeviceTree::new();
    let mut ptr = ram_ptr;

    let magic = read_big_endian(ptr as *const u32);
    if magic != 0xd00d_feed {
        kprintf!("Invalid device tree magic {:#x}\n", magic);
        return dt;
    }

    let totalsize = read_big_endian(unsafe { (ptr.add(4) as *const u32) });
    let off_dt_struct = read_big_endian(unsafe { (ptr.add(8) as *const u32) });
    let off_dt_strings = read_big_endian(unsafe { (ptr.add(12) as *const u32) });
    let off_mem_rsvmap = read_big_endian(unsafe { (ptr.add(16) as *const u32) });
    let version = read_big_endian(unsafe { (ptr.add(20) as *const u32) });
    let last_comp_version = read_big_endian(unsafe { (ptr.add(24) as *const u32) });
    let boot_cpuid_phys = read_big_endian(unsafe { (ptr.add(28) as *const u32) });
    let size_dt_strings = read_big_endian(unsafe { (ptr.add(32) as *const u32) });
    let size_dt_struct = read_big_endian(unsafe { (ptr.add(36) as *const u32) });

    dt.base = ram_ptr;
    dt.structure_offset = off_dt_struct;
    dt.strings_offset = off_dt_strings;
    dt.mem_offset = off_mem_rsvmap;
    dt.version = version;
    dt.last_comp_version = last_comp_version;
    dt.structure_size = size_dt_struct;
    dt.strings_size = size_dt_strings;

    dt
}

fn read_big_endian<T: Copy + From<u8> + Shl<i32, Output = T> + Shr<i32, Output = T> + BitAnd<T, Output = T> + BitOr<T, Output = T> + LowerHex>(addr: *const T) -> T {
    let mut input: T = unsafe {*addr};
    let mut result: T = From::from(0);
    kprintf!("input: {:#x}\n", input);

    for _ in 0..core::mem::size_of::<T>() {
        result = result << 8;
        // TODO: 0xFF from will sign extend for signed types. Avoid that.
        result = result | (input & From::from(0xff));
        input = input >> 8;
    }

    result
}

fn strnlen(ptr: *const u8, max_len: usize) -> usize {
    let mut ptr: *const u8 = ptr;
    let mut i = 0;
    while i < max_len && unsafe { *ptr } != 0 {
        ptr = unsafe { ptr.add(1) };
        i += 1;
    }
    i
}

const MAX_STR_LEN: usize = 256;

impl DeviceTree {
    fn new() -> Self {
        let dt = Self {
            base: core::ptr::null(),
            structure_offset: 0,
            strings_offset: 0,
            mem_offset: 0,
            version: 0,
            last_comp_version: 0,
            structure_size: 0,
            strings_size: 0,
        };
        dt
    }

    pub fn get_string_from_offset(&self, offset: u32) -> &str {
        let ptr = unsafe { self.base.add(self.strings_offset as usize).add(offset as usize) };
        let len = strnlen(ptr, MAX_STR_LEN);
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        unsafe {core::str::from_utf8_unchecked(slice)}
    }

    pub fn print_structure(&self) {
        let mut indent: usize = 0;
        let mut ptr = unsafe { self.base.add(self.structure_offset as usize) };
        while ptr < unsafe { self.base.add(self.structure_offset as usize + self.structure_size as usize) } {
            ptr = unsafe { ptr.add(size as usize) };
        }
    }
}
