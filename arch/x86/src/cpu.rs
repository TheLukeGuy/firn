use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{ExtSystem, Flags, GeneralByteReg, Instr, WordReg};
use firn_core::cpu::Restrict;
use firn_core::{cpu, System};

#[derive(Eq, PartialEq)]
pub enum Feature {
    InstrCpu1,
}

pub struct Cpu {
    features: Vec<Feature>,

    regs: [u8; 2 * 8],
    segments: [u16; 4],
    pub flags: Flags,
    pub ip: u16,

    pub decoded: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            features: Vec::new(),

            regs: [0; 2 * 8],
            segments: [0; 4],
            flags: Flags::new(),
            ip: 0,

            decoded: 0,
        }
    }

    pub fn reg_8(&self, reg: GeneralByteReg) -> u8 {
        self.regs[reg as usize]
    }

    pub fn reg_16(&self, reg: WordReg) -> u16 {
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

    pub fn inc_reg_8(&mut self, reg: GeneralByteReg, amount: u8) {
        let old = self.reg_8(reg);
        self.set_reg_8(reg, old.wrapping_add(amount));
    }

    pub fn inc_reg_16(&mut self, reg: WordReg, amount: u16) {
        let old = self.reg_16(reg);
        self.set_reg_16(reg, old.wrapping_add(amount));
    }

    pub fn dec_reg_8(&mut self, reg: GeneralByteReg, amount: u8) {
        let old = self.reg_8(reg);
        self.set_reg_8(reg, old.wrapping_sub(amount));
    }

    pub fn dec_reg_16(&mut self, reg: WordReg, amount: u16) {
        let old = self.reg_16(reg);
        self.set_reg_16(reg, old.wrapping_sub(amount));
    }

    pub fn inc_ip_8(&mut self, amount: u8) {
        let amount = amount as i8 as u16;
        self.ip = self.ip.wrapping_add(amount as u16);
    }

    pub fn inc_ip_16(&mut self, amount: u16) {
        self.ip = self.ip.wrapping_add(amount);
    }
}

impl cpu::Cpu for Cpu {
    fn reset(&mut self) {
        self.set_reg_16(Cs.into(), 0xffff);
        self.set_reg_16(Ds.into(), 0x0000);
        self.set_reg_16(Es.into(), 0x0000);
        self.set_reg_16(Ss.into(), 0x0000);

        self.ip = 0;
    }

    fn step(sys: &mut System<Self>) {
        let instr = Instr::decode(sys);
        sys.cpu.decoded += 1;

        let address = sys.linear_mem(Cs, sys.cpu.ip);
        println!(
            "({:#x}) ({}) [{:#04x}] Decoded: {}",
            address, sys.cpu.decoded, instr.opcode, instr.meta.mnemonic
        );

        instr.execute(sys);
    }
}

impl Restrict for Cpu {
    type Feature = Feature;

    fn add_feature(&mut self, feature: Self::Feature) {
        self.features.push(feature);
    }

    fn has_feature(&self, feature: Self::Feature) -> bool {
        self.features.contains(&feature)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
