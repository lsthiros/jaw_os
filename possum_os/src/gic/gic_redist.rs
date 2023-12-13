// SPDX-License-Identifier: GPL-3.0-only
use crate::gic::common::{write_register, read_register};

pub struct GicRedist {
    gicr_base: usize,
}

impl GicRedist {
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
            ptr::read_volatile((self.gicr_ctlr + GICR_WAKER_OFFSET) as *mut u32)
        };
        kprintf!("gicr_waker_contents: {:#x}\n", gicr_waker_contents);
        // Clear sleep bit in waker
        const WAKER_SLEEP_BIT: u32 = 0b1 << 1;
        gicr_waker_contents &= !WAKER_SLEEP_BIT;
        unsafe {
            ptr::write_volatile((self.gicr_ctlr + GICR_WAKER_OFFSET) as *mut u32, gicr_waker_contents);
        }
        kprintf!("Marking system as awake\n");
        // Loop until ChildrenAsleep is 0

        const GICR_WAKER_CHILDREN_ASLEEP_BIT: u32 = 0b1 << 2;
        loop {
            gicr_waker_contents = unsafe {
                ptr::read_volatile((self.gicr_ctlr + GICR_WAKER_OFFSET) as *mut u32)
            };
            if gicr_waker_contents & GICR_WAKER_CHILDREN_ASLEEP_BIT == 0 {
                break;
            }
        }
        kprintf!("system marked as awake\n");
    }

    pub fn set_interrupt_config(&self, interrupt_id: u32, config: u32) {
        const GICR_ICFGR_OFFSET: usize = 0x0c;
        const GICR_ICFGR_FIELD_WIDTH: u32 = 2;
        write_register(
            self.gicr_base + GICR_ICFGR_OFFSET,
            GICR_ICFGR_FIELD_WIDTH,
            interrupt_id,
            config,
        );
    }

    pub fn get_redistributor_pending(&self, interrupt: u32) -> bool {
        const GICR_ISPENDR_OFFSET: usize = 0x200;
        const GICR_ISPENDR_FIELD_SIZE: u32 = 1;
        read_register(
            self.gicr_ctlr + GICR_ISPENDR_OFFSET,
            GICR_ISPENDR_FIELD_SIZE,
            interrupt,
        ) == 1
    }
}