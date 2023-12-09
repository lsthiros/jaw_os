// SPDX-License-Identifier: GPL-3.0-only
use core::str;
use core::arch::asm;

use crate::ring_buffer::RingBuffer;
use crate::simple_uart::SimpleUart;
use crate::gic::Gic;
use crate::gic::InterruptType;
use crate::exception;
use crate::{kprintf, uart_printf};

// Console struct that contains a SimpleUart and a RingBuffer
pub struct Console {
    uart: SimpleUart,
    buffer: RingBuffer<u8, 1024>,
    needs_start: bool,
}

// Define a ConsoleCallback type thats a function pointer that takes a &str and return u8
pub type ConsoleCallback = fn(&str) -> u8;

// Define a ConsoleCommand struct that contains a &str and a ConsoleCallback
pub struct ConsoleCommand {
    command: &'static str,
    callback: ConsoleCallback,
}

fn echo(input: &str) -> u8 {
    kprintf!("{}\n", input);
    0
}

fn device_tree(_: &str) -> u8 {
    let dt = crate::device_tree::device_tree_from_ram_ptr(
        crate::device_tree::QEMU_DEVICE_TREE_OFSET as *const u8,
    );
    kprintf!("Device Tree:{:#?}\n", dt);
    dt.print_structure();
    0
}

fn interrupt_test(_: &str) -> u8 {
    const TIMER_IRQ: u32 = 30;

    exception::init_exception_table();
    let current_el: exception::ExceptionLevel = exception::get_current_el();
    kprintf!("Current exception level: {:?}\n", current_el);

    let gic = Gic::new(0x0800_0000 as usize, 0x0801_0000 as usize, 0x080A_0000 as usize);
    kprintf!("Init GIC\n");
    gic.init_gic();
    // Set the timer interrupt to be level sensitive with set_cfg
    kprintf!("Set cfg\n");

    gic.set_redistributor_priority(TIMER_IRQ, 0);
    gic.set_group(TIMER_IRQ, true);
    gic.set_cfg(TIMER_IRQ, InterruptType::LevelSensitive);
    gic.clear_pending(TIMER_IRQ);
    gic.set_enable(TIMER_IRQ);

    kprintf!("Set timer\n");

    let freq_val: u64;
    let ctl_val: u64 = 1;
    let next: u64;
    let delta: u64 = 100_000_000;
    unsafe {
        asm!(
            "mrs {0}, CNTFRQ_EL0",
            "msr CNTP_CTL_EL0, {1}",
            "msr CNTP_TVAL_EL0, {3}",
            "mrs {2}, CNTP_CVAL_EL0",
            out(reg) freq_val,
            in(reg) ctl_val,
            out(reg) next,
            in(reg) delta,
        );
    }

    // kprintf frequency and tick values as hex
    kprintf!("freq_val: {:#x}\n next: {:#x}\n", freq_val, next);

    loop {
        let mut nop_cnt: u64 = 0;
        while nop_cnt < 20000000 {
            unsafe {
                asm!("nop");
            }
            nop_cnt += 1;
        }
        let cntpct_val: u64;
        let timer_ctl: u64;
        let remaining: u64;
        unsafe {
            asm!(
                "mrs {0}, CNTPCT_EL0",
                "mrs {1}, CNTP_CTL_EL0",
                "mrs {2}, CNTP_TVAL_EL0",
                out(reg) cntpct_val,
                out(reg) timer_ctl,
                out(reg) remaining,
            );
        }
        kprintf!(
            "cntpct_val: {:#x} ctl: {:#x} remain:{:#x}",
            cntpct_val,
            timer_ctl,
            remaining
        );
        let pending: u64 = gic.get_pending(TIMER_IRQ) as u64;
        kprintf!(" pending: {:#x}\n", pending);
        if (pending != 0) {
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
        }

        if (remaining > delta) {
            kprintf!("remaining > delta. Setting interrupt manually and hoping for the best\n");
            gic.set_pending(TIMER_IRQ);
            gic.set_redistributor_pending(TIMER_IRQ);
            kprintf!("good luck, us :)\n");
            let val: u32 = gic.get_pending(TIMER_IRQ) as u32;
            let other_val: u32 = gic.get_redistributor_pending(TIMER_IRQ) as u32;
            kprintf!("val: {:#x} other_val {:#x}\n", val, other_val);
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
        }
    }
    0
}

// Static array of ConsoleCommands
static COMMANDS: [ConsoleCommand; 3] = [
    ConsoleCommand {
        command: "echo",
        callback: echo,
    },
    ConsoleCommand {
        command: "dt",
        callback: device_tree,
    },
    ConsoleCommand {
        command: "int",
        callback: interrupt_test,
    },
];

impl Console {
    pub fn new() -> Self {
        Self {
            uart: SimpleUart::new(0x0900_0000 as *mut u8),
            buffer: RingBuffer::new(),
            needs_start: true,
        }
    }

    pub fn service(&mut self) {
        if self.needs_start {
            self.uart.putc(b'>');
            self.needs_start = false;
        }
        if !self.uart.empty() {
            let byte = self.uart.getc();
            if byte == 0x0D {
                self.uart.putc(b'\n');
                let (buffer, count) = self.buffer.flush();
                let input = str::from_utf8(&buffer[..count]).unwrap();
                let mut found = false;
                for command in COMMANDS.iter() {
                    if input.starts_with(command.command) {
                        let result = (command.callback)(&input[command.command.len()..]);
                        if result == 0 {
                            self.uart.puts("Command executed successfully\n");
                        } else {
                            uart_printf!(self.uart, "Command failed with error code {}\n", result);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    kprintf!("Command not found\n");
                }
                self.needs_start = true;
            } else {
                if byte == 0x7F || byte == 0x08 {
                    // backspace
                    if !self.buffer.is_empty() {
                        _ = self.buffer.pop();
                        self.uart.puts("\x08 \x08"); // erase last character
                    }
                } else {
                    self.buffer.enqueue(byte).unwrap();
                    self.uart.putc(byte);
                }
            }
        }
    }
}
