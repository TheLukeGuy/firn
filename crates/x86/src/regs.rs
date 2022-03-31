use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum GeneralByteReg {
    Al = 0,
    Cl = 1,
    Dl = 2,
    Bl = 3,
    Ah = 4,
    Ch = 5,
    Dh = 6,
    Bh = 7,
}

impl GeneralByteReg {
    pub fn from_u8(reg: u8) -> Option<Self> {
        num_traits::FromPrimitive::from_u8(reg)
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum GeneralWordReg {
    Ax = 0,
    Cx = 1,
    Dx = 2,
    Bx = 3,
    Sp = 4,
    Bp = 5,
    Si = 6,
    Di = 7,
}

impl GeneralWordReg {
    pub fn from_u8(reg: u8) -> Option<Self> {
        num_traits::FromPrimitive::from_u8(reg)
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum SegmentReg {
    Es = 0,
    Cs = 1,
    Ss = 2,
    Ds = 3,
}

impl SegmentReg {
    pub fn from_u8(reg: u8) -> Option<Self> {
        num_traits::FromPrimitive::from_u8(reg)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WordReg {
    General(GeneralWordReg),
    Segment(SegmentReg),
}

impl From<GeneralWordReg> for WordReg {
    fn from(reg: GeneralWordReg) -> Self {
        WordReg::General(reg)
    }
}

impl From<SegmentReg> for WordReg {
    fn from(reg: SegmentReg) -> Self {
        WordReg::Segment(reg)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GeneralReg {
    Byte(GeneralByteReg),
    Word(GeneralWordReg),
}

impl From<GeneralByteReg> for GeneralReg {
    fn from(reg: GeneralByteReg) -> Self {
        GeneralReg::Byte(reg)
    }
}

impl From<GeneralWordReg> for GeneralReg {
    fn from(reg: GeneralWordReg) -> Self {
        GeneralReg::Word(reg)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Reg {
    Byte(GeneralByteReg),
    Word(WordReg),
}

impl From<GeneralByteReg> for Reg {
    fn from(reg: GeneralByteReg) -> Self {
        Reg::Byte(reg)
    }
}

impl From<GeneralWordReg> for Reg {
    fn from(reg: GeneralWordReg) -> Self {
        Reg::Word(WordReg::General(reg))
    }
}

impl From<SegmentReg> for Reg {
    fn from(reg: SegmentReg) -> Self {
        Reg::Word(WordReg::Segment(reg))
    }
}

impl From<WordReg> for Reg {
    fn from(reg: WordReg) -> Self {
        Reg::Word(reg)
    }
}

impl From<GeneralReg> for Reg {
    fn from(reg: GeneralReg) -> Self {
        match reg {
            GeneralReg::Byte(reg) => Reg::Byte(reg),
            GeneralReg::Word(reg) => Reg::Word(WordReg::General(reg)),
        }
    }
}
