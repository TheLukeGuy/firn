use crate::cpu::Cpu;
use crate::device::{Device, IoPortHandler};
use crate::mem::MemMap;

pub struct System<C>
where
    C: Cpu + ?Sized,
{
    pub cpu: Box<C>,
    pub mem: MemMap,
    pub devices: Option<Vec<Box<dyn Device<C>>>>,
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn new(cpu: C, mem: MemMap) -> Self {
        Self {
            cpu: Box::new(cpu),
            mem,
            devices: Some(Vec::new()),
        }
    }

    pub fn add_device(&mut self, device: impl Device<C>) {
        let boxed = Box::new(device);
        self.devices.as_mut().unwrap().push(boxed);
    }

    pub fn init(&mut self) {
        let mut devices = self.devices.take().unwrap();
        for device in &mut devices {
            device.init(self);
        }
        self.devices = Some(devices);

        self.cpu.init();
    }

    pub fn start(&mut self) {
        self.cpu.reset();
        loop {
            let mut devices = self.devices.take().unwrap();
            for device in &mut devices {
                device.step(self);
            }
            self.devices = Some(devices);

            C::step(self);
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.start();
    }
}

macro_rules! filter_port {
    ($device:ident, $port:ident) => {
        $device
            .ports()
            .iter()
            .filter(|&(handler_port, _)| *handler_port == $port)
    };
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn port_in_8(&mut self, port: u16) -> Option<u8> {
        let mut devices = self.devices.take().unwrap();
        for device in &mut devices {
            for (_, handler) in filter_port!(device, port) {
                if let IoPortHandler::In8(handler) = handler {
                    let value = handler(&mut **device, self);
                    self.devices = Some(devices);
                    return Some(value);
                }
            }
        }
        self.devices = Some(devices);

        None
    }

    pub fn port_in_16(&mut self, port: u16) -> Option<u16> {
        let mut devices = self.devices.take().unwrap();
        for device in &mut devices {
            for (_, handler) in filter_port!(device, port) {
                if let IoPortHandler::In16(handler) = handler {
                    let value = handler(&mut **device, self);
                    self.devices = Some(devices);
                    return Some(value);
                }
            }
        }
        self.devices = Some(devices);

        None
    }

    pub fn port_out_8(&mut self, port: u16, value: u8) -> Option<()> {
        let mut devices = self.devices.take().unwrap();
        for device in &mut devices {
            for (_, handler) in filter_port!(device, port) {
                if let IoPortHandler::Out8(handler) = handler {
                    handler(&mut **device, self, value);
                    self.devices = Some(devices);
                    return Some(());
                }
            }
        }
        self.devices = Some(devices);

        None
    }

    pub fn port_out_16(&mut self, port: u16, value: u16) -> Option<()> {
        let mut devices = self.devices.take().unwrap();
        for device in &mut devices {
            for (_, handler) in filter_port!(device, port) {
                if let IoPortHandler::Out16(handler) = handler {
                    handler(&mut **device, self, value);
                    self.devices = Some(devices);
                    return Some(());
                }
            }
        }
        self.devices = Some(devices);

        None
    }
}
