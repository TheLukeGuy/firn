use crate::cpu::Cpu;
use crate::device::{Device, DeviceRef, DynDeviceRef, IoPortHandler};
use crate::mem::MemMap;

pub struct System<C>
where
    C: Cpu + ?Sized,
{
    pub cpu: Box<C>,
    pub mem: MemMap,
    pub devices: Vec<DynDeviceRef<C>>,
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn new(cpu: C, mem: MemMap) -> Self {
        Self {
            cpu: Box::new(cpu),
            mem,
            devices: Vec::new(),
        }
    }

    pub fn add_device<D>(&mut self, device: D) -> DeviceRef<C, D>
    where
        D: Device<C>,
    {
        let device = DynDeviceRef::new(device);
        self.devices.push(device);

        device.specific()
    }

    pub fn init(&mut self) {
        for device in self.devices {
            device.borrow_mut().init(self);
        }
        self.cpu.init();
    }

    pub fn start(&mut self) {
        self.cpu.reset();
        loop {
            for device in self.devices {
                device.borrow_mut().step(self);
            }
            C::step(self);
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.start();
    }
}

macro_rules! port_handler {
    ($self:ident, $port:ident, $variant:ident) => {
        $self
            .devices
            .iter_mut()
            .filter_map(|device| {
                device
                    .ports()
                    .remove(&$port)
                    .map(|handlers| (device, handlers))
            })
            .filter_map(|(device, handlers)| {
                let handler = handlers
                    .iter()
                    .filter_map(|handler| match handler {
                        IoPortHandler::$variant(handler) => Some(handler),
                        _ => None,
                    })
                    .next();

                handler.map(|handler| (&mut **device, *handler))
            })
            .next()
    };
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn port_in_8(&mut self, port: u16) -> Option<u8> {
        match port_handler!(self, port, In8) {
            Some((device, handler)) => Some(handler(device)),
            None => panic!("unimplemented IO port: {:#x}", port),
        }
    }

    pub fn port_in_16(&mut self, port: u16) -> Option<u16> {
        match port_handler!(self, port, In16) {
            Some((device, handler)) => Some(handler(device)),
            None => panic!("unimplemented IO port: {:#x}", port),
        }
    }

    pub fn port_out_8(&mut self, port: u16, value: u8) {
        match port_handler!(self, port, Out8) {
            Some((device, handler)) => handler(device, value),
            None => panic!("unimplemented IO port: {:#x}", port),
        }
    }

    pub fn port_out_16(&mut self, port: u16, value: u16) {
        match port_handler!(self, port, Out16) {
            Some((device, handler)) => handler(device, value),
            None => panic!("unimplemented IO port: {:#x}", port),
        }
    }
}
