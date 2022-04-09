use crate::GeneralByteReg::Ah;
use crate::{ExtSystem, System};
use firn_arch_x86_macros::instr;

#[instr(POPF)]
pub fn popf(sys: &mut System) {
    let value = sys.pop_16();
    sys.cpu.flags.set_16(value);
}

#[instr(PUSHF)]
pub fn pushf(sys: &mut System) {
    let value = sys.cpu.flags.get_16();
    sys.push_16(value);
}

#[instr(SAHF)]
pub fn sahf(sys: &mut System) {
    let value = sys.cpu.reg_8(Ah);
    sys.cpu.flags.set_8(value);
}

#[instr(LAHF)]
pub fn lahf(sys: &mut System) {
    let value = sys.cpu.flags.get_8();
    sys.cpu.set_reg_8(Ah, value);
}

#[instr(CMC)]
pub fn cmc(sys: &mut System) {
    sys.cpu.flags.carry = !sys.cpu.flags.carry;
}

#[instr(CLC)]
pub fn clc(sys: &mut System) {
    sys.cpu.flags.carry = false;
}

#[instr(STC)]
pub fn stc(sys: &mut System) {
    sys.cpu.flags.carry = true;
}

#[instr(CLI)]
pub fn cli(sys: &mut System) {
    sys.cpu.flags.interrupt = false;
}

#[instr(STI)]
pub fn sti(sys: &mut System) {
    sys.cpu.flags.interrupt = true;
}

#[instr(CLD)]
pub fn cld(sys: &mut System) {
    sys.cpu.flags.direction = false;
}

#[instr(STD)]
pub fn std(sys: &mut System) {
    sys.cpu.flags.direction = true;
}
