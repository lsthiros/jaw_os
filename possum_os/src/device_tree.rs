// SPDX-License-Identifier: GPL-3.0-only
use core::convert::From;
use core::fmt::LowerHex;
use core::mem::size_of;
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

    for _ in 0..core::mem::size_of::<T>() {
        result = result << 8;
        // TODO: 0xFF from will sign extend for signed types. Avoid that.
        result = result | (input & From::from(0xff));
        input = input >> 8;
    }

    result
}

fn strnlen(ptr: *const u8, max_len: usize) -> Option<usize> {
    let mut ptr: *const u8 = ptr;
    let mut i = 0;
    while i < max_len && unsafe { *ptr } != 0 {
        ptr = unsafe { ptr.add(1) };
        i += 1;
    }
    if i == max_len {
        None
    } else {
        Some(i)
    }
}

fn console_indent(indent: usize) {
    const SPACES_PER_INDENT: usize = 2;
    for _ in 0..(indent * SPACES_PER_INDENT) {
        kprintf!(" ");
    }
}

fn next_four_byte_align(offset: usize) -> usize {
    const FOUR_BYTE_MASK: usize = 0b11;
    const ALIGNMENT: usize = 4;
    (offset + (ALIGNMENT - 1)) & !FOUR_BYTE_MASK
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
        // TODO: This is unsafe. We should check the length.
        let slice = unsafe { core::slice::from_raw_parts(ptr, len.unwrap()) };
        unsafe {core::str::from_utf8_unchecked(slice)}
    }

    pub fn print_structure(&self) {
        /// Size of a Device Tree tag in bytes
        const FDT_TAG_SIZE: usize = 4;

        let mut indent: usize = 0;
        let mut ptr = unsafe { self.base.add(self.structure_offset as usize) };
        let mut valid = true;
        let mut found_end = false;

        let end: *const u8 = unsafe { self.base.add(self.structure_offset as usize + self.structure_size as usize) };
        while (ptr < end) && valid  && !found_end {
            let tag = read_big_endian(ptr as *const u32);
            match tag {
                FDT_BEGIN_NODE => {
                    kprintf!("BEGIN_NODE\n");
                    let name_start = unsafe { ptr.add(FDT_TAG_SIZE) };
                    console_indent(indent);
                    if let Some(name_len) = strnlen(name_start, MAX_STR_LEN) {
                        kprintf!("Name len: {}\n", name_len);
                        let name_slice = unsafe { core::slice::from_raw_parts(name_start, name_len) };
                        let name = unsafe { core::str::from_utf8_unchecked(name_slice) };
                        kprintf!("{}\n", name);
                        indent += 1;
                        const NULL_BYTE_SIZE: usize = 1;
                        ptr = unsafe { name_start.add(next_four_byte_align(name_len + NULL_BYTE_SIZE)) };
                    } else {
                        kprintf!("<invalid node name>\n");
                        valid = false;
                    }
                }
                FDT_END_NODE => {
                    kprintf!("END_NODE\n");
                    if indent > 0 {
                        indent -= 1;
                        ptr = unsafe { ptr.add(FDT_TAG_SIZE) };
                    }
                    else {
                        kprintf!("Invalid end node\n");
                        valid = false;
                    }
                }
                FDT_PROP => {
                    kprintf!("PROP\n");
                    const NAME_OFFSET_OFFSET: usize = FDT_TAG_SIZE;
                    const SIZE_OFFSET: usize = NAME_OFFSET_OFFSET + size_of::<u32>();
                    const DATA_OFFSET: usize = SIZE_OFFSET + size_of::<u32>();

                    let name_offset: u32 = read_big_endian(unsafe { ptr.add(NAME_OFFSET_OFFSET) as *const u32 });
                    let name: &str = self.get_string_from_offset(name_offset);
                    let size: u32 = read_big_endian(unsafe { ptr.add(SIZE_OFFSET) as *const u32 });
                    let data: *const u8 = unsafe { ptr.add(DATA_OFFSET) };

                    console_indent(indent);
                    kprintf!("{}: ", name);
                    if size > 0 {
                        let mut i = 0;
                        while i < size {
                            let byte = unsafe { *data.add(i as usize) };
                            kprintf!("{:02x} ", byte);
                            i += 1;
                        }
                        kprintf!("\n");
                    } else {
                        kprintf!("<empty>\n");
                    }
                    ptr = unsafe { data.add((next_four_byte_align(size.try_into().unwrap())) as usize) };
                }
                FDT_NOP => {
                    kprintf!("NOP\n");
                    ptr = unsafe { ptr.add(FDT_TAG_SIZE) };
                }
                FDT_END => {
                    kprintf!("END\n");
                    found_end = true;
                }
                _ => {
                    kprintf!("Unknown tag {:#x}\n", tag);
                    break;
                }
            }
        }
        if !found_end {
            kprintf!("No end tag found\n");
        }
    }
}
