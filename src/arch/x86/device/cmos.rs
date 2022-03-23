use crate::arch::x86::{Device, Port, PortMatchResult};

pub struct Cmos {}

impl Cmos {
    pub fn new() -> Self {
        Self {}
    }
}

// This is probably temporary, the constructor will likely take in parameters
impl Default for Cmos {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for Cmos {
    fn match_port(&mut self, port: Port) -> PortMatchResult {
        match port {
            Port::Out8 { port: 0x70, value } => {
                println!("Selected CMOS register: {:#x}", value);
                PortMatchResult::Matched
            }
            Port::In8 {
                port: 0x71,
                value: _,
            } => {
                println!("Read CMOS register");
                PortMatchResult::Matched
            }
            Port::Out8 { port: 0x71, value } => {
                println!("Wrote to CMOS register: {:#x}", value);
                PortMatchResult::Matched
            }

            _ => PortMatchResult::Unknown,
        }
    }
}
