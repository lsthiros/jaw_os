// SPDX-License-Identifier: GPL-3.0-only
use core::ptr;

/// Represents the type of interrupt.
pub enum InterruptType {
    /// Level-sensitive interrupt.
    LevelSensitive = 0,
    /// Edge-triggered interrupt.
    EdgeTriggered = 2,
}

pub enum CpuId {
    Cpu0 = 0,
    Cpu1 = 1,
    Cpu2 = 2,
    Cpu3 = 3,
    Cpu4 = 4,
    Cpu5 = 5,
    Cpu6 = 6,
    Cpu7 = 7,
}


/// Writes a value to a register at a given base address, using the specified field width and entry index.
///
/// # Arguments
///
/// * `base` - The base address of the register.
/// * `field_width` - The width of the field in bits.
/// * `entry` - The index of the entry within the register.
/// * `value` - The value to write to the register.
///
/// # Safety
///
/// This function uses unsafe operations to read and write volatile memory. It should only be used in
/// situations where direct register access is necessary and appropriate safety measures are taken.
pub fn write_register(base: usize, field_width: u32, entry: u32, value: u32) {
    let entries_per_register: u32 = 32 / field_width;
    let offset: usize = (entry / entries_per_register) as usize;
    let bit_offset: u32 = (entry % entries_per_register) * field_width;
    let field_mask: u32 = (1 << field_width) - 1;

    let register: *mut u32 = (base + offset) as *mut u32;
    let current_value: u32 = unsafe { ptr::read_volatile(register) };
    let value_to_write: u32 = (current_value & !(field_mask << bit_offset)) | ((value & field_mask) << bit_offset);
    unsafe {
        ptr::write_volatile(register, value_to_write);
    }
}

/// Reads a register at the specified base address and extracts a field value based on the field width and entry index.
///
/// # Arguments
///
/// * `base` - The base address of the register.
/// * `field_width` - The width of the field in bits.
/// * `entry` - The index of the entry within the register.
///
/// # Returns
///
/// The extracted field value.
pub fn read_register(base: usize, field_width: u32, entry: u32) -> u32 {
    let entries_per_register: u32 = 32 / field_width;
    let offset: usize = (entry / entries_per_register) as usize;
    let bit_offset: u32 = (entry % entries_per_register) * field_width;
    let field_mask: u32 = (1 << field_width) - 1;

    let register: *mut u32 = (base + offset) as *mut u32;
    let current_value: u32 = unsafe { ptr::read_volatile(register) };
    (current_value >> bit_offset) & field_mask
}