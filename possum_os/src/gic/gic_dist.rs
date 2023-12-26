// SPDX-License-Identifier: GPL-3.0-only

use core::arch::asm;
use core::ptr;

use crate::{kprintf, gic};
use crate::gic::common::{write_register, read_register, InterruptType, CpuId};

// GIC Info and addresses from:
// http://web.archive.org/web/20230327162435/https://lowenware.com/blog/aarch64-gic-and-timer-interrupt/
pub struct GicDistributor {
    gicd_ctlr: usize,
}

impl GicDistributor {
    // Distributor Controller
    const GICD_CTLR_OFFSET: usize = 0x000;

    pub fn new(gicd_ctlr: usize) -> Self {
        Self {
            gicd_ctlr,
        }
    }

    // This will initialize the GIC as per instructions from:
    // GICv3_Software_Overview_Official_Release_B.pdf
    // document id: DAI 0492B
    pub fn init_gic(&self) {

        // 4.1: Global settings
        let mut ctlr_contents: u32 = unsafe {
            ptr::read_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32)
        };
        kprintf!("ctlr_contents: {:#x}\n", ctlr_contents);

        // Disable Security
        const GICD_CTLR_DISABLE_SECURITY: u32 = 0b1 << 6;
        ctlr_contents |= GICD_CTLR_DISABLE_SECURITY;
        unsafe {
            ptr::write_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32, ctlr_contents);
        }

        // Enable Group 0 and Group 1 interrupts
        const GICD_CTLR_ENABLE_G1NS: u32 = 0b1 << 1;
        const GICD_CTLR_ENABLE_G0: u32 = 0b1 << 0;
        ctlr_contents |= GICD_CTLR_ENABLE_G1NS | GICD_CTLR_ENABLE_G0;
        unsafe {
            ptr::write_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32, ctlr_contents);
        }

        // Enable non-security affinity-routed interrupts
        const GICD_CTLR_ARE_NS_BIT: u32 = 0b1 << 5;
        ctlr_contents |= GICD_CTLR_ARE_NS_BIT;
        unsafe {
            ptr::write_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32, ctlr_contents);
        }
    }

    pub fn set_group(&self, interrupt: u32, group: bool) {
        const GICD_IGROUPR_FIELD_SIZE: u32 = 1;
        const GICD_IGROUPR_OFFSET: usize = 0x080;

        write_register(
            self.gicd_ctlr + GICD_IGROUPR_OFFSET,
            GICD_IGROUPR_FIELD_SIZE,
            interrupt,
            group as u32,
        );
    }

    pub fn set_enable(&self, interrupt: u32) {
        const GICD_ISENABLER_FIELD_SIZE: u32 = 1;
        const GICD_ISENABLER_OFFSET: usize = 0x100;
        write_register(self.gicd_ctlr + GICD_ISENABLER_OFFSET,
            GICD_ISENABLER_FIELD_SIZE,
            interrupt,
            0b1);
    }

    pub fn clear_enable(&self, interrupt: u32) {
        const GICD_ICENABLER_FIELD_SIZE: u32 = 1;
        const GICD_ICENABLER_OFFSET: usize = 0x180;
        write_register(
            self.gicd_ctlr + GICD_ICENABLER_OFFSET,
            GICD_ICENABLER_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }


    pub fn set_priority(&self, interrupt: u32, priority: u8) {
        const GICD_IPRIORITYR_OFFSET: usize = 0x400;
        const GICD_IPRIORITYR_FIELD_SIZE: u32 = 8;

        write_register(
            self.gicd_ctlr + GICD_IPRIORITYR_OFFSET,
            GICD_IPRIORITYR_FIELD_SIZE,
            interrupt,
            priority as u32,
        );
    }


    pub fn set_target(&self, interrupt: u32, target: CpuId) {
        const GICD_ITARGETSR_OFFSET: usize = 0x800;
        const GICD_ITARGETSR_SIZE: u32 = 4;
        const TARGET_BIT_WIDTH: u32 = 8;

        let interrupt_target_register_offset: usize = (interrupt / GICD_ITARGETSR_SIZE) as usize;
        let gicd_itargetsr: *mut u32 = (self.gicd_ctlr + GICD_ITARGETSR_OFFSET) as *mut u32;
        let current_interrupt_target: u32 =
            unsafe { ptr::read_volatile(gicd_itargetsr.add(interrupt_target_register_offset)) };

        let interrupt_target_mask: u32 = 1
            << ((((interrupt % GICD_ITARGETSR_SIZE) * TARGET_BIT_WIDTH) + (target as u32)) as u32);
        let new_interrupt_target: u32 = current_interrupt_target | interrupt_target_mask;
        unsafe {
            kprintf!(
                "new_interrupt_target {:#x} write to {:#p}\n",
                new_interrupt_target,
                gicd_itargetsr.add(interrupt_target_register_offset)
            );
            ptr::write_volatile(
                gicd_itargetsr.add(interrupt_target_register_offset),
                new_interrupt_target,
            );
        };
    }

    pub fn set_cfg(&self, interrupt: u32, cfg: InterruptType) {
        const GICD_ICFGR_OFFSET: usize = 0xC00;
        const GICD_ICFGR_FIELD_SIZE: u32 = 2;
        write_register(
            self.gicd_ctlr + GICD_ICFGR_OFFSET,
            GICD_ICFGR_FIELD_SIZE,
            interrupt,
            cfg as u32,
        );
    }

    pub fn clear_pending(&self, interrupt: u32) {
        const GICD_ICPENDR_FIELD_SIZE: u32 = 1;
        const GICD_ICPENDR_OFFSET: usize = 0x280;
        write_register(
            self.gicd_ctlr + GICD_ICPENDR_OFFSET,
            GICD_ICPENDR_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn get_pending(&self, interrupt: u32) -> bool {
        const GICD_ISPENDR_FIELD_SIZE: u32 = 1;
        const GICD_ISPENDR_OFFSET: usize = 0x200;
        read_register(
            self.gicd_ctlr + GICD_ISPENDR_OFFSET,
            GICD_ISPENDR_FIELD_SIZE,
            interrupt,
        ) == 1
    }

    pub fn set_pending(&self, interrupt: u32) {
        const GICD_ISPENDR_FIELD_SIZE: u32 = 1;
        const GICD_ISPENDR_OFFSET: usize = 0x200;
        write_register(
            self.gicd_ctlr + GICD_ISPENDR_OFFSET,
            GICD_ISPENDR_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }
}
