pub use derive::TreeDisplay;

mod ast;
mod tree_display;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn some_test() {
    use ast::*;

    let ident = match identifier(0, 0) {
      Ast::Identifier(ident) => ident,
      _ => unreachable!(),
    };

    let tree = ident;
    print!("-------\n{}\n-------\n", tree_display::tree_format(&tree));
  }
}
