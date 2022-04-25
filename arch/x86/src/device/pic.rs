use crate::{Cpu, System};
use firn_core::device::{Device, PortRequest, PortResponse};

pub const MASTER_COMMAND_PORT: u16 = 0x20;
pub const MASTER_DATA_PORT: u16 = 0x21;
pub const SLAVE_COMMAND_PORT: u16 = 0xa0;
pub const SLAVE_DATA_PORT: u16 = 0xa1;

#[derive(Eq, PartialEq)]
pub enum PicType {
    Master,
    Slave,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum InitControlWord {
    Icw1,
    Icw2,
    Icw3,
    Icw4,
}

pub struct Pic {
    pub pic_type: PicType,
    pub vector_offset: u8,

    awaiting_icw: InitControlWord,
    expecting_icw: InitControlWord,
}

impl Pic {
    pub fn new(pic_type: PicType) -> Self {
        Self {
            pic_type,
            vector_offset: 0,

            awaiting_icw: InitControlWord::Icw1,
            expecting_icw: InitControlWord::Icw2,
        }
    }

    pub fn handle_command(&mut self, command: u8) {
        let init = command & 0x10 != 0;
        if init {
            let expecting_icw_4 = command & 0x1 != 0;
            let single_pic = command & 0x2 != 0;
            if expecting_icw_4 {
                self.expecting_icw = InitControlWord::Icw4;
            } else if !single_pic {
                self.expecting_icw = InitControlWord::Icw3;
            }

            self.awaiting_icw = InitControlWord::Icw2;
            return;
        }

        todo!("handle OCW2 and OCW3");
    }

    pub fn handle_data_write(&mut self, data: u8) {
        match self.awaiting_icw {
            InitControlWord::Icw1 => todo!("set ISR or IRR"),
            InitControlWord::Icw2 => {
                self.vector_offset = data;
                self.await_icw_if_expected(InitControlWord::Icw3);
            }
            InitControlWord::Icw3 => self.await_icw_if_expected(InitControlWord::Icw4),
            InitControlWord::Icw4 => {
                self.awaiting_icw = InitControlWord::Icw1;
                self.expecting_icw = InitControlWord::Icw2;

                todo!("handle ICW4");
            }
        }
    }

    fn await_icw_if_expected(&mut self, icw: InitControlWord) {
        if icw > self.expecting_icw {
            self.awaiting_icw = InitControlWord::Icw1;
            self.expecting_icw = InitControlWord::Icw2;
        } else {
            self.awaiting_icw = icw;
        }
    }
}

impl Device<Cpu> for Pic {
    fn handle_port(&mut self, _sys: &mut System, request: PortRequest) -> Option<PortResponse> {
        let (command_port, data_port): (u16, u16) = match self.pic_type {
            PicType::Master => (MASTER_COMMAND_PORT, MASTER_DATA_PORT),
            PicType::Slave => (SLAVE_COMMAND_PORT, SLAVE_DATA_PORT),
        };

        match request {
            PortRequest::Out8(port, command) if port == command_port => {
                self.handle_command(command);
                Some(PortResponse::Out)
            }
            PortRequest::Out8(port, data) if port == data_port => {
                self.handle_data_write(data);
                Some(PortResponse::Out)
            }
            _ => None,
        }
    }
}

pub struct DualPic {
    pub master: Pic,
    pub slave: Pic,
}

impl DualPic {
    pub fn new() -> Self {
        Self {
            master: Pic::new(PicType::Master),
            slave: Pic::new(PicType::Slave),
        }
    }
}

impl Device<Cpu> for DualPic {
    fn handle_port(&mut self, _sys: &mut System, request: PortRequest) -> Option<PortResponse> {
        match request {
            PortRequest::Out8(port, command) if port == MASTER_COMMAND_PORT => {
                self.master.handle_command(command);
                Some(PortResponse::Out)
            }
            PortRequest::Out8(port, data) if port == MASTER_DATA_PORT => {
                self.master.handle_data_write(data);
                Some(PortResponse::Out)
            }

            PortRequest::Out8(port, command) if port == SLAVE_COMMAND_PORT => {
                self.slave.handle_command(command);
                Some(PortResponse::Out)
            }
            PortRequest::Out8(port, data) if port == SLAVE_DATA_PORT => {
                self.slave.handle_data_write(data);
                Some(PortResponse::Out)
            }

            _ => None,
        }
    }
}

impl Default for DualPic {
    fn default() -> Self {
        Self::new()
    }
}
