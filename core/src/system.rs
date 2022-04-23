use crate::cpu::Cpu;
use crate::device::{Device, Devices};
use crate::mem::MemMap;
use std::sync::{Arc, Mutex};

pub struct System<C>
where
    C: Cpu,
{
    pub cpu: Box<C>,
    pub mem: MemMap,
    devices: Devices<C>,
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn new(cpu: C, mem: MemMap) -> Self {
        Self {
            cpu: Box::new(cpu),
            mem,
            devices: Devices::new(),
        }
    }

    pub fn init(&mut self) {
        let devices = Devices::clone(&self.devices);
        devices.init_all(self);

        self.cpu.init();
    }

    pub fn start(&mut self) {
        self.cpu.reset();

        let devices = Devices::clone(&self.devices);
        loop {
            devices.step_all(self);
            C::step(self);
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.start();
    }

    pub fn add_device<D>(&mut self, device: D) -> Arc<Mutex<D>>
    where
        D: Device<C> + 'static,
    {
        self.devices.push(device)
    }

    pub fn port_in_8(&mut self, port: u16) -> Option<u8> {
        let devices = Devices::clone(&self.devices);

        devices.port_in_8(self, port)
    }

    pub fn port_in_16(&mut self, port: u16) -> Option<u16> {
        let devices = Devices::clone(&self.devices);

        devices.port_in_16(self, port)
    }

    pub fn port_out_8(&mut self, port: u16, value: u8) -> Option<()> {
        let devices = Devices::clone(&self.devices);

        devices.port_out_8(self, port, value)
    }

    pub fn port_out_16(&mut self, port: u16, value: u16) -> Option<()> {
        let devices = Devices::clone(&self.devices);

        devices.port_out_16(self, port, value)
    }
}
