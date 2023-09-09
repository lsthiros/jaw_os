use core::arch::asm;
use core::ptr;

use crate::kprintf;

// GIC Info and addresses from:
// http://web.archive.org/web/20230327162435/https://lowenware.com/blog/aarch64-gic-and-timer-interrupt/
pub struct Gic {
    gicd_ctlr: usize,
    gicc_ctlr: usize,
}

pub enum InterruptType {
    LevelSensitive = 0,
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

impl Gic {
    // Distributor Controller
    const GICD_CTLR_OFFSET: usize = 0x000;

    // CPU Interface Controller
    const GICC_CTLR_OFFSET: usize = 0x000;
    const GICC_PMR_OFFSET: usize = 0x004;

    pub fn new(gicd_ctlr: usize, gicc_ctlr: usize) -> Self {
        Self {
            gicd_ctlr,
            gicc_ctlr,
        }
    }

    pub fn init_gic(&self) {
        unsafe {
            ptr::write_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32, 1);
            ptr::write_volatile((self.gicc_ctlr + Self::GICC_CTLR_OFFSET) as *mut u32, 1);
            ptr::write_volatile((self.gicc_ctlr + Self::GICC_PMR_OFFSET) as *mut u32, 0xFF);
        }
    }

    pub fn set_enable(&self, interrupt: u32) {
        const GICD_ISENABLER_SIZE: u32 = 32;
        const GICD_ISENABLER_OFFSET: usize = 0x100;
        let gicd_isenabler: *mut u32 = (self.gicd_ctlr + GICD_ISENABLER_OFFSET) as *mut u32;

        let interrupt_set_enable_bit: u32 = 1 << (interrupt % GICD_ISENABLER_SIZE);
        let interrupt_set_enable_register: usize = (interrupt / GICD_ISENABLER_SIZE) as usize;
        unsafe {
            ptr::write_volatile(gicd_isenabler.add(interrupt_set_enable_register),interrupt_set_enable_bit);
        }
    }

    pub fn clear_enable(&self, interrupt: u32) {
        const GICD_ICENABLER_SIZE: u32 = 32;
        const GICD_ICENABLER_OFFSET: usize = 0x180;
        let interrupt_clear_enable_bit: u32 = 1 << (interrupt % GICD_ICENABLER_SIZE);
        let interrupt_clear_enable_register: usize = (interrupt / GICD_ICENABLER_SIZE) as usize;
        let gicd_icenabler: *mut u32 = (self.gicd_ctlr + GICD_ICENABLER_OFFSET) as *mut u32;
        unsafe {
            ptr::write_volatile(gicd_icenabler.add(interrupt_clear_enable_register), interrupt_clear_enable_bit);
        }
    }

    pub fn set_priority(&self, interrupt: u32, priority: u8) {
        const GICD_IPRIORITYR_OFFSET: usize = 0x400;
        const GICD_IPRIORITYR_SIZE: u32 = 4;
        const PRIORITY_BIT_WIDTH: u32 = 8;
        const PRIORITY_FIELD_MASK: u32 = (1 << PRIORITY_BIT_WIDTH) - 1;

        let interrupt_priority_register_offset: usize = (interrupt / GICD_IPRIORITYR_SIZE) as usize;
        let gicd_ipriorityr: *mut u32 = (self.gicd_ctlr + GICD_IPRIORITYR_OFFSET) as *mut u32;

        let interrupt_priority_bit_offset: u32 =
            (interrupt % GICD_IPRIORITYR_SIZE) * PRIORITY_BIT_WIDTH;
        let interrupt_priority_mask: u32 = PRIORITY_FIELD_MASK << interrupt_priority_bit_offset;
        let interrupt_priority_in_place: u32 = (priority as u32) << interrupt_priority_bit_offset;

        let current_interrupt_priority: u32 = unsafe {
            ptr::read_volatile(
                    gicd_ipriorityr.add(interrupt_priority_register_offset),
            )
        };
        let new_interrupt_priority: u32 =
            (current_interrupt_priority & !interrupt_priority_mask) | interrupt_priority_in_place;
        unsafe {
            ptr::write_volatile(
                gicd_ipriorityr.add(interrupt_priority_register_offset),
                new_interrupt_priority,
            );
        };
    }

    pub fn set_target(&self, interrupt: u32, target: CpuId) {
        const GICD_ITARGETSR_OFFSET: usize = 0x800;
        const GICD_ITARGETSR_SIZE: u32 = 4;
        const TARGET_BIT_WIDTH: u32 = 8;

        let interrupt_target_register_offset: usize = (interrupt / GICD_ITARGETSR_SIZE) as usize;
        let gicd_itargetsr: *mut u32 = (self.gicd_ctlr + GICD_ITARGETSR_OFFSET) as *mut u32;
        let current_interrupt_target: u32 = unsafe {
            ptr::read_volatile(
                gicd_itargetsr.add(interrupt_target_register_offset),
            )
        };

        let interrupt_target_mask: u32 =
            1 << (interrupt % GICD_ITARGETSR_SIZE * TARGET_BIT_WIDTH + target as u32);
        let new_interrupt_target: u32 = current_interrupt_target | interrupt_target_mask;
        unsafe {
            ptr::write_volatile(
                gicd_itargetsr.add(interrupt_target_register_offset),
                new_interrupt_target,
            );
        };
    }

    pub fn set_cfg(&self, interrupt: u32, cfg: InterruptType) {
        const GICD_ICFGR_OFFSET: usize = 0xC00;
        const GICD_ICFGR_SIZE: u32 = 16;
        const CFG_BIT_WIDTH: u32 = 2;

        let interrupt_cfg_register_offset: usize = (interrupt / GICD_ICFGR_SIZE) as usize;
        let gicd_icfgr: *mut u32 = (self.gicd_ctlr + GICD_ICFGR_OFFSET) as *mut u32;
        let current_interrupt_cfg: u32 = unsafe {
            ptr::read_volatile(
                gicd_icfgr.add(interrupt_cfg_register_offset),
            )
        };

        let interrupt_cfg_mask: u32 =
            (cfg as u32) << ((interrupt % GICD_ICFGR_SIZE) * CFG_BIT_WIDTH);
        let interrupt_cfg_field_mask: u32 = 3 << ((interrupt % GICD_ICFGR_SIZE) * CFG_BIT_WIDTH);
        let new_interrupt_cfg: u32 =
            (current_interrupt_cfg & !interrupt_cfg_field_mask) | interrupt_cfg_mask;
        unsafe {
            ptr::write_volatile(
                gicd_icfgr.add(interrupt_cfg_register_offset),
                new_interrupt_cfg,
            );
        };
    }

    pub fn clear_pending(&self, interrupt: u32) {
        const GICD_ICPENDR_SIZE: u32 = 32;
        const GICD_ICPENDR_OFFSET: usize = 0x280;
        let interrupt_clear_pending_bit: u32 = 1 << (interrupt % GICD_ICPENDR_SIZE);
        let interrupt_clear_pending_register: usize = (interrupt / GICD_ICPENDR_SIZE) as usize;
        let gicd_icpendr: *mut u32 = (self.gicd_ctlr + GICD_ICPENDR_OFFSET) as *mut u32;
        unsafe {
            ptr::write_volatile(
                gicd_icpendr.add(GICD_ICPENDR_OFFSET + interrupt_clear_pending_register),
                interrupt_clear_pending_bit,
            );
        }
    }
}
