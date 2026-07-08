pub use derive::TreeDisplay;
use std::any::Any;

#[cfg(feature = "color")]
mod theme;
#[cfg(feature = "color")]
pub use theme::*;

mod display;
#[allow(unused_imports)]
pub use display::*;

mod support;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────
pub trait TreeDisplay {
  fn tree(&self) -> Tree;
}

pub trait TreeContent: Any {
  fn to_string(&self) -> String;
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  let mut result = String::new();
  value.tree().write_root(&mut result);
  result
}

pub struct Tree {
  pub label: Option<String>,
  pub content: Box<dyn TreeContent>,
  pub subtrees: Vec<Tree>,
}

impl Tree {
  pub fn new<T: TreeContent>(content: T, subtrees: Vec<Tree>) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees,
    }
  }

  pub fn leaf<T: TreeContent>(content: T) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees: Vec::new(),
    }
  }

  pub fn labeled(mut self, label: impl Into<Option<String>>) -> Self {
    self.label = label.into();
    self
  }

  pub fn is_leaf(&self) -> bool {
    self.subtrees.is_empty()
  }
}
