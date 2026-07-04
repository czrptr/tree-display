pub use derive::TreeDisplay;
use std::rc::Rc;
use std::sync::Arc;

pub struct Field {
  pub name: String,
  pub value: String,
}

pub struct TreeNode {
  pub label: String,
  pub fields: Vec<Field>,
  pub children: Vec<TreeNode>,
}

impl Field {
  fn write(&self, out: &mut String, prefix: &str, last: bool) {
    out.push_str(prefix);
    out.push_str(if last { "╰─ " } else { "├─ " });
    out.push_str(&self.name);
    out.push_str(&self.value);
  }
}

impl TreeNode {
  pub fn write_root(&self, out: &mut String) {
    out.push_str(&self.label);

    let total = self.fields.len() + self.children.len();
    let mut index = 0;

    for field in &self.fields {
      index += 1;
      out.push('\n');
      field.write(out, "", index == total);
    }

    for child in &self.children {
      index += 1;
      out.push('\n');
      child.write(out, "", index == total);
    }
  }

  fn write(&self, out: &mut String, prefix: &str, last: bool) {
    out.push_str(prefix);
    out.push_str(if last { "╰╼ " } else { "├╼ " });
    out.push_str(&self.label);

    let next_prefix = format!("{}{}", prefix, if last { "   " } else { "│  " },);

    let total = self.fields.len() + self.children.len();
    let mut index = 0;

    for field in &self.fields {
      index += 1;
      out.push('\n');
      field.write(out, &next_prefix, index == total);
    }

    for child in &self.children {
      index += 1;
      out.push('\n');
      child.write(out, &next_prefix, index == total);
    }
  }
}

pub trait TreeDisplay {
  fn tree(&self) -> TreeNode;
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Box<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for &T {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay> TreeDisplay for Option<T> {
  fn tree(&self) -> TreeNode {
    match self {
      Some(value) => value.tree(),
      None => TreeNode {
        label: "None".into(),
        fields: Vec::new(),
        children: Vec::new(),
      },
    }
  }
}

impl<T: TreeDisplay> TreeDisplay for Vec<T> {
  fn tree(&self) -> TreeNode {
    TreeNode {
      label: String::from("Vec"),
      fields: Vec::new(),
      children: self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect(),
    }
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Rc<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Arc<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  let mut result = String::new();
  value.tree().write_root(&mut result);
  result
}

#[derive(Debug, TreeDisplay)]
struct Foo0();

#[derive(Debug, TreeDisplay)]
struct Foo1(Foo0, #[tree(child)] Option<Box<Foo2>>);

#[derive(Debug, TreeDisplay)]
struct Foo2 {
  #[tree(unlabeled)]
  field1: bool,
  #[tree(child, label = "ceva")]
  field2: Foo1,
  #[tree(child)]
  field3: Foo1,
  #[tree(child)]
  field4: Vec<Foo1>,
  #[tree(ignore)]
  ignored: u32,
}

// #[derive(TreeDisplay)]
// enum Foo3 {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn some_test() {
    let tree = Foo2 {
      field1: true,
      field2: Foo1(
        Foo0(),
        Some(Box::new(Foo2 {
          field1: false,
          field2: Foo1(Foo0(), None),
          field3: Foo1(Foo0(), None),
          field4: vec![Foo1(Foo0(), None)],
          ignored: 0,
        })),
      ),
      field3: Foo1(Foo0(), None),
      field4: vec![Foo1(Foo0(), None)],
      ignored: 0,
    };
    print!("-------\n{}\n-------\n", tree_format(&tree));
  }
}
