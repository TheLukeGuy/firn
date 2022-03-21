use crate::mem::Mem;
use std::fmt::{Debug, Formatter};
use std::ops::{Range, RangeInclusive};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct MemRange {
    start: usize,
    end: usize,
}

impl MemRange {
    pub fn new(start: usize, end: usize) -> Self {
        if start > end {
            panic!("memory range cannot have a higher start than end");
        }

        Self { start, end }
    }

    pub fn from_memory_full(memory: &impl Mem) -> Self {
        Self::new(0, memory.size() - 1)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn count(&self) -> usize {
        (self.end - self.start) + 1
    }

    pub fn contains(&self, address: usize) -> bool {
        address >= self.start && address <= self.end
    }
}

impl From<Range<usize>> for MemRange {
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start, range.end - 1)
    }
}

impl From<RangeInclusive<usize>> for MemRange {
    fn from(range: RangeInclusive<usize>) -> Self {
        let (start, end) = range.into_inner();
        Self::new(start, end)
    }
}

impl Debug for MemRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}..{:#x}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::BasicMem;

    fn create_test_range() -> MemRange {
        MemRange::new(0xf0000, 0xfffff)
    }

    #[test]
    fn should_correctly_construct_from_range() {
        let range = MemRange::from(8..16);
        assert_eq!((8, 15), (range.start(), range.end()));
    }

    #[test]
    fn should_correctly_construct_from_inclusive_range() {
        let range = MemRange::from(8..=15);
        assert_eq!((8, 15), (range.start(), range.end()));
    }

    #[test]
    fn should_correctly_construct_from_memory() {
        let memory = BasicMem::new(16);
        let range = MemRange::from_memory_full(&memory);
        assert_eq!((0, 15), (range.start(), range.end()));
    }

    #[test]
    fn should_calculate_count_correctly() {
        let range = create_test_range();
        assert_eq!(1024 * 64, range.count());
    }

    #[test]
    fn should_contain_valid_addresses() {
        let range = create_test_range();
        assert!(range.contains(0xfffff));
    }

    #[test]
    fn should_not_contain_lower_addresses() {
        let range = create_test_range();
        assert!(!range.contains(0xeffff));
    }

    #[test]
    fn should_not_contain_higher_addresses() {
        let range = create_test_range();
        assert!(!range.contains(0x100000));
    }
}
