use crate::System;

pub fn add_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i8).overflowing_add(right as i8);
    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

pub fn add_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);
    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

pub fn adc_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let cf = sys.cpu.flags.carry as u8;

    let (value, first_overflow) = left.overflowing_add(right);
    let (value, second_overflow) = value.overflowing_add(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i8).overflowing_add(right as i8);
    let (_, second_overflow) = signed_value.overflowing_add(cf as i8);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

pub fn adc_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let cf = sys.cpu.flags.carry as u16;

    let (value, first_overflow) = left.overflowing_add(right);
    let (value, second_overflow) = value.overflowing_add(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i16).overflowing_add(right as i16);
    let (_, second_overflow) = signed_value.overflowing_add(cf as i16);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

pub fn sub_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i8).overflowing_sub(right as i8);
    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

pub fn sub_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i16).overflowing_sub(right as i16);
    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

pub fn sbb_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let cf = sys.cpu.flags.carry as u8;

    let (value, first_overflow) = left.overflowing_sub(right);
    let (value, second_overflow) = value.overflowing_sub(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i8).overflowing_sub(right as i8);
    let (_, second_overflow) = signed_value.overflowing_sub(cf as i8);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

pub fn sbb_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let cf = sys.cpu.flags.carry as u16;

    let (value, first_overflow) = left.overflowing_sub(right);
    let (value, second_overflow) = value.overflowing_sub(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i16).overflowing_sub(right as i16);
    let (_, second_overflow) = signed_value.overflowing_sub(cf as i16);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

pub fn or_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left | right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn or_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left | right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn and_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left & right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn and_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left & right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn xor_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left ^ right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn xor_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left ^ right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

pub fn set_basic_flags_8(sys: &mut System, value: u8) {
    sys.cpu.flags.set_parity_from_u8(value);
    sys.cpu.flags.set_zero_from_u8(value);
    sys.cpu.flags.set_sign_from_u8(value);
}

pub fn set_basic_flags_16(sys: &mut System, value: u16) {
    sys.cpu.flags.set_parity_from_u16(value);
    sys.cpu.flags.set_zero_from_u16(value);
    sys.cpu.flags.set_sign_from_u16(value);
}

pub fn set_all_flags_8(sys: &mut System, value: u8, overflow: bool, signed_overflow: bool) {
    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

pub fn set_all_flags_16(sys: &mut System, value: u16, overflow: bool, signed_overflow: bool) {
    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}
