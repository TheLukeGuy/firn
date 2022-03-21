pub trait Cpu {
    fn reset(&mut self);
    fn step(&mut self);

    fn run(&mut self) {
        self.reset();
        loop {
            self.step();
        }
    }
}
