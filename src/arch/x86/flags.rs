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
