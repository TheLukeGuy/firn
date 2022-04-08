use crate::GeneralByteReg::Al;
use crate::SegmentReg::Es;
use crate::{ExtSystem, GeneralByteReg, GeneralWordReg, RegMem, RmPtr, SegmentReg, System};

pub fn mov_r8_imm8(sys: &mut System, reg: GeneralByteReg, imm: u8) {
    sys.cpu.set_reg_8(reg, imm);
}

pub fn mov_al_moffs8(sys: &mut System, segment: SegmentReg, offset: u16) {
    let value = sys.mem_8(segment, offset);
    sys.cpu.set_reg_8(Al, value);
}

pub fn mov_rm8_r8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let value = sys.cpu.reg_8(reg);
    rm.set_8(sys, value);
}

pub fn mov_r16_imm16(sys: &mut System, reg: GeneralWordReg, imm: u16) {
    sys.cpu.set_reg_16(reg.into(), imm);
}

pub fn mov_rm16_r16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let value = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);
}

pub fn mov_sreg_rm16(sys: &mut System, reg: SegmentReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

pub fn mov_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

pub fn les_r16_m16_16(sys: &mut System, reg: GeneralWordReg, mem: RmPtr) {
    // TODO: Ensure this is correct
    let (instr_segment, offset_addr) = mem.address(sys);
    let offset = sys.mem_16(instr_segment, offset_addr);
    let segment = sys.mem_16(instr_segment, offset_addr + 2);
    sys.cpu.set_reg_16(reg.into(), offset);
    sys.cpu.set_reg_16(Es.into(), segment);
}

pub fn mov_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let value = rm.get_8(sys);
    sys.cpu.set_reg_8(reg, value);
}
