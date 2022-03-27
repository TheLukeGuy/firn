use crate::arch::x86::GeneralWordReg::{Ax, Cx, Di, Si};
use crate::arch::x86::SegmentReg::Es;
use crate::arch::x86::{instr, Cpu};

pub fn stosw(cpu: &mut Cpu, rep: bool) {
    for offset in instr::rep_16(rep, || cpu.get_reg_16(Cx.into())) {
        let di = cpu.get_reg_16(Di.into());
        let value = cpu.get_reg_16(Ax.into());
        cpu.set_mem_16(Es, di + offset, value);

        if cpu.flags.direction {
            cpu.dec_reg_16(Si.into(), 2);
            cpu.dec_reg_16(Di.into(), 2);
        } else {
            cpu.inc_reg_16(Si.into(), 2);
            cpu.inc_reg_16(Di.into(), 2);
        }
    }
}
