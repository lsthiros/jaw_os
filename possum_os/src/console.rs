// SPDX-License-Identifier: GPL-3.0-only

pub struct Fifo<const N: usize> {
    buffer: [u8; N],
    tail: usize,
}

impl<const N: usize> Fifo<N> {
    pub fn new() -> Self {
        Fifo {
            buffer: [0; N],
            tail: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tail == 0
    }

    pub fn is_full(&self) -> bool {
        self.tail == N
    }

    pub fn push(&mut self, data: u8) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Fifo is full");
        }

        self.buffer[self.tail] = data;
        self.tail += 1;

        Ok(())
    }

    pub fn flush(&mut self) -> [u8; N] {
        let mut flushed_data = [0; N];

        for (i, data) in self.buffer.iter().enumerate() {
            flushed_data[i] = *data;
        }

        self.tail = 0;

        flushed_data
    }
}
