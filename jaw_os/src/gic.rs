use core::arch::asm;
use core::ptr;

// GIC Info and addresses from:
// http://web.archive.org/web/20230327162435/https://lowenware.com/blog/aarch64-gic-and-timer-interrupt/
pub struct Gic {
    gicd_ctlr: *mut u32,
    gicc_ctlr: *mut u32,
}

impl Gic {
    // Distributor Controller
    const GICD_CTLR_OFFSET: usize = 0x000;
    const GICD_ISENABLER_OFFSET: usize = 0x100;
    const GICD_ICENABLER_OFFSET: usize = 0x180;

    // CPU Interface Controller
    const GICC_CTLR_OFFSET: usize = 0x000;
    const GICC_PMR_OFFSET: usize = 0x004;
    const GICC_BPR_OFFSET: usize = 0x008;

    pub fn new(gicd_ctlr: *mut u32, gicc_ctlr: *mut u32) -> Self {
        Self {
            gicd_ctlr,
            gicc_ctlr,
        }
    }

    pub fn init_gic(&self) {
        unsafe {
            ptr::write_volatile(self.gicd_ctlr.add(Self::GICD_CTLR_OFFSET), 1);
            ptr::write_volatile(self.gicc_ctlr.add(Self::GICC_CTLR_OFFSET), 1);
            ptr::write_volatile(self.gicc_ctlr.add(Self::GICC_PMR_OFFSET), 0xFF);
        }
    }

    pub fn set_enable(&self, interrupt: u32) {
        const GICD_ISENABLER_SIZE: u32 = 32;
        let interrupt_set_enable_bit: u32 = 1 << (interrupt % GICD_ISENABLER_SIZE);
        let interrupt_set_enable_register: usize = (interrupt / GICD_ISENABLER_SIZE) as usize;
        unsafe {
            ptr::write_volatile(
                self.gicd_ctlr.add(Self::GICD_ISENABLER_OFFSET + interrupt_set_enable_register),
                interrupt_set_enable_bit,
            );
        }
    }

    pub fn clear_enable(&self, interrupt: u32) {
        const GICD_ICENABLER_SIZE: u32 = 32;
        let interrupt_clear_enable_bit: u32 = 1 << (interrupt % GICD_ICENABLER_SIZE);
        let interrupt_clear_enable_register: usize = (interrupt / GICD_ICENABLER_SIZE) as usize;
        unsafe {
            ptr::write_volatile(
                self.gicd_ctlr.add(Self::GICD_ICENABLER_OFFSET + interrupt_clear_enable_register),
                interrupt_clear_enable_bit
            );
        }
    }

    pub fn set_priority(&self, interrupt: u32, priority: u8) {
        const GICD_IPRIORITYR_OFFSET: usize = 0x400;
        const GICD_IPRIORITYR_SIZE: u32 = 4;
        const PRIORITY_BIT_WIDTH: u32 = 8;
        const PRIORITY_FIELD_MASK: u32 = (1 << PRIORITY_BIT_WIDTH) - 1;

        let interrupt_priority_register_offset: usize = (interrupt / GICD_IPRIORITYR_SIZE) as usize;
        let interrupt_priority_bit_offset: u32 = (interrupt % GICD_IPRIORITYR_SIZE) * PRIORITY_BIT_WIDTH;
        let interrupt_priority_mask: u32 =  PRIORITY_FIELD_MASK << interrupt_priority_bit_offset;
        let interrupt_priority_in_place: u32 = (priority as u32) << interrupt_priority_bit_offset;
        let current_interrupt_priority: u32 = unsafe {
            ptr::read_volatile(
                self.gicd_ctlr.add(GICD_IPRIORITYR_OFFSET + interrupt_priority_register_offset)
            )
        };
        let new_interrupt_priority: u32 = (current_interrupt_priority & !interrupt_priority_mask) | interrupt_priority_in_place;
        unsafe {
            ptr::write_volatile(
                self.gicd_ctlr.add(GICD_IPRIORITYR_OFFSET + interrupt_priority_register_offset),
                new_interrupt_priority
            );
        };
    }
}

