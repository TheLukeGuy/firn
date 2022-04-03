use downcast_rs::{impl_downcast, Downcast};
use multimap::MultiMap;

pub use firn_core_macros::io_port;
pub use firn_core_macros::io_ports;

pub trait Device: Downcast {
    fn init(&mut self) {}
    fn step(&mut self) {}

    fn ports(&self) -> MultiMap<u16, IoPortHandler> {
        MultiMap::new()
    }
}

impl_downcast!(Device);

pub struct IoPortMeta {
    pub port: u16,
    pub handler: IoPortHandler,
}

pub enum IoPortHandler {
    In8(fn(&mut dyn Device) -> u8),
    In16(fn(&mut dyn Device) -> u16),

    Out8(fn(&mut dyn Device, u8)),
    Out16(fn(&mut dyn Device, u16)),
}
