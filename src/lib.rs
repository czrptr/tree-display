pub use derive::TreeDisplay;
use std::any::Any;

#[cfg(feature = "color")]
mod theme;
#[cfg(feature = "color")]
pub use theme::*;

pub mod display;
pub mod support;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────
pub trait TreeDisplay {
  fn tree(&self) -> Tree;
}

pub trait Content: Any {
  fn to_string(&self) -> String;
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  let mut result = String::new();
  value.tree().write_root(&mut result);
  result
}

pub struct Tree {
  pub label: Option<Box<dyn Content>>,
  pub content: Box<dyn Content>,
  pub subtrees: Vec<Tree>,
}

impl Tree {
  pub fn new(content: impl Content, subtrees: Vec<Tree>) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees,
    }
  }

  pub fn leaf(content: impl Content) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees: Vec::new(),
    }
  }

  pub fn labeled(mut self, label: impl Content) -> Self {
    self.label = Some(Box::new(label));
    self
  }

  pub fn is_leaf(&self) -> bool {
    self.subtrees.is_empty()
  }
}
