use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::GeneralWordReg::{Ax, Di, Si};
use crate::arch::x86::SegmentReg::Es;
use crate::arch::x86::{instr, Cpu};

pub fn stosw(cpu: &mut Cpu, rep: bool) {
    for offset in instr::rep(cpu, rep) {
        let di = cpu.reg_16(Di.into());
        let value = cpu.reg_16(Ax.into());
        cpu.set_mem_16(Es, di.wrapping_add(offset), value);

        if cpu.flags.direction {
            cpu.dec_reg_16(Si.into(), 2);
            cpu.dec_reg_16(Di.into(), 2);
        } else {
            cpu.inc_reg_16(Si.into(), 2);
            cpu.inc_reg_16(Di.into(), 2);
        }
    }
}

pub fn stosb(cpu: &mut Cpu, rep: bool) {
    for offset in instr::rep(cpu, rep) {
        let di = cpu.reg_16(Di.into());
        let value = cpu.reg_8(Al);
        cpu.set_mem_8(Es, di.wrapping_add(offset), value);

        if cpu.flags.direction {
            cpu.dec_reg_16(Si.into(), 1);
            cpu.dec_reg_16(Di.into(), 1);
        } else {
            cpu.inc_reg_16(Si.into(), 1);
            cpu.inc_reg_16(Di.into(), 1);
        }
    }
}
