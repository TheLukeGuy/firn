use crate::GeneralWordReg::Sp;
use crate::SegmentReg::{Cs, Ss};
use crate::{Cpu, GeneralByteReg, GeneralWordReg, SegmentReg, WordReg};

pub type System = firn_core::System<Cpu>;

pub trait ExtSystem {
    fn mem_linear_8(&self, address: usize) -> u8;
    fn mem_linear_16(&self, address: usize) -> u16;

    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize;

    fn mem_8(&self, segment: SegmentReg, offset: u16) -> u8;
    fn mem_16(&self, segment: SegmentReg, offset: u16) -> u16;

    fn mem_reg_8(&self, segment: SegmentReg, offset: GeneralWordReg) -> u8;
    fn mem_reg_16(&self, segment: SegmentReg, offset: GeneralWordReg) -> u16;

    fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8);
    fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16);

    fn set_mem_reg_8(&mut self, segment: SegmentReg, offset: GeneralWordReg, value: u8);
    fn set_mem_reg_16(&mut self, segment: SegmentReg, offset: GeneralWordReg, value: u16);

    fn peek_mem_8(&mut self) -> u8;
    fn peek_mem_16(&mut self) -> u16;

    fn read_mem_8(&mut self) -> u8;
    fn read_mem_16(&mut self) -> u16;

    fn push_8(&mut self, value: u8);
    fn push_16(&mut self, value: u16);

    fn push_reg_8(&mut self, reg: GeneralByteReg);
    fn push_reg_16(&mut self, reg: WordReg);

    fn pop_8(&mut self) -> u8;
    fn pop_16(&mut self) -> u16;

    fn pop_reg_8(&mut self, reg: GeneralByteReg);
    fn pop_reg_16(&mut self, reg: WordReg);

    fn interrupt(&mut self, interrupt: u8);
}

impl ExtSystem for System {
    fn mem_linear_8(&self, address: usize) -> u8 {
        self.mem[address]
    }

    fn mem_linear_16(&self, address: usize) -> u16 {
        let low = self.mem[address];
        let high = self.mem[address.wrapping_add(1)];

        u16::from_le_bytes([low, high])
    }

    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize {
        let segment = self.cpu.reg_16(segment.into()) as usize;

        (segment << 4).wrapping_add(offset as usize)
    }

    fn mem_8(&self, segment: SegmentReg, offset: u16) -> u8 {
        let linear = self.linear_mem(segment, offset);

        self.mem_linear_8(linear)
    }

    fn mem_16(&self, segment: SegmentReg, offset: u16) -> u16 {
        let linear = self.linear_mem(segment, offset);

        self.mem_linear_16(linear)
    }

    fn mem_reg_8(&self, segment: SegmentReg, offset: GeneralWordReg) -> u8 {
        let offset = self.cpu.reg_16(offset.into());
        self.mem_8(segment, offset)
    }

    fn mem_reg_16(&self, segment: SegmentReg, offset: GeneralWordReg) -> u16 {
        let offset = self.cpu.reg_16(offset.into());
        self.mem_16(segment, offset)
    }

    fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8) {
        let linear = self.linear_mem(segment, offset);
        self.mem[linear] = value;
    }

    fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16) {
        let linear = self.linear_mem(segment, offset);

        let [low, high] = value.to_le_bytes();
        self.mem[linear] = low;
        self.mem[linear.wrapping_add(1)] = high;
    }

    fn set_mem_reg_8(&mut self, segment: SegmentReg, offset: GeneralWordReg, value: u8) {
        let offset = self.cpu.reg_16(offset.into());
        self.set_mem_8(segment, offset, value);
    }

    fn set_mem_reg_16(&mut self, segment: SegmentReg, offset: GeneralWordReg, value: u16) {
        let offset = self.cpu.reg_16(offset.into());
        self.set_mem_16(segment, offset, value);
    }

    fn peek_mem_8(&mut self) -> u8 {
        self.mem_8(Cs, self.cpu.ip)
    }

    fn peek_mem_16(&mut self) -> u16 {
        self.mem_16(Cs, self.cpu.ip)
    }

    fn read_mem_8(&mut self) -> u8 {
        let value = self.peek_mem_8();
        self.cpu.inc_ip_16(1);

        value
    }

    fn read_mem_16(&mut self) -> u16 {
        let value = self.peek_mem_16();
        self.cpu.inc_ip_16(2);

        value
    }

    fn push_8(&mut self, value: u8) {
        let sp = self.cpu.reg_16(Sp.into()).wrapping_sub(1);
        self.cpu.set_reg_16(Sp.into(), sp);
        self.set_mem_8(Ss, sp, value);
    }

    fn push_16(&mut self, value: u16) {
        let sp = self.cpu.reg_16(Sp.into()).wrapping_sub(2);
        self.cpu.set_reg_16(Sp.into(), sp);
        self.set_mem_16(Ss, sp, value);
    }

    fn push_reg_8(&mut self, reg: GeneralByteReg) {
        let value = self.cpu.reg_8(reg);
        self.push_8(value);
    }

    fn push_reg_16(&mut self, reg: WordReg) {
        let value = self.cpu.reg_16(reg);
        self.push_16(value);
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

    fn pop_reg_8(&mut self, reg: GeneralByteReg) {
        let value = self.pop_8();
        self.cpu.set_reg_8(reg, value);
    }

    fn pop_reg_16(&mut self, reg: WordReg) {
        let value = self.pop_16();
        self.cpu.set_reg_16(reg, value);
    }

    fn interrupt(&mut self, interrupt: u8) {
        let flags = self.cpu.flags.get_16();
        self.push_16(flags);

        self.cpu.flags.interrupt = false;
        self.cpu.flags.trap = false;

        let cs = self.cpu.reg_16(Cs.into());
        self.push_16(cs);
        self.push_16(self.cpu.ip);

        let ivt_element = (interrupt as usize) << 2;
        let new_ip = self.mem_linear_16(ivt_element);
        let new_cs = self.mem_linear_16(ivt_element + 2);

        self.cpu.ip = new_ip;
        self.cpu.set_reg_16(Cs.into(), new_cs);
    }
}
