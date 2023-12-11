use std::any::Any;

pub trait Solver: SolverToAny {
  fn part_one(&self) -> std::io::Result<String>;
  fn part_two(&self) -> std::io::Result<String>;
}

pub trait SolverToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}
