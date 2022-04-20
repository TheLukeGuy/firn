use downcast_rs::{impl_downcast, Downcast};
use multimap::MultiMap;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

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

pub struct DynDeviceRef<C>
where
    C: Cpu + ?Sized,
{
    rc: Rc<RefCell<dyn Device<C>>>,
}

impl<C> DynDeviceRef<C>
where
    C: Cpu,
{
    pub fn new(device: impl Device<C> + 'static) -> Self {
        let rc = Rc::new(RefCell::new(device));

        Self { rc }
    }

    pub fn borrow(&self) -> &dyn Device<C> {
        &*(&*self.rc).borrow()
    }

    pub fn borrow_mut(&self) -> &mut dyn Device<C> {
        &mut *self.rc.borrow_mut()
    }

    pub fn specific<D>(&self) -> DeviceRef<C, D>
    where
        D: Device<C>,
    {
        DeviceRef {
            dyn_ref: self.clone(),
            _marker: PhantomData,
        }
    }
}

impl<C> Clone for DynDeviceRef<C>
where
    C: Cpu,
{
    fn clone(&self) -> Self {
        Self {
            rc: Rc::clone(&self.rc),
        }
    }
}

pub struct DeviceRef<C, D>
where
    C: Cpu,
    D: Device<C>,
{
    dyn_ref: DynDeviceRef<C>,
    _marker: PhantomData<D>,
}

impl<C, D> DeviceRef<C, D>
where
    C: Cpu + 'static,
    D: Device<C>,
{
    pub fn borrow(&self) -> &D {
        self.dyn_ref
            .borrow()
            .downcast_ref()
            .expect("device cannot be downcast to type parameter")
    }

    pub fn borrow_mut(&self) -> &mut D {
        self.dyn_ref
            .borrow_mut()
            .downcast_mut()
            .expect("device cannot be downcast to type parameter")
    }
}
