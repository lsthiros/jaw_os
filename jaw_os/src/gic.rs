use core::arch::asm;
use core::ptr;

// GIC Info and addresses from:
// http://web.archive.org/web/20230327162435/https://lowenware.com/blog/aarch64-gic-and-timer-interrupt/
// Distributor Controller
const GICD_CTLR_OFFSET: usize = 0x000;
const GICD_ISENABLER_OFFSET: usize = 0x100;

// CPU Interface Controller
const GICC_CTLR_OFFSET: usize = 0x000;
const GICC_PMR_OFFSET: usize = 0x004;
const GICC_BPR_OFFSET: usize = 0x008;

pub struct Gic {
    gicd_ctlr: *mut u32,
    gicc_ctlr: *mut u32,
}

impl Gic {
    pub fn init_gic(&self) {
        unsafe {
            ptr::write_volatile(self.gicd_ctlr.add(GICD_CTLR_OFFSET), 1);
            ptr::write_volatile(self.gicc_ctlr.add(GICC_CTLR_OFFSET), 1);
            ptr::write_volatile(self.gicc_ctrl.add(GICC_PMR_OFFSET), 0xFF);
        }
    }

    const GICD_ISENABLER_SIZE: u32 = 32;
    pub fn enable(interrupt: u32) {
        let interrupt_bit: u32 = 1 << (interrupt % GICD_ISENABLER_SIZE);
        let interrupt_offset: u32 = interrupt / GICD_ISENABLER_SIZE;
        unsafe {
            ptr::write_volatile(
                self.gicd_ctlr.add(GICD_ISENABLER_OFFSET + interrupt_offset),
                interrupt_bit,
            );
        }
    }
}

