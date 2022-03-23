pub mod cmos;

pub use cmos::Cmos;

#[derive(Debug, Copy, Clone)]
pub enum Port<'a> {
    In8 { port: u16, value: &'a u8 },
    Out8 { port: u16, value: u8 },

    In16 { port: u16, value: &'a u16 },
    Out16 { port: u16, value: u16 },
}

pub enum PortMatchResult {
    Matched,
    Unknown,
}

pub trait Device {
    fn match_port(&mut self, port: Port) -> PortMatchResult;
}
