use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::{Ax, Di, Si};
use crate::SegmentReg::Es;
use crate::{ExtSystem, System};
use firn_arch_x86_macros::instr;

#[instr(STOSW)]
pub fn stosw(sys: &mut System) {
    for offset in /* TODO: rep */ 0..0 {
        let di = sys.cpu.reg_16(Di.into());
        let value = sys.cpu.reg_16(Ax.into());
        sys.set_mem_16(Es, di.wrapping_add(offset), value);

        if sys.cpu.flags.direction {
            sys.cpu.dec_reg_16(Si.into(), 2);
            sys.cpu.dec_reg_16(Di.into(), 2);
        } else {
            sys.cpu.inc_reg_16(Si.into(), 2);
            sys.cpu.inc_reg_16(Di.into(), 2);
        }
    }
}

#[instr(STOSB)]
pub fn stosb(sys: &mut System) {
    for offset in /* TODO: rep */ 0..0 {
        let di = sys.cpu.reg_16(Di.into());
        let value = sys.cpu.reg_8(Al);
        sys.set_mem_8(Es, di.wrapping_add(offset), value);

        if sys.cpu.flags.direction {
            sys.cpu.dec_reg_16(Si.into(), 1);
            sys.cpu.dec_reg_16(Di.into(), 1);
        } else {
            sys.cpu.inc_reg_16(Si.into(), 1);
            sys.cpu.inc_reg_16(Di.into(), 1);
        }
    }
}
