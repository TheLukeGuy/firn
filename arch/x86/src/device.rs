pub mod cmos;

pub use cmos::Cmos;

#[derive(Debug)]
pub enum IoInstr<'a> {
    In8(&'a mut u8),
    Out8(u8),

    In16(&'a mut u16),
    Out16(u16),
}

pub enum PortMatchResult {
    Matched,
    Unknown,
}

pub trait Device {
    fn match_port(&mut self, port: u16, instr: &mut IoInstr) -> PortMatchResult;

    fn init(&mut self) {}
    fn step(&mut self) {}
}
