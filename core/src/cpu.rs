use crate::System;

pub trait Cpu: Sized {
    fn init(&mut self) {}
    fn reset(&mut self) {}

    fn step(sys: &mut System<Self>);
}

pub trait Restrict: Cpu {
    type Feature: PartialEq;

    fn add_feature(&mut self, feature: Self::Feature);
    fn has_feature(&self, feature: Self::Feature) -> bool;
}
