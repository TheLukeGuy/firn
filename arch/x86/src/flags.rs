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

    pub fn get_8(&self) -> u8 {
        let mut value = 0x2;

        if self.carry {
            value |= 0x001;
        }
        if self.parity {
            value |= 0x004;
        }
        if self.adjust {
            value |= 0x010;
        }
        if self.zero {
            value |= 0x040;
        }
        if self.sign {
            value |= 0x080;
        }

        value
    }

    pub fn get_16(&self) -> u16 {
        let mut value = self.get_8() as u16 | 0xf000;

        if self.trap {
            value |= 0x100;
        }
        if self.interrupt {
            value |= 0x200;
        }
        if self.direction {
            value |= 0x400;
        }
        if self.overflow {
            value |= 0x800;
        }

        value
    }

    pub fn set_8(&mut self, value: u8) {
        self.carry = value & 0x001 != 0;
        self.parity = value & 0x004 != 0;
        self.adjust = value & 0x010 != 0;
        self.zero = value & 0x040 != 0;
        self.sign = value & 0x080 != 0;
    }

    pub fn set_16(&mut self, value: u16) {
        self.set_8(value as u8);

        self.trap = value & 0x100 != 0;
        self.interrupt = value & 0x200 != 0;
        self.direction = value & 0x400 != 0;
        self.overflow = value & 0x800 != 0;
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
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_flags() -> Flags {
        Flags {
            carry: true,
            parity: true,
            adjust: false,
            zero: true,
            sign: false,
            trap: true,
            interrupt: false,
            direction: false,
            overflow: true,
        }
    }

    #[test]
    fn should_convert_to_u8_correctly() {
        let flags = create_test_flags();
        let converted = flags.get_8();
        assert_eq!(0b01000111, converted);
    }

    #[test]
    fn should_convert_to_u16_correctly() {
        let flags = create_test_flags();
        let converted = flags.get_16();
        assert_eq!(0b1111100101000111, converted);
    }

    #[test]
    fn should_convert_from_u8_correctly() {
        let mut flags = Flags::new();
        flags.set_8(0b01000111);
        assert!(flags.carry && flags.parity && !flags.adjust && flags.zero && !flags.sign);
    }

    #[test]
    fn should_convert_from_u16_correctly() {
        let mut flags = Flags::new();
        flags.set_16(0b1111100101000111);
        assert!(
            flags.carry
                && flags.parity
                && !flags.adjust
                && flags.zero
                && !flags.sign
                && flags.trap
                && !flags.interrupt
                && !flags.direction
                && flags.overflow
        );
    }
}
