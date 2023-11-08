// SPDX-License-Identifier: GPL-3.0-only
use core::default::Default;
use core::marker::Copy;
use core::str;

use crate::kprintf;
use crate::simple_uart::SimpleUart;

struct RingBuffer<T: Copy + Default, const N: usize> {
    buffer: [T; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [Default::default(); N],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count == N
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }

        self.buffer[self.tail] = item;
        self.tail = (self.tail + 1) % N;
        self.count += 1;

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.head];
        self.head = (self.head + 1) % N;
        self.count -= 1;

        Some(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = (self.tail + N - 1) % N;
        self.tail = index;
        self.count -= 1;

        Some(self.buffer[index])
    }

    pub fn flush(&mut self) -> ([T; N], usize) {
        let mut result = [Default::default(); N];
        let mut count = 0;

        while let Some(item) = self.dequeue() {
            result[count] = item;
            count += 1;
        }

        (result, count)
    }
}

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
            kprintf!(">");
            self.needs_start = false;
        }
        if !self.uart.empty() {
            let byte = self.uart.getc();
            if byte == 0x0D {
                kprintf!("\n");
                let (buffer, count) = self.buffer.flush();
                let input = str::from_utf8(&buffer[..count]).unwrap();
                let mut found = false;
                for command in COMMANDS.iter() {
                    if input.starts_with(command.command) {
                        let result = (command.callback)(&input[command.command.len()..]);
                        if result == 0 {
                            kprintf!("Command executed successfully\n");
                        } else {
                            kprintf!("Command failed with error code {}\n", result);
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
                        kprintf!("\x08 \x08"); // erase last character
                    }
                } else {
                    self.buffer.enqueue(byte).unwrap();
                    kprintf!("{}", byte as char);
                }
            }
        }
    }
}
