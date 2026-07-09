pub mod color;
pub mod context;
pub mod format;
pub mod lines;
pub mod support;
pub mod theme;

use context::Context;
use format::Content;

pub use derive::TreeDisplay;
pub use format::Formatter;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

pub trait TreeDisplay {
  fn tree(&self, context: &Context) -> Tree;
}

pub struct Tree {
  pub label: Option<Box<dyn Content>>,
  pub content: Box<dyn Content>,
  pub subtrees: Vec<Tree>,
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

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
