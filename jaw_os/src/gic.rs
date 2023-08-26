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

    const GICD_ISENABLER_SIZE: u32 = 32;
    pub fn enable(&self, interrupt: u32) {
        let interrupt_bit: u32 = 1 << (interrupt % Self::GICD_ISENABLER_SIZE);
        let interrupt_offset: usize = (interrupt / Self::GICD_ISENABLER_SIZE) as usize;
        unsafe {
            ptr::write_volatile(
                self.gicd_ctlr.add(Self::GICD_ISENABLER_OFFSET + interrupt_offset),
                interrupt_bit,
            );
        }
    }
}

