use crate::System;

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

pub fn rol_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 7;
        sys.cpu.flags.carry = msb == 1;
        value = (value << 1) + msb;
    }

    if count == 1 {
        let msb = value >> 7;
        let carry = sys.cpu.flags.carry as u8;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn rol_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 15;
        sys.cpu.flags.carry = msb == 1;
        value = (value << 1) + msb;
    }

    if count == 1 {
        let msb = value >> 15;
        let carry = sys.cpu.flags.carry as u16;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn ror_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = (value >> 1) + (lsb * 2u8.pow(7));
    }

    if count == 1 {
        let lsb = value & 1;
        let next_msb = (value >> 6) & 1;
        sys.cpu.flags.overflow = lsb != next_msb;
    }

    value
}

pub fn ror_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = (value >> 1) + (lsb * 2u16.pow(15));
    }

    if count == 1 {
        let lsb = value & 1;
        let next_msb = (value >> 14) & 1;
        sys.cpu.flags.overflow = lsb != next_msb;
    }

    value
}

pub fn rcl_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 7;
        let carry = sys.cpu.flags.carry as u8;
        value = (value << 1) + carry;
        sys.cpu.flags.carry = msb == 1;
    }

    if count == 1 {
        let msb = value >> 7;
        let carry = sys.cpu.flags.carry as u8;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn rcl_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 15;
        let carry = sys.cpu.flags.carry as u16;
        value = (value << 1) + carry;
        sys.cpu.flags.carry = msb == 1;
    }

    if count == 1 {
        let msb = value >> 15;
        let carry = sys.cpu.flags.carry as u16;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn rcr_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        let carry = sys.cpu.flags.carry as u8;
        value = (value >> 1) + (carry * 2u8.pow(7));
        sys.cpu.flags.carry = lsb == 1;
    }

    if count == 1 {
        let msb = value >> 7;
        let next_msb = (value >> 6) & 1;
        sys.cpu.flags.overflow = msb != next_msb;
    }

    value
}

pub fn rcr_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        let carry = sys.cpu.flags.carry as u16;
        value = (value >> 1) + (carry * 2u16.pow(7));
        sys.cpu.flags.carry = lsb == 1;
    }

    if count == 1 {
        let msb = value >> 15;
        let next_msb = (value >> 14) & 1;
        sys.cpu.flags.overflow = msb != next_msb;
    }

    value
}

pub fn shl_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 7;
        sys.cpu.flags.carry = msb == 1;
        value <<= 1;
    }

    set_basic_flags_8(sys, value);
    if count == 1 {
        let msb = value >> 7;
        let carry = sys.cpu.flags.carry as u8;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn shl_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 15;
        sys.cpu.flags.carry = msb == 1;
        value <<= 1;
    }

    set_basic_flags_16(sys, value);
    if count == 1 {
        let msb = value >> 15;
        let carry = sys.cpu.flags.carry as u16;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

pub fn shr_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value >>= 1;
    }

    set_basic_flags_8(sys, value);
    if count == 1 {
        // TODO: Set OF
        // I have no idea what OF is supposed to be set to, the Am186 manual says it's set to
        // "temp" but it doesn't define any variable with that name...
    }

    value
}

pub fn shr_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value >>= 1;
    }

    set_basic_flags_16(sys, value);
    if count == 1 {
        // TODO: Set OF
        // I have no idea what OF is supposed to be set to, the Am186 manual says it's set to
        // "temp" but it doesn't define any variable with that name...
    }

    value
}

pub fn sar_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let start_msb = component & 0x80;
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = value >> 1 & start_msb;
    }

    set_basic_flags_8(sys, value);
    if count == 1 {
        sys.cpu.flags.overflow = false;
    }

    value
}

pub fn sar_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let start_msb = component & 0x8000;
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = value >> 1 & start_msb;
    }

    set_basic_flags_16(sys, value);
    if count == 1 {
        sys.cpu.flags.overflow = false;
    }

    value
}
