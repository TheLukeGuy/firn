use crate::mem;
use crate::mem::{DumpRadix, Mem, MemDump, MemRange};
use linked_hash_map::LinkedHashMap;
use std::fs;
use std::ops::{Index, IndexMut};
use std::path::Path;

pub struct MemMap {
    pub addressable: usize,
    mappings: LinkedHashMap<MemRange, Box<dyn Mem>>,
}

impl MemMap {
    pub fn new(addressable: usize) -> Self {
        Self {
            addressable,
            mappings: LinkedHashMap::new(),
        }
    }

    pub fn map(&mut self, range: MemRange, memory: impl Mem + 'static) {
        if range.count() != memory.size() {
            panic!("range count must be the same as the memory size");
        }

        self.mappings.insert(range, Box::new(memory));
    }

    pub fn map_from(&mut self, start: usize, end: usize, memory: impl Mem + 'static) {
        let range = MemRange::new(start, end);
        self.map(range, memory);
    }

    pub fn map_full(&mut self, memory: impl Mem + 'static) {
        let range = MemRange::from_memory_full(&memory);
        self.map(range, memory);
    }

    fn map_index(&self, index: usize) -> Option<(MemRange, usize)> {
        if index > self.addressable {
            panic!(
                "index out of bounds: addressable memory size is {} but the index is {}",
                self.addressable, index
            );
        }

        for (range, memory) in self.mappings.iter().rev() {
            if !range.contains(index) {
                continue;
            }

            let padding_size = range.count() - memory.size();
            let mapped_index = index - range.start() - padding_size;
            return Some((*range, mapped_index));
        }

        None
    }
}

impl MemDump for MemMap {
    fn dump(&self) -> Vec<u8> {
        (0..self.addressable).map(|index| self[index]).collect()
    }

    fn dump_to_str(&self, radix: DumpRadix) -> String {
        mem::format_str_dump(radix, (0..self.addressable).map(|index| self[index]))
    }

    fn dump_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::write(path, self.dump())
    }
}

impl Index<usize> for MemMap {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let (key, mapped_index) = match self.map_index(index) {
            Some(mapped) => mapped,
            None => return &0,
        };

        let mapping = &self.mappings[&key];
        &mapping[mapped_index]
    }
}

impl IndexMut<usize> for MemMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (key, mapped_index) = match self.map_index(index) {
            Some(mapped) => mapped,
            None => panic!("cannot mutably index a memory address with no mapping"),
        };

        let mapping = &mut self.mappings[&key];
        &mut mapping[mapped_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::BasicMem;

    fn create_test_map() -> MemMap {
        // . | addressable memory          * | physical memory
        // - | addressable mapped memory   = | physical mapped memory
        // + | non-zero value for testing
        //
        // ................................................................
        // ******+*************************
        //                         ----------------
        //                         ===+============
        //
        // The value of a certain address should always be from the lowest layer in this diagram:
        // ******+*****************===+============........................

        let mut physical = BasicMem::new(32);
        physical[6] = 98;

        let mut mapped = BasicMem::new(16);
        mapped[3] = 46;

        let mut map = MemMap::new(64);
        map.map(MemRange::new(0, 31), physical);
        map.map(MemRange::new(24, 39), mapped);

        map
    }

    #[test]
    fn should_read_physical_values() {
        let map = create_test_map();
        assert_eq!(98, map[6]);
    }

    #[test]
    fn should_read_mapped_values() {
        let map = create_test_map();
        assert_eq!(46, map[27]);
    }

    #[test]
    fn should_read_zero_for_empty_values() {
        let map = create_test_map();
        assert_eq!(0, map[48]);
    }

    #[test]
    fn should_write_physical_values() {
        let mut map = create_test_map();
        map[13] = 114;
        assert_eq!(114, map[13]);
    }

    #[test]
    fn should_write_mapped_values() {
        let mut map = create_test_map();
        map[26] = 71;
        assert_eq!(71, map[26]);
    }
}
