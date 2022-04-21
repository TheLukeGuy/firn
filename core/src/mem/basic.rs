use crate::mem::basic_mem_impl;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct BasicMem {
    memory: Vec<u8>,
}

impl BasicMem {
    pub fn new(size: usize) -> Self {
        let mut memory = Vec::new();
        memory.resize(size, 0);

        Self { memory }
    }
}

basic_mem_impl!(BasicMem, memory);

impl Deref for BasicMem {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

impl DerefMut for BasicMem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}

impl Index<usize> for BasicMem {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for BasicMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_and_write_bytes() {
        let mut memory = BasicMem::new(16);
        memory[6] = 145;
        assert_eq!(145, memory[6]);
    }
}
