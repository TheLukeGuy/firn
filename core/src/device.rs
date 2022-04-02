pub trait Device {
    fn init(&mut self) {}
    fn step(&mut self) {}
}

pub enum IoPortHandler<D>
where
    D: Device,
{
    In8(fn(&mut D) -> u8),
    In16(fn(&mut D) -> u16),

    Out8(fn(&mut D, u8)),
    Out16(fn(&mut D, u16)),
}

impl<D> IoPortHandler<D>
where
    D: Device,
{
    pub fn in_8(&self, device: &mut D) -> u8 {
        match self {
            Self::In8(handler) => handler(device),
            _ => panic!("expected 8-bit input port"),
        }
    }

    pub fn in_16(&self, device: &mut D) -> u16 {
        match self {
            Self::In16(handler) => handler(device),
            _ => panic!("expected 16-bit input port"),
        }
    }

    pub fn out_8(&self, device: &mut D, value: u8) {
        match self {
            Self::Out8(handler) => handler(device, value),
            _ => panic!("expected 8-bit output port"),
        }
    }

    pub fn out_16(&self, device: &mut D, value: u16) {
        match self {
            Self::Out16(handler) => handler(device, value),
            _ => panic!("expected 16-bit output port"),
        }
    }
}

pub struct IoPortWrapper<D>
where
    D: Device,
{
    pub port: u16,
    pub handler: IoPortHandler<D>,
}
