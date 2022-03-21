use crate::num::Radix;
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::Path;

pub mod basic;
pub mod eeprom;
pub mod map;
pub mod range;

pub use basic::BasicMem;
pub use eeprom::{Eeprom, DEFAULT_BIOS};
pub use map::MemMap;
pub use range::MemRange;

pub fn format_str_dump(radix: Radix, iter: impl Iterator<Item = u8>) -> String {
    iter.map(|byte| match radix {
        Radix::Binary => format!("{:08b}", byte),
        Radix::Octal => format!("{:03o}", byte),
        Radix::Decimal => format!("{:03}", byte),
        Radix::Hexadecimal => format!("{:02x}", byte),
    })
    .collect::<Vec<String>>()
    .join(" ")
}

pub trait Mem: Deref<Target = [u8]> + DerefMut {
    fn size(&self) -> usize;
}

pub trait MemDump {
    fn dump_to_str(&self, radix: Radix) -> String;
    fn dump_to_file(&self, path: impl AsRef<Path>) -> io::Result<()>;
}
