
pub trait SteppedAlgorithm {
    type ReturnType;

    fn step(&mut self) -> bool;
    fn run(self) -> Self::ReturnType;
}
