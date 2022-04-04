use crate::GeneralWordReg::Sp;
use crate::SegmentReg::{Cs, Ss};
use crate::{Cpu, GeneralByteReg, SegmentReg, WordReg};
use firn_core::System;

pub trait ExtSystem {
    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize;

    fn mem_8(&self, segment: SegmentReg, offset: u16) -> u8;
    fn mem_16(&self, segment: SegmentReg, offset: u16) -> u16;

    fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8);
    fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16);

    fn read_mem_8(&mut self) -> u8;
    fn read_mem_16(&mut self) -> u16;

    fn push_8(&mut self, value: u8);
    fn push_16(&mut self, value: u16);

    fn pop_8(&mut self) -> u8;
    fn pop_16(&mut self) -> u16;

    fn push_reg_8(&mut self, reg: GeneralByteReg);
    fn push_reg_16(&mut self, reg: WordReg);
}

impl ExtSystem for System<Cpu> {
    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize {
        let segment = self.cpu.reg_16(segment.into()) as usize;

        (segment << 4) + offset as usize
    }

    fn mem_8(&self, segment: SegmentReg, offset: u16) -> u8 {
        let linear = self.linear_mem(segment, offset);

        self.mem[linear]
    }

    fn mem_16(&self, segment: SegmentReg, offset: u16) -> u16 {
        let linear = self.linear_mem(segment, offset);

        let low = self.mem[linear];
        let high = self.mem[linear + 1];

        u16::from_le_bytes([low, high])
    }

    fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8) {
        let linear = self.linear_mem(segment, offset);
        self.mem[linear] = value;
    }

    fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16) {
        let linear = self.linear_mem(segment, offset);

        let [low, high] = value.to_le_bytes();
        self.mem[linear] = low;
        self.mem[linear + 1] = high;
    }

    fn read_mem_8(&mut self) -> u8 {
        let value = self.mem_8(Cs, self.cpu.ip);
        self.cpu.ip += 1;

        value
    }

    fn read_mem_16(&mut self) -> u16 {
        let value = self.mem_16(Cs, self.cpu.ip);
        self.cpu.ip += 2;

        value
    }

    fn push_8(&mut self, value: u8) {
        let sp = self.cpu.reg_16(Sp.into()) - 1;
        self.cpu.set_reg_16(Sp.into(), sp);
        self.set_mem_8(Ss, sp, value);
    }

    fn push_16(&mut self, value: u16) {
        let sp = self.cpu.reg_16(Sp.into()) - 2;
        self.cpu.set_reg_16(Sp.into(), sp);
        self.set_mem_16(Ss, sp, value);
    }

    fn pop_8(&mut self) -> u8 {
        let sp = self.cpu.reg_16(Sp.into());
        let value = self.mem_8(Ss, sp);
        self.cpu.inc_reg_16(Sp.into(), 1);

        value
    }

    fn pop_16(&mut self) -> u16 {
        let sp = self.cpu.reg_16(Sp.into());
        let value = self.mem_16(Ss, sp);
        self.cpu.inc_reg_16(Sp.into(), 2);

        value
    }

    fn push_reg_8(&mut self, reg: GeneralByteReg) {
        let value = self.cpu.reg_8(reg);
        self.push_8(value);
    }

    fn push_reg_16(&mut self, reg: WordReg) {
        let value = self.cpu.reg_16(reg);
        self.push_16(value);
    }
}
