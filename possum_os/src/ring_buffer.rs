// SPDX-License-Identifier: GPL-3.0-only

use core::default::Default;
use core::marker::Copy;

pub struct RingBuffer<T: Copy + Default, const N: usize> {
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
