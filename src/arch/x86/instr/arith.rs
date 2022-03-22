use crate::arch::x86::{Cpu, GeneralWordReg, RegMem};

pub fn xor_rm16_r16(cpu: &mut Cpu, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(cpu);
    let value = cpu.get_reg_16(reg.into());
    rm.set_16(cpu, old ^ value);
}
