use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::{Ax, Dx};
use crate::System;
use firn_arch_x86_macros::instr;

#[instr("IN AL, imm8")]
pub fn in_al_imm8(sys: &mut System, imm: u8) {
    let value = sys
        .port_in_8(imm as u16)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", imm));
    sys.cpu.set_reg_8(Al, value);
}

#[instr("IN AX, imm8")]
pub fn in_ax_imm8(sys: &mut System, imm: u8) {
    let value = sys
        .port_in_16(imm as u16)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", imm));
    sys.cpu.set_reg_16(Ax.into(), value);
}

#[instr("IN AL, DX")]
pub fn in_al_dx(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys
        .port_in_8(port)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", port));
    sys.cpu.set_reg_8(Al, value);
}

#[instr("IN AX, DX")]
pub fn in_ax_dx(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys
        .port_in_16(port)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", port));
    sys.cpu.set_reg_16(Ax.into(), value);
}

#[instr("OUT imm8, AL")]
pub fn out_imm8_al(sys: &mut System, imm: u8) {
    let value = sys.cpu.reg_8(Al);
    sys.port_out_8(imm as u16, value)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", imm));
}

#[instr("OUT imm8, AX")]
pub fn out_imm8_ax(sys: &mut System, imm: u8) {
    let value = sys.cpu.reg_16(Ax.into());
    sys.port_out_16(imm as u16, value)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", imm));
}

#[instr("OUT DX, AL")]
pub fn out_dx_al(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.cpu.reg_8(Al);
    sys.port_out_8(port, value)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", port));
}

#[instr("OUT DX, AX")]
pub fn out_dx_ax(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.cpu.reg_16(Ax.into());
    sys.port_out_16(port, value)
        .unwrap_or_else(|| panic!("unimplemented IO port: {:#x}", port));
}
