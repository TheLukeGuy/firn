use std::io;
use std::ops::{Deref, DerefMut};
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

pub trait Mem: Deref<Target = [u8]> + DerefMut {
    fn size(&self) -> usize;
}

pub trait MemDump {
    fn dump_to_str(&self, radix: DumpRadix) -> String;
    fn dump_to_file(&self, path: impl AsRef<Path>) -> io::Result<()>;
}
