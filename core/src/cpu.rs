use crate::System;

pub trait Cpu {
    fn init(&mut self) {}
    fn reset(&mut self) {}

    fn step(sys: &mut System<Self>);
}
