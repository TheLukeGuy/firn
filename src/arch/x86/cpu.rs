use crate::arch::x86::SegmentReg::{Cs, Ds, Es, Ss};
use crate::arch::x86::{
    Device, GeneralByteReg, Instr, IoInstr, PortMatchResult, SegmentReg, WordReg,
};
use crate::{cpu, System};
use std::io;
use std::io::Write;

pub struct Flags {
    pub carry: bool,
    pub parity: bool,
    pub adjust: bool,
    pub zero: bool,
    pub sign: bool,
    pub trap: bool,
    pub interrupt: bool,
    pub direction: bool,
    pub overflow: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            carry: false,
            parity: false,
            adjust: false,
            zero: false,
            sign: false,
            trap: false,
            interrupt: false,
            direction: false,
            overflow: false,
        }
    }

    pub fn set_parity_from_u8(&mut self, value: u8) {
        self.parity = value.count_ones() % 2 == 0;
    }

    pub fn set_parity_from_u16(&mut self, value: u16) {
        let lsb = (value & 0xff) as u8;
        self.set_parity_from_u8(lsb);
    }

    // TODO: Methods for setting AF

    pub fn set_zero_from_u8(&mut self, value: u8) {
        self.zero = value == 0;
    }

    pub fn set_zero_from_u16(&mut self, value: u16) {
        self.zero = value == 0;
    }

    pub fn set_sign_from_u8(&mut self, value: u8) {
        self.sign = (value as i8).is_negative();
    }

    pub fn set_sign_from_u16(&mut self, value: u16) {
        self.sign = (value as i16).is_negative();
    }

    pub fn set_pzs_from_u8(&mut self, value: u8) {
        self.set_parity_from_u8(value);
        self.set_zero_from_u8(value);
        self.set_sign_from_u8(value);
    }

    pub fn set_pzs_from_u16(&mut self, value: u16) {
        self.set_parity_from_u16(value);
        self.set_zero_from_u16(value);
        self.set_sign_from_u16(value);
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Cpu {
    pub system: System,
    devices: Vec<Box<dyn Device>>,

    regs: [u8; 2 * 8],
    segments: [u16; 4],
    pub flags: Flags,
    pub ip: u16,

    pub decoded: u64,
}

impl Cpu {
    pub fn new(system: System) -> Self {
        Self {
            system,
            devices: Vec::new(),

            regs: [0; 2 * 8],
            segments: [0; 4],
            flags: Flags::new(),
            ip: 0,

            decoded: 0,
        }
    }

    pub fn get_reg_8(&self, reg: GeneralByteReg) -> u8 {
        self.regs[reg as usize]
    }

    pub fn get_reg_16(&self, reg: WordReg) -> u16 {
        match reg {
            WordReg::General(reg) => {
                let low = self.regs[reg as usize];
                let high = self.regs[reg as usize + 4];

                u16::from_le_bytes([low, high])
            }
            WordReg::Segment(reg) => self.segments[reg as usize],
        }
    }

    pub fn set_reg_8(&mut self, reg: GeneralByteReg, value: u8) {
        self.regs[reg as usize] = value;
    }

    pub fn set_reg_16(&mut self, reg: WordReg, value: u16) {
        match reg {
            WordReg::General(reg) => {
                let [low, high] = value.to_le_bytes();

                self.regs[reg as usize] = low;
                self.regs[reg as usize + 4] = high;
            }
            WordReg::Segment(reg) => self.segments[reg as usize] = value,
        };
    }

    pub fn get_mem_8(&self, segment: SegmentReg, offset: u16) -> u8 {
        let linear = self.linear_mem(segment, offset);

        self.system.mem[linear]
    }

    pub fn get_mem_16(&self, segment: SegmentReg, offset: u16) -> u16 {
        let linear = self.linear_mem(segment, offset);

        let low = self.system.mem[linear];
        let high = self.system.mem[linear + 1];

        u16::from_le_bytes([low, high])
    }

    pub fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8) {
        let linear = self.linear_mem(segment, offset);
        self.system.mem[linear] = value;
    }

    pub fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16) {
        let linear = self.linear_mem(segment, offset);

        let [low, high] = value.to_le_bytes();
        self.system.mem[linear] = low;
        self.system.mem[linear + 1] = high;
    }

    pub fn read_mem_8(&mut self) -> u8 {
        let value = self.get_mem_8(Cs, self.ip);
        self.ip += 1;

        value
    }

    pub fn read_mem_16(&mut self) -> u16 {
        let value = self.get_mem_16(Cs, self.ip);
        self.ip += 2;

        value
    }

    pub fn add_device(&mut self, device: impl Device + 'static) {
        self.devices.push(Box::new(device));
    }

    pub fn match_port(&mut self, port: u16, instr: &mut IoInstr) {
        for device in &mut self.devices {
            if let PortMatchResult::Matched = device.match_port(port, instr) {
                return;
            }
        }

        println!("Unhandled port: {:#x} | {:?}", port, instr);
    }

    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize {
        let segment = self.get_reg_16(segment.into()) as usize;

        (segment << 4) + offset as usize
    }
}

impl cpu::Cpu for Cpu {
    fn init(&mut self) {
        for device in &mut self.devices {
            device.init();
        }
    }

    fn reset(&mut self) {
        self.set_reg_16(Cs.into(), 0xffff);
        self.set_reg_16(Ds.into(), 0x0000);
        self.set_reg_16(Es.into(), 0x0000);
        self.set_reg_16(Ss.into(), 0x0000);

        self.ip = 0;
    }

    fn step(&mut self) {
        for device in &mut self.devices {
            device.step();
        }

        print!("({:#x}) ", self.linear_mem(Cs, self.ip));
        io::stdout().flush().unwrap();

        let instr = Instr::decode(self);
        self.decoded += 1;
        println!("({:02}) Decoded: {:?}", self.decoded, instr);
        instr.execute(self);
    }
}
