pub trait Cpu {
    fn init(&mut self);
    fn reset(&mut self);
    fn step(&mut self);

    fn run(&mut self) {
        self.init();
        self.reset();
        loop {
            self.step();
        }
    }
}
