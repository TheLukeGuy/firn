use crate::System;

pub trait Cpu {
    fn system(&mut self) -> &mut System;

    fn init(&mut self) {}
    fn reset(&mut self) {}

    fn step(&mut self);

    fn run(&mut self) {
        for device in &mut self.system().devices {
            device.init();
        }
        self.init();

        self.reset();
        loop {
            for device in &mut self.system().devices {
                device.step();
            }
            self.step();
        }
    }
}
