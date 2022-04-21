use crate::cpu::Cpu;
use crate::System;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum PortRequest {
    In8(u16),
    In16(u16),
    Out8(u16, u8),
    Out16(u16, u16),
}

pub enum PortResponse {
    In8(u8),
    In16(u16),
    Out,
}

pub trait Device<C>
where
    C: Cpu,
{
    fn init(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    fn step(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    fn handle_port(&mut self, sys: &mut System<C>, request: PortRequest) -> Option<PortResponse> {
        let _ = (sys, request);

        None
    }
}

pub struct Devices<C>
where
    C: Cpu,
{
    devices: Vec<Rc<RefCell<dyn Device<C>>>>,
}

impl<C> Devices<C>
where
    C: Cpu,
{
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn push<D>(&mut self, device: D) -> Rc<RefCell<D>>
    where
        D: Device<C> + 'static,
    {
        let rc = Rc::new(RefCell::new(device));
        let clone = Rc::clone(&rc);
        self.devices.push(rc);

        clone
    }

    pub fn init_all(&self, sys: &mut System<C>) {
        for device in &self.devices {
            device.borrow_mut().init(sys);
        }
    }

    pub fn step_all(&self, sys: &mut System<C>) {
        for device in &self.devices {
            device.borrow_mut().step(sys);
        }
    }

    pub fn port_in_8(&self, sys: &mut System<C>, port: u16) -> Option<u8> {
        let request = PortRequest::In8(port);
        for device in &self.devices {
            let value = device.borrow_mut().handle_port(sys, request);
            if let Some(PortResponse::In8(value)) = value {
                return Some(value);
            }
        }

        None
    }

    pub fn port_in_16(&self, sys: &mut System<C>, port: u16) -> Option<u16> {
        let request = PortRequest::In16(port);
        for device in &self.devices {
            let value = device.borrow_mut().handle_port(sys, request);
            if let Some(PortResponse::In16(value)) = value {
                return Some(value);
            }
        }

        None
    }

    pub fn port_out_8(&self, sys: &mut System<C>, port: u16, value: u8) -> Option<()> {
        let request = PortRequest::Out8(port, value);

        self.port_out(sys, request)
    }

    pub fn port_out_16(&self, sys: &mut System<C>, port: u16, value: u16) -> Option<()> {
        let request = PortRequest::Out16(port, value);

        self.port_out(sys, request)
    }

    fn port_out(&self, sys: &mut System<C>, request: PortRequest) -> Option<()> {
        for device in &self.devices {
            let value = device.borrow_mut().handle_port(sys, request);
            if let Some(PortResponse::Out) = value {
                return Some(());
            }
        }

        None
    }
}

impl<C> Clone for Devices<C>
where
    C: Cpu,
{
    fn clone(&self) -> Self {
        Self {
            devices: self.devices.clone(),
        }
    }
}

impl<C> Default for Devices<C>
where
    C: Cpu,
{
    fn default() -> Self {
        Self::new()
    }
}
