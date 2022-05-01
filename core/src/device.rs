use crate::cpu::Cpu;
use crate::System;
use std::sync::{Arc, Mutex};

/// A port request that devices can choose to handle.
///
/// A `PortRequest` is sent to devices in [`Device::handle_port`]. For more information on port
/// handling, see there.
///
/// [`Device::handle_port`]: Device::handle_port
#[derive(Copy, Clone)]
pub enum PortRequest {
    /// An input port request which requests an 8-bit value: `In8(port)`.
    In8(u16),
    /// An input port request which requests a 16-bit value: `In16(port)`.
    In16(u16),
    /// An output port request which supplies an 8-bit value: `Out8(port, value)`.
    Out8(u16, u8),
    /// An output port request which supplies a 16-bit value: `Out16(port, value)`.
    Out16(u16, u16),
}

/// A port response that's returned when a device chooses to handle a port.
///
/// A `PortResponse` responds to a [`PortRequest`] which is sent to devices in
/// [`Device::handle_port`]. For more information on port handling, see `Device::handle_port`.
///
/// [`PortRequest`]: PortRequest
/// [`Device::handle_port`]: Device::handle_port
pub enum PortResponse {
    /// A port response which responds to a [`PortRequest::In8`] with an 8-bit response value.
    ///
    /// [`PortRequest::In8`]: PortRequest::In8
    In8(u8),
    /// A port response which responds to a [`PortRequest::In16`] with a 16-bit response value.
    ///
    /// [`PortRequest::In16`]: PortRequest::In16
    In16(u16),
    /// A port response which responds to a [`PortRequest::Out8`] or [`PortRequest::Out16`].
    ///
    /// [`PortRequest::Out8`]: PortRequest::Out8
    /// [`PortRequest::Out16`]: PortRequest::Out16
    Out,
}

/// A (technically) optional device that connects to a system.
///
/// A "device" here simply refers to any component of the system besides the motherboard (see
/// [`System`]), CPU (see [`Cpu`]), or memory (see [`Mem`] and [`MemMap`]). Everything besides those
/// three components is considered "optional" and should be represented as a `Device`.
///
/// Devices can access other devices by creating a constructor that takes in an
/// `Arc<Mutex<OtherDevice>>` as a parameter, which a user can obtain from [`System::add_device`].
/// These parameters should come before any other parameters in a device constructor for consistency
/// with other devices.
///
/// You must implement `Device<C>` for every `Cpu` that your device can be used with, where `C` is
/// the `Cpu` type. If a real-world device only supports a couple of CPUs, you should probably only
/// implement `Device` for those CPUs. Try to support as many CPUs that the real device supports as
/// possible.
///
/// [`System`]: crate::System
/// [`Cpu`]: crate::cpu::Cpu
/// [`Mem`]: crate::mem::Mem
/// [`MemMap`]: crate::mem::MemMap
/// [`System::add_device`]: crate::System::add_device
pub trait Device<C>: Send + Sync
where
    C: Cpu,
{
    /// Initializes the device.
    ///
    /// This is called in [`System::init`] (and therefore [`System::run`]) before the [`Cpu`] is
    /// initialized. Devices are initialized in an undefined order so you should not rely on any
    /// other devices being initialized when initializing your own device. In the future, there will
    /// be a way for devices to have more control over when they're initialized in relation to other
    /// devices.
    ///
    /// If this method isn't implemented, the `Device` will do nothing during initialization.
    ///
    /// [`System::init`]: crate::System::init
    /// [`System::run`]: crate::System::run
    /// [`Cpu`]: crate::cpu::Cpu
    fn init(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    /// Executes the next iteration of the device.
    ///
    /// This is called constantly while the [`System`] is running. All devices will be stepped
    /// before the [`Cpu`] is stepped. Devices are stepped in an undefined order. In the future,
    /// there will be a way for devices to have more control over when they're stepped in relation
    /// to other devices.
    ///
    /// Devices have no control over how often this method is called; instead it's determined by
    /// [`System::start`]. If you need to loop at a certain frequency, use [`std::thread`] instead
    /// of (or in addition to) implementing `step`.
    ///
    /// If this method isn't implemented, the `Device` will do nothing during each iteration of the
    /// execution loop.
    ///
    /// [`System`]: crate::System
    /// [`Cpu`]: crate::cpu::Cpu
    /// [`System::start`]: crate::System::start
    /// [`std::thread`]: std::thread
    fn step(&mut self, sys: &mut System<C>) {
        let _ = sys;
    }

    /// Handles a port request, or ignores it.
    ///
    /// The CPU determines when to call this method. For example, an x86 CPU will call it primarily
    /// when an IN or OUT instruction is executed. Some CPUs don't support this form of I/O, so it
    /// may or may not ever be called.
    ///
    /// The device receives a [`PortRequest`] which contains the port and, if it's an output port,
    /// the value that's being output. The device must return an `Option` containing a
    /// [`PortResponse`], which should be `None` if the device did not handle the request or `Some`
    /// if the device did handle the request. If the device did handle the request, the
    /// `PortResponse` contains nothing if it's an output port, or a value if it's an input port.
    ///
    /// When the CPU wants a port to be handled, it calls the correct method in [`System`] which
    /// calls the correct method in [`Devices`]. `Devices` will loop through all devices in an
    /// undefined order and find the first one that handles the port. Once it finds one device that
    /// handles the port, it doesn't send the port request to any more devices.
    ///
    /// | If the `PortRequest` is... | And you...      | You return...       |
    /// | -------------------------- | --------------- | ------------------- |
    /// | `In8(port)`                | Handle it       | `Some(In8(value))`  |
    /// | `In16(port)`               | Handle it       | `Some(In16(value))` |
    /// | `Out8(port, value)`        | Handle it       | `Some(Out)`         |
    /// | `Out16(port, value)`       | Handle it       | `Some(Out)`         |
    /// | Anything                   | Don't handle it | `None`              |
    ///
    /// If this method isn't implemented, the `Device` will do nothing and return `None` (indicating
    /// the port wasn't handled) for every port request.
    ///
    /// [`PortRequest`]: PortRequest
    /// [`PortResponse`]: PortResponse
    /// [`System`]: crate::System
    /// [`Devices`]: Devices
    fn handle_port(&mut self, sys: &mut System<C>, request: PortRequest) -> Option<PortResponse> {
        let _ = (sys, request);

        None
    }
}

/// A collection of devices.
pub struct Devices<C>
where
    C: Cpu,
{
    devices: Vec<Arc<Mutex<dyn Device<C>>>>,
}

impl<C> Devices<C>
where
    C: Cpu,
{
    /// Creates a new empty collection of devices.
    ///
    /// [`Device`]: Device
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Pushes a device to the end of the collection.
    ///
    /// This returns an `Arc<Mutex<D>>` (where `D` is the type of device passed in) which can be
    /// given to other devices that need to access this device. See [`Device`] for more information.
    ///
    /// [`Device`]: Device
    pub fn push<D>(&mut self, device: D) -> Arc<Mutex<D>>
    where
        D: Device<C> + 'static,
    {
        let arc = Arc::new(Mutex::new(device));
        let clone = Arc::clone(&arc);
        self.devices.push(arc);

        clone
    }

    /// Initializes all devices in the collection.
    pub fn init_all(&self, sys: &mut System<C>) {
        for device in &self.devices {
            device.lock().unwrap().init(sys);
        }
    }

    /// Steps all devices in the collection.
    pub fn step_all(&self, sys: &mut System<C>) {
        for device in &self.devices {
            device.lock().unwrap().step(sys);
        }
    }

    /// Handles an input port request which expects an 8-bit response.
    ///
    /// See [`Device::handle_port`] for more information.
    ///
    /// [`Device::handle_port`]: Device::handle_port
    pub fn port_in_8(&self, sys: &mut System<C>, port: u16) -> Option<u8> {
        let request = PortRequest::In8(port);
        for device in &self.devices {
            let value = device.lock().unwrap().handle_port(sys, request);
            if let Some(PortResponse::In8(value)) = value {
                return Some(value);
            }
        }

        None
    }

    /// Handles an input port request which expects a 16-bit response.
    ///
    /// See [`Device::handle_port`] for more information.
    ///
    /// [`Device::handle_port`]: Device::handle_port
    pub fn port_in_16(&self, sys: &mut System<C>, port: u16) -> Option<u16> {
        let request = PortRequest::In16(port);
        for device in &self.devices {
            let value = device.lock().unwrap().handle_port(sys, request);
            if let Some(PortResponse::In16(value)) = value {
                return Some(value);
            }
        }

        None
    }

    /// Handles an output port request..
    ///
    /// See [`Device::handle_port`] for more information.
    ///
    /// [`Device::handle_port`]: Device::handle_port
    pub fn port_out_8(&self, sys: &mut System<C>, port: u16, value: u8) -> Option<()> {
        let request = PortRequest::Out8(port, value);

        self.port_out(sys, request)
    }

    /// Handles an output port request..
    ///
    /// See [`Device::handle_port`] for more information.
    ///
    /// [`Device::handle_port`]: Device::handle_port
    pub fn port_out_16(&self, sys: &mut System<C>, port: u16, value: u16) -> Option<()> {
        let request = PortRequest::Out16(port, value);

        self.port_out(sys, request)
    }

    fn port_out(&self, sys: &mut System<C>, request: PortRequest) -> Option<()> {
        for device in &self.devices {
            let value = device.lock().unwrap().handle_port(sys, request);
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
