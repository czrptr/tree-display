pub use derive::TreeDisplay;
use std::fmt;

pub trait TreeDisplay {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  struct TreeDisplayWrapper<'a, T: TreeDisplay>(pub &'a T);

  impl<'a, T: TreeDisplay> fmt::Display for TreeDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      self.0.fmt(f)
    }
  }

  format!("{}", TreeDisplayWrapper(value))
}

#[derive(Debug, Default, TreeDisplay)]
struct Foo1(Box<Foo2>);

#[derive(Debug, Default, TreeDisplay)]
struct Foo2 {
  #[tree(inline)]
  field1: bool,
  field2: Foo1,
  #[tree(inline)]
  field3: Foo1,
  field4: Foo1,
  #[tree(ignore)]
  ignored: u32,
}
// #[derive(TreeDisplay)]
enum Foo3 {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn some_test() {
    print!("-------\n{}\n-------\n", tree_format(&Foo2::default()));
  }
}
