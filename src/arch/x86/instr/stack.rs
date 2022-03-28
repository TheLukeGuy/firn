use crate::arch::x86::SegmentReg::{Ds, Es};
use crate::arch::x86::{Cpu, GeneralWordReg};

pub fn push_r16(cpu: &mut Cpu, reg: GeneralWordReg) {
    cpu.push_reg_16(reg.into());
}

pub fn push_ds(cpu: &mut Cpu) {
    cpu.push_reg_16(Ds.into());
}

pub fn pop_r16(cpu: &mut Cpu, reg: GeneralWordReg) {
    let value = cpu.pop_16();
    cpu.set_reg_16(reg.into(), value);
}

pub fn push_es(cpu: &mut Cpu) {
    cpu.push_reg_16(Es.into());
}

pub fn pop_es(cpu: &mut Cpu) {
    let value = cpu.pop_16();
    cpu.set_reg_16(Es.into(), value);
}
