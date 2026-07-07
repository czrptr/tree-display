pub use derive::TreeDisplay;

#[cfg(feature = "color")]
mod theme;
#[cfg(feature = "color")]
pub use theme::*;

mod display;
pub use display::*;

mod support;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────
pub trait TreeDisplay {
  fn tree(&self) -> Tree;
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  let mut result = String::new();
  value.tree().write_root(&mut result);
  result
}

pub struct Tree {
  pub label: Option<String>,
  pub content: String,
  pub subtrees: Vec<Tree>,
}

impl Tree {
  pub fn new(content: impl ToString, subtrees: Vec<Tree>) -> Tree {
    Tree {
      label: None,
      content: content.to_string(),
      subtrees,
    }
  }

  pub fn leaf(content: impl ToString) -> Tree {
    Tree {
      label: None,
      content: content.to_string(),
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
