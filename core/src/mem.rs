use std::io;
use std::ops::{Index, IndexMut};
use std::path::Path;

pub mod basic;
pub mod eeprom;
pub mod map;
pub mod range;

pub use basic::BasicMem;
pub use eeprom::Eeprom;
pub use map::MemMap;
pub use range::MemRange;

#[derive(Copy, Clone)]
pub enum DumpRadix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

pub fn format_str_dump(radix: DumpRadix, iter: impl Iterator<Item = u8>) -> String {
    iter.map(|byte| match radix {
        DumpRadix::Binary => format!("{:08b}", byte),
        DumpRadix::Octal => format!("{:03o}", byte),
        DumpRadix::Decimal => format!("{:03}", byte),
        DumpRadix::Hexadecimal => format!("{:02x}", byte),
    })
    .collect::<Vec<String>>()
    .join(" ")
}

pub trait Mem: Index<usize, Output = u8> + IndexMut<usize> + Send + Sync {
    fn size(&self) -> usize;

    fn dump(&self) -> Vec<u8>;
    fn dump_to_str(&self, radix: DumpRadix) -> String;
    fn dump_to_file(&self, path: impl AsRef<Path>) -> io::Result<()>
    where
        Self: Sized;
}

macro_rules! basic_mem_impl {
    ($struct:ident, $vec_field:ident) => {
        impl $crate::mem::Mem for $struct {
            fn size(&self) -> usize {
                self.len()
            }

            fn dump(&self) -> Vec<u8> {
                self.$vec_field.clone()
            }

            fn dump_to_str(&self, radix: $crate::mem::DumpRadix) -> String {
                $crate::mem::format_str_dump(radix, self.iter().copied())
            }

            fn dump_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()>
            where
                Self: Sized,
            {
                std::fs::write(path, &self.$vec_field)
            }
        }
    };
}

pub(crate) use basic_mem_impl;
