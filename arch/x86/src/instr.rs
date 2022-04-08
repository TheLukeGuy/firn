use crate::GeneralWordReg::Cx;
use crate::{
    opcodes, ExtSystem, GeneralByteReg, GeneralWordReg, Modrm, ModrmRegType, RegMem, RmPtr,
    SegmentReg, Size, System,
};
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub mod arith;
pub mod control;
pub mod flags;
pub mod ports;
pub mod stack;
pub mod strings;
pub mod transfer;

pub fn rep(sys: &mut System, rep: bool) -> Range<u16> {
    if rep {
        0..sys.cpu.reg_16(Cx.into())
    } else {
        0..1
    }
}

pub struct InstrFunc<F>(pub F);

impl<F> Debug for InstrFunc<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

macro_rules! instr_enum {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $(
                $variant_name:ident $( {
                    $( $value_name:ident : $value_type:ty ),* $(,)?
                } )?
            ),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        $vis enum $name {
            $(
                $variant_name {
                    func: InstrFunc<fn(sys: &mut System, $( $( $value_name: $value_type ),* )?)>,
                    $( $( $value_name: $value_type ),* )?
                }
            ),*
        }

        impl $name {
            pub fn execute(self, sys: &mut System) {
                match self {
                    $(
                        $name::$variant_name {
                            func,
                            $( $( $value_name ),* )?
                        } => func.0(sys, $( $( $value_name ),* )?)
                    ),*
                }
            }
        }
    }
}

instr_enum! {
    #[derive(Debug)]
    pub enum Instr {
        Basic,
        BasicRep {
            rep: bool,
        },
        Ptr16_16 {
            offset: u16,
            segment: u16,
        },
        R8Imm8 {
            reg: GeneralByteReg,
            imm: u8,
        },
        Moffs8 {
            segment: SegmentReg,
            offset: u16,
        },
        Imm8 {
            imm: u8,
        },
        R8Rm8 {
            reg: GeneralByteReg,
            rm: RegMem,
        },
        R16Imm16 {
            reg: GeneralWordReg,
            imm: u16,
        },
        SregRm16 {
            reg: SegmentReg,
            rm: RegMem,
        },
        Rm8Imm8 {
            rm: RegMem,
            imm: u8,
        },
        R16Rm16 {
            reg: GeneralWordReg,
            rm: RegMem,
        },
        Imm16Imm8 {
            first: u16,
            second: u8,
        },
        Imm16 {
            imm: u16,
        },
        R16 {
            reg: GeneralWordReg,
        },
        Rm16Imm8 {
            rm: RegMem,
            imm: u8,
        },
        R16M16 {
            reg: GeneralWordReg,
            mem: RmPtr,
        },
    }
}

impl Instr {
    pub fn decode(sys: &mut System) -> Self {
        opcodes::decode(sys)
    }

    pub fn new_basic(func: fn(sys: &mut System)) -> Self {
        Instr::Basic {
            func: InstrFunc(func),
        }
    }

    pub fn new_basic_rep(func: fn(sys: &mut System, rep: bool), rep: bool) -> Self {
        Instr::BasicRep {
            func: InstrFunc(func),
            rep,
        }
    }

    pub fn new_ptr16_16(
        func: fn(sys: &mut System, offset: u16, segment: u16),
        sys: &mut System,
    ) -> Self {
        Instr::Ptr16_16 {
            func: InstrFunc(func),
            offset: sys.read_mem_16(),
            segment: sys.read_mem_16(),
        }
    }

    pub fn new_r8_imm8(
        func: fn(sys: &mut System, reg: GeneralByteReg, imm: u8),
        sys: &mut System,
        reg: GeneralByteReg,
    ) -> Self {
        Instr::R8Imm8 {
            func: InstrFunc(func),
            reg,
            imm: sys.read_mem_8(),
        }
    }

    pub fn new_moffs8(
        func: fn(sys: &mut System, segment: SegmentReg, offset: u16),
        sys: &mut System,
        segment: SegmentReg,
    ) -> Self {
        Instr::Moffs8 {
            func: InstrFunc(func),
            segment,
            offset: sys.read_mem_16(),
        }
    }

    pub fn new_imm8(func: fn(sys: &mut System, imm: u8), sys: &mut System) -> Self {
        Instr::Imm8 {
            func: InstrFunc(func),
            imm: sys.read_mem_8(),
        }
    }

    pub fn new_r8_rm8(
        func: fn(sys: &mut System, reg: GeneralByteReg, rm: RegMem),
        sys: &mut System,
    ) -> Self {
        let modrm = Self::modrm_all_8(sys);
        Instr::R8Rm8 {
            func: InstrFunc(func),
            reg: modrm.byte_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_r16_imm16(
        func: fn(sys: &mut System, reg: GeneralWordReg, imm: u16),
        sys: &mut System,
        reg: GeneralWordReg,
    ) -> Self {
        Instr::R16Imm16 {
            func: InstrFunc(func),
            reg,
            imm: sys.read_mem_16(),
        }
    }

    pub fn new_sreg_rm16(
        func: fn(sys: &mut System, reg: SegmentReg, rm: RegMem),
        sys: &mut System,
    ) -> Self {
        let modrm = Self::modrm_segment_16(sys);
        Instr::SregRm16 {
            func: InstrFunc(func),
            reg: modrm.segment_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_rm8_imm8(func: fn(sys: &mut System, rm: RegMem, imm: u8), sys: &mut System) -> Self {
        let modrm = Self::modrm_all_8(sys);
        Instr::Rm8Imm8 {
            func: InstrFunc(func),
            rm: modrm.reg_mem,
            imm: sys.read_mem_8(),
        }
    }

    pub fn new_r16_rm16(
        func: fn(sys: &mut System, reg: GeneralWordReg, rm: RegMem),
        sys: &mut System,
    ) -> Self {
        let modrm = Self::modrm_all_16(sys);
        Instr::R16Rm16 {
            func: InstrFunc(func),
            reg: modrm.word_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_imm16_imm8(
        func: fn(sys: &mut System, first: u16, second: u8),
        sys: &mut System,
    ) -> Self {
        Instr::Imm16Imm8 {
            func: InstrFunc(func),
            first: sys.read_mem_16(),
            second: sys.read_mem_8(),
        }
    }

    pub fn new_imm16(func: fn(sys: &mut System, imm: u16), sys: &mut System) -> Self {
        Instr::Imm16 {
            func: InstrFunc(func),
            imm: sys.read_mem_16(),
        }
    }

    pub fn new_r16(func: fn(sys: &mut System, reg: GeneralWordReg), reg: GeneralWordReg) -> Self {
        Instr::R16 {
            func: InstrFunc(func),
            reg,
        }
    }

    pub fn new_rm16_imm8(
        func: fn(sys: &mut System, rm: RegMem, imm: u8),
        sys: &mut System,
    ) -> Self {
        let modrm = Self::modrm_all_16(sys);
        Instr::Rm16Imm8 {
            func: InstrFunc(func),
            rm: modrm.reg_mem,
            imm: sys.read_mem_8(),
        }
    }

    pub fn new_r16_m16(
        func: fn(sys: &mut System, reg: GeneralWordReg, mem: RmPtr),
        sys: &mut System,
    ) -> Self {
        let modrm = Self::modrm_all_16(sys);
        let mem = match modrm.reg_mem {
            RegMem::Ptr(ptr) => ptr,
            RegMem::Reg(_) => panic!("expected memory pointer in ModRM byte"),
        };

        Instr::R16M16 {
            func: InstrFunc(func),
            reg: modrm.word_reg(),
            mem,
        }
    }

    fn modrm_all_8(sys: &mut System) -> Modrm {
        let modrm = sys.read_mem_8();
        Modrm::decode(sys, modrm, Some(ModrmRegType::ByteSized), Size::Byte)
    }

    fn modrm_all_16(sys: &mut System) -> Modrm {
        let modrm = sys.read_mem_8();
        Modrm::decode(sys, modrm, Some(ModrmRegType::WordSized), Size::Word)
    }

    fn modrm_segment_16(sys: &mut System) -> Modrm {
        let modrm = sys.read_mem_8();
        Modrm::decode(sys, modrm, Some(ModrmRegType::Segment), Size::Word)
    }
}
