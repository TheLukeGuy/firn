use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::GeneralWordReg::Dx;
use crate::arch::x86::{Cpu, IoInstr};

pub fn out_imm8_al(cpu: &mut Cpu, imm: u8) {
    let value = cpu.reg_8(Al);
    let mut instr = IoInstr::Out8(value);
    cpu.match_port(imm as u16, &mut instr);
}

pub fn in_al_imm8(cpu: &mut Cpu, imm: u8) {
    let mut value = 0;
    let mut instr = IoInstr::In8(&mut value);
    cpu.match_port(imm as u16, &mut instr);
    cpu.set_reg_8(Al, value);
}

pub fn in_al_dx(cpu: &mut Cpu) {
    let port = cpu.reg_16(Dx.into());
    let mut value = 0;
    let mut instr = IoInstr::In8(&mut value);
    cpu.match_port(port, &mut instr);
    cpu.set_reg_8(Al, value);
}
