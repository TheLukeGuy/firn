use downcast_rs::{impl_downcast, Downcast};
use multimap::MultiMap;

use crate::cpu::Cpu;
use crate::System;
pub use firn_core_macros::io_port;
pub use firn_core_macros::io_ports;

pub trait Device<C>: Downcast
where
    C: Cpu,
{
    fn init(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    fn step(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    fn ports(&self) -> MultiMap<u16, IoPortHandler<C>> {
        MultiMap::new()
    }
}

impl_downcast!(Device<C> where C: Cpu);

pub struct IoPortMeta<C>
where
    C: Cpu,
{
    pub port: u16,
    pub handler: IoPortHandler<C>,
}

pub enum IoPortHandler<C>
where
    C: Cpu,
{
    In8(fn(&mut dyn Device<C>, sys: &mut System<C>) -> u8),
    In16(fn(&mut dyn Device<C>, sys: &mut System<C>) -> u16),

    Out8(fn(&mut dyn Device<C>, sys: &mut System<C>, u8)),
    Out16(fn(&mut dyn Device<C>, sys: &mut System<C>, u16)),
}
