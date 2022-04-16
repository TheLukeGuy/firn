use crate::{arith, System};
use firn_arch_x86_macros::shift_instr;

shift_instr!(ROL);
shift_instr!(ROR);
shift_instr!(RCL);
shift_instr!(RCR);
shift_instr!(SHL);
shift_instr!(SHR);
shift_instr!(SAR);

fn rol_8(sys: &mut System, component: u8, count: u8) -> u8 {
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

fn rol_16(sys: &mut System, component: u16, count: u8) -> u16 {
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

fn ror_8(sys: &mut System, component: u8, count: u8) -> u8 {
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

fn ror_16(sys: &mut System, component: u16, count: u8) -> u16 {
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

fn rcl_8(sys: &mut System, component: u8, count: u8) -> u8 {
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

fn rcl_16(sys: &mut System, component: u16, count: u8) -> u16 {
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

fn rcr_8(sys: &mut System, component: u8, count: u8) -> u8 {
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

fn rcr_16(sys: &mut System, component: u16, count: u8) -> u16 {
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

fn shl_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 7;
        sys.cpu.flags.carry = msb == 1;
        value <<= 1;
    }

    arith::set_basic_flags_8(sys, value);
    if count == 1 {
        let msb = value >> 7;
        let carry = sys.cpu.flags.carry as u8;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

fn shl_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let msb = value >> 15;
        sys.cpu.flags.carry = msb == 1;
        value <<= 1;
    }

    arith::set_basic_flags_16(sys, value);
    if count == 1 {
        let msb = value >> 15;
        let carry = sys.cpu.flags.carry as u16;
        sys.cpu.flags.overflow = msb != carry;
    }

    value
}

fn shr_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value >>= 1;
    }

    arith::set_basic_flags_8(sys, value);
    if count == 1 {
        // TODO: Set OF
        // I have no idea what OF is supposed to be set to, the Am186 manual says it's set to
        // "temp" but it doesn't define any variable with that name...
    }

    value
}

fn shr_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value >>= 1;
    }

    arith::set_basic_flags_16(sys, value);
    if count == 1 {
        // TODO: Set OF
        // I have no idea what OF is supposed to be set to, the Am186 manual says it's set to
        // "temp" but it doesn't define any variable with that name...
    }

    value
}

fn sar_8(sys: &mut System, component: u8, count: u8) -> u8 {
    let start_msb = component & 0x80;
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = value >> 1 & start_msb;
    }

    arith::set_basic_flags_8(sys, value);
    if count == 1 {
        sys.cpu.flags.overflow = false;
    }

    value
}

fn sar_16(sys: &mut System, component: u16, count: u8) -> u16 {
    let start_msb = component & 0x8000;
    let mut value = component;
    for _ in 1..=count {
        let lsb = value & 1;
        sys.cpu.flags.carry = lsb == 1;
        value = value >> 1 & start_msb;
    }

    arith::set_basic_flags_16(sys, value);
    if count == 1 {
        sys.cpu.flags.overflow = false;
    }

    value
}
