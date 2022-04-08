use crate::System;

pub fn cli(sys: &mut System) {
    sys.cpu.flags.interrupt = false;
}

pub fn cld(sys: &mut System) {
    sys.cpu.flags.direction = false;
}
