use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Dx;
use crate::System;

pub fn out_imm8_al(sys: &mut System, imm: u8) {
    let value = sys.cpu.reg_8(Al);
    sys.port_out_8(imm as u16, value);
}

pub fn in_al_imm8(sys: &mut System, imm: u8) {
    let value = sys.port_in_8(imm as u16);
    sys.cpu.set_reg_8(Al, value.unwrap_or(0));
}

pub fn in_al_dx(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.port_in_8(port);
    sys.cpu.set_reg_8(Al, value.unwrap_or(0));
}
