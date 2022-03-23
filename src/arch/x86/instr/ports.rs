use crate::arch::x86::Cpu;
use crate::arch::x86::GeneralByteReg::Al;

pub fn out_imm8_al(cpu: &mut Cpu, imm: u8) {
    println!("OUT {:#x}, AL ({:#x})", imm, cpu.get_reg_8(Al));
}

pub fn in_al_imm8(cpu: &mut Cpu, imm: u8) {
    println!("IN AL, {:#x}", imm);
}
