use enum_dispatch::enum_dispatch;


#[enum_dispatch]
trait Calculator {
  fn calculate(&self) -> i32;
}

struct Add {
  a: i32,
  b: i32,
}

impl Calculator for Add {
  fn calculate(&self) -> i32 {
    self.a + self.b
  }
}

struct Sub {
  a: i32,
  b: i32,
}
impl Calculator for Sub {
  fn calculate(&self) -> i32 {
    self.a - self.b
  }
}

#[enum_dispatch(Calculator)]
enum Operation {
  Add,
  Sub,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_add_operation() {
    let add = Add { a: 1, b: 2 };
    let add_op = Operation::Add(add);
    assert_eq!(add_op.calculate(), 3);
  } 
}