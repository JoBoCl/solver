use anyhow::Result;
use std::any::Any;

pub trait Solver: SolverToAny {
  fn part_one(&self) -> Result<String>;
  fn part_two(&self) -> Result<String>;
}

pub trait SolverToAny: 'static {
  fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! solver_to_any {
  ($name:ident) => {
    impl SolverToAny for $name {
      fn as_any(&self) -> &dyn std::any::Any { self }
    }
  };
}
