// SPDX-License-Identifier: GPL-3.0-only

use core::arch::asm;
use core::ptr;

use crate::{kprintf, gic};

// GIC Info and addresses from:
// http://web.archive.org/web/20230327162435/https://lowenware.com/blog/aarch64-gic-and-timer-interrupt/
pub struct Gic {
    gicd_ctlr: usize,
    gicc_ctlr: usize,
    gicr_ctlr: usize,
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

fn write_register(base: usize, field_width: u32, entry: u32, value: u32) {
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

fn read_register(base: usize, field_width: u32, entry: u32) -> u32 {
    let entries_per_register: u32 = 32 / field_width;
    let offset: usize = (entry / entries_per_register) as usize;
    let bit_offset: u32 = (entry % entries_per_register) * field_width;
    let field_mask: u32 = (1 << field_width) - 1;

    let register: *mut u32 = (base + offset) as *mut u32;
    let current_value: u32 = unsafe { ptr::read_volatile(register) };
    (current_value >> bit_offset) & field_mask
}

impl Gic {
    // Distributor Controller
    const GICD_CTLR_OFFSET: usize = 0x000;

    pub fn new(gicd_ctlr: usize, gicc_ctlr: usize, gicr_ctlr: usize) -> Self {
        Self {
            gicd_ctlr,
            gicc_ctlr,
            gicr_ctlr,
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
        // Turn on Group 0 and Group 1 interrupts
        ctlr_contents |= 1 << 0;
        ctlr_contents |= 1 << 1;

        // Disable Security
        ctlr_contents |= 1 << 6;

        // Disable affinity routing
        ctlr_contents &= !(1 << 4);

        unsafe {
            ptr::write_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32, ctlr_contents | 0b11);
        }

        ctlr_contents = unsafe {
            ptr::read_volatile((self.gicd_ctlr + Self::GICD_CTLR_OFFSET) as *mut u32)
        };

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
        kprintf!("ctlr_contents: {:#x}\n", ctlr_contents);

        // Enable SRE bypass for EL1. If we don't do this, we'll get a synchronous
        // exception when we try to write ICC_GRPEN1_EL1
        let sre_el1_contents: u64;
        unsafe {
            asm!(
                "mrs {0}, ICC_SRE_EL1",
                out(reg) sre_el1_contents,
            );
        }
        const SRE_BYPASS: u64 = 0b1;
        let sre_el1_desired: u64 = sre_el1_contents | SRE_BYPASS;
        unsafe {
            asm!(
                "msr ICC_SRE_EL1, {0}",
                in(reg) sre_el1_desired,
            );
        }


        // Section 4.2.1: Mark PE as awake
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

        // Get ICC_CTLR_EL1 contents and print them
        let icc_ctlr_el1_contents: u64;
        unsafe {
            asm!(
                "mrs {0}, ICC_CTLR_EL1",
                out(reg) icc_ctlr_el1_contents,
            );
        }

        let bpr0: u64;
        unsafe {
            asm!(
                "mrs {0}, ICC_BPR0_EL1",
                out(reg) bpr0,
            );
        }
        kprintf!("bpr0: {:#x}\n", bpr0);

        const PMR_MINIMUM_PRIORITY: u64 = 0xFF;
        unsafe {
            asm!("msr ICC_PMR_EL1, {0}", in(reg) PMR_MINIMUM_PRIORITY);
        }

        const GROUP_ENABLE: u64 = 0b1;
        unsafe {
            asm!(
                "msr ICC_IGRPEN1_EL1, {0}",
                "msr DAIFclr, 0x2",
                in(reg) GROUP_ENABLE,
            );
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

    pub fn set_redistributor_enable(&self, interrupt: u32) {
        const GICR_ISENABLER_FIELD_SIZE: u32 = 1;
        const GICR_ISENABLER_OFFSET: usize = 0x100;
        write_register(
            self.gicr_ctlr + GICR_ISENABLER_OFFSET,
            GICR_ISENABLER_FIELD_SIZE,
            interrupt,
            0b1,
        );
    }

    pub fn clear_redistributor_enable(&self, interrupt: u32) {
        const GICR_ICENABLER_FIELD_SIZE: u32 = 1;
        const GICR_ICENABLER_OFFSET: usize = 0x180;
        write_register(
            self.gicr_ctlr + GICR_ICENABLER_OFFSET,
            GICR_ICENABLER_FIELD_SIZE,
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

    pub fn set_redistributor_priority(&self, interrupt: u32, priority: u8) {
        const GICR_IPRIORITYR_OFFSET: usize = 0x400;
        const GICR_IPRIORITYR_FIELD_SIZE: u32 = 8;

        write_register(
            self.gicr_ctlr + GICR_IPRIORITYR_OFFSET,
            GICR_IPRIORITYR_FIELD_SIZE,
            interrupt,
            priority as u32,
        );
    }

    pub fn get_redistributor_priority(&self, interrupt: u32) -> u8 {
        const GICR_IPRIORITYR_OFFSET: usize = 0x400;
        const GICR_IPRIORITYR_FIELD_SIZE: u32 = 8;
        read_register(
            self.gicr_ctlr + GICR_IPRIORITYR_OFFSET,
            GICR_IPRIORITYR_FIELD_SIZE,
            interrupt,
        ) as u8
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
        const GICD_ICFGR_FIELD_SIZE: u32 = 16;
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
