// SPDX-License-Identifier: GPL-3.0-only
use crate::gic::common::{write_register, read_register, InterruptType};
use crate::kprintf;

use core::ptr;

pub struct GicRedist {
    gicr_base: usize,
}

impl GicRedist {
    const GICR_SGI_BASE_OFFSET: usize = 0x10000;
    pub fn new(gicr_base: usize) -> GicRedist {
        GicRedist { gicr_base }
    }

    // This will initialize the GIC as per instructions from:
    // GICv3_Software_Overview_Official_Release_B.pdf
    // document id: DAI 0492B
    pub fn init(&self) {

        // 4.2 PE Specific Initialization
        // 4.2.1: Wake up the PE
        // Mark PE as awake. writing to system registers other than ICC_SRE_EL1 will cause
        // unpredictable system behavior until this is done
        const GICR_WAKER_OFFSET: usize = 0x014;
        let mut gicr_waker_contents: u32 = unsafe {
            ptr::read_volatile((self.gicr_base + GICR_WAKER_OFFSET) as *mut u32)
        };
        kprintf!("gicr_waker_contents: {:#x}\n", gicr_waker_contents);
        // Clear sleep bit in waker
        const WAKER_SLEEP_BIT: u32 = 0b1 << 1;
        gicr_waker_contents &= !WAKER_SLEEP_BIT;
        unsafe {
            ptr::write_volatile((self.gicr_base + GICR_WAKER_OFFSET) as *mut u32, gicr_waker_contents);
        }
        kprintf!("Marking system as awake\n");
        // Loop until ChildrenAsleep is 0

        const GICR_WAKER_CHILDREN_ASLEEP_BIT: u32 = 0b1 << 2;
        loop {
            gicr_waker_contents = unsafe {
                ptr::read_volatile((self.gicr_base + GICR_WAKER_OFFSET) as *mut u32)
            };
            if gicr_waker_contents & GICR_WAKER_CHILDREN_ASLEEP_BIT == 0 {
                break;
            }
        }
        kprintf!("system marked as awake\n");
    }

    pub fn set_interrupt_config(&self, interrupt_id: u32, config: u32) {
        const GICR_ICFGR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x0c;
        const GICR_ICFGR_FIELD_WIDTH: u32 = 2;
        write_register(
            self.gicr_base + GICR_ICFGR_OFFSET,
            GICR_ICFGR_FIELD_WIDTH,
            interrupt_id,
            config,
        );
    }

    pub fn get_redistributor_pending(&self, interrupt: u32) -> bool {
        const GICR_ISPENDR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x200;
        const GICR_ISPENDR_FIELD_SIZE: u32 = 1;
        read_register(
            self.gicr_base + GICR_ISPENDR_OFFSET,
            GICR_ISPENDR_FIELD_SIZE,
            interrupt,
        ) == 1
    }

    pub fn set_enable(&self, interrupt: u32) {
        const GICR_ISENABLER_FIELD_SIZE: u32 = 1;
        const GICR_ISENABLER_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x100;
        write_register(
            self.gicr_base + GICR_ISENABLER_OFFSET,
            GICR_ISENABLER_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn clear_enable(&self, interrupt: u32) {
        const GICR_ICENABLER_FIELD_SIZE: u32 = 1;
        const GICR_ICENABLER_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x180;
        write_register(
            self.gicr_base + GICR_ICENABLER_OFFSET,
            GICR_ICENABLER_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn set_priority(&self, interrupt: u32, priority: u8) {
        const GICR_IPRIORITYR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x400;
        const GICR_IPRIORITYR_FIELD_SIZE: u32 = 8;

        write_register(
            self.gicr_base + GICR_IPRIORITYR_OFFSET,
            GICR_IPRIORITYR_FIELD_SIZE,
            interrupt,
            priority as u32,
        );
    }

    pub fn get_priority(&self, interrupt: u32) -> u8 {
        const GICR_IPRIORITYR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x400;
        const GICR_IPRIORITYR_FIELD_SIZE: u32 = 8;
        read_register(
            self.gicr_base + GICR_IPRIORITYR_OFFSET,
            GICR_IPRIORITYR_FIELD_SIZE,
            interrupt,
        ) as u8
    }

    pub fn set_pending(&self, interrupt: u32) {
        const GICR_ISPENDR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x200;
        const GICR_ISPENDR_FIELD_SIZE: u32 = 1;
        write_register(
            self.gicr_base + GICR_ISPENDR_OFFSET,
            GICR_ISPENDR_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn set_cfg(&self, interrupt: u32, cfg: InterruptType) {
        const GICR_ICFGR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0xC00;
        const GICR_ICFGR_FIELD_SIZE: u32 = 2;
        write_register(
            self.gicr_base + GICR_ICFGR_OFFSET,
            GICR_ICFGR_FIELD_SIZE,
            interrupt,
            cfg as u32,
        );
    }

    pub fn clear_pending(&self, interrupt: u32) {
        const GICR_ICPENDR_FIELD_SIZE: u32 = 1;
        const GICR_ICPENDR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x280;
        write_register(
            self.gicr_base + GICR_ICPENDR_OFFSET,
            GICR_ICPENDR_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn get_pending(&self, interrupt: u32) -> bool {
        const GICD_ISPENDR_FIELD_SIZE: u32 = 1;
        const GICD_ISPENDR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x200;
        read_register(
            self.gicr_base + GICD_ISPENDR_OFFSET,
            GICD_ISPENDR_FIELD_SIZE,
            interrupt,
        ) == 1
    }

    pub fn set_group(&self, interrupt: u32, group: bool) {
        const GICD_IGROUPR_FIELD_SIZE: u32 = 1;
        const GICD_IGROUPR_OFFSET: usize = GicRedist::GICR_SGI_BASE_OFFSET + 0x080;

        write_register(
            self.gicr_base + GICD_IGROUPR_OFFSET,
            GICD_IGROUPR_FIELD_SIZE,
            interrupt,
            group as u32,
        );
    }
}