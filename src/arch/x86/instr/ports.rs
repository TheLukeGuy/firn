use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::{Cpu, Port};

pub fn out_imm8_al(cpu: &mut Cpu, imm: u8) {
    let value = cpu.get_reg_8(Al);
    let port = Port::Out8 {
        port: imm as u16,
        value,
    };
    cpu.match_port(port);
}

pub fn in_al_imm8(cpu: &mut Cpu, imm: u8) {
    let value = 0;
    let port = Port::In8 {
        port: imm as u16,
        value: &value,
    };
    cpu.match_port(port);
    cpu.set_reg_8(Al, value);
}
