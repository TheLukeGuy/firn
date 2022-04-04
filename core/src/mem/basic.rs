use crate::mem;
use crate::mem::{DumpRadix, Mem, MemDump};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, io};

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

impl Mem for BasicMem {
    fn size(&self) -> usize {
        self.memory.len()
    }
}

impl MemDump for BasicMem {
    fn dump(&self) -> Vec<u8> {
        self.memory.clone()
    }

    fn dump_to_str(&self, radix: DumpRadix) -> String {
        mem::format_str_dump(radix, (*self).iter().copied())
    }

    fn dump_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, &self.memory)
    }
}

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
