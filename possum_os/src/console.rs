// SPDX-License-Identifier: GPL-3.0-only
use core::str;

use crate::simple_uart::SimpleUart;
use crate::ring_buffer::RingBuffer;
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

// Static array of ConsoleCommands
static COMMANDS: [ConsoleCommand; 1] = [ConsoleCommand {
    command: "echo",
    callback: echo,
}];

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
