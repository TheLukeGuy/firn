use crate::Cpu;
use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Dx;

pub fn out_imm8_al(cpu: &mut Cpu, imm: u8) {
    let value = cpu.reg_8(Al);
    cpu.system.port_out_8(imm as u16, value);
}

pub fn in_al_imm8(cpu: &mut Cpu, imm: u8) {
    let value = cpu.system.port_in_8(imm as u16);
    cpu.set_reg_8(Al, value.unwrap_or(0));
}

pub fn in_al_dx(cpu: &mut Cpu) {
    let port = cpu.reg_16(Dx.into());
    let value = cpu.system.port_in_8(port);
    cpu.set_reg_8(Al, value.unwrap_or(0));
}
