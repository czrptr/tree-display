//! A library for displaying Rust data structures as syntax-highlighted trees.
//!
//! `tree-display` provides a simple derive macro (`#[derive(TreeDisplay)]`)
//! to render any data structure as a beautifully formatted tree in the terminal.
//! Features include syntax highlighting, custom value mapping, and full theming support.
//!
//! ## Quick Start
//! ```
//! use tree_display::{TreeDisplay, Formatter};
//!
//! #[derive(Debug, TreeDisplay)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let person = Person {
//!     name: "Alice".to_string(),
//!     age: 30,
//! };
//!
//! println!("{}", Formatter::of(&person).format());
//! ```
//!
//! ## Features
//! - **Derive macro**: `#[derive(TreeDisplay)]` for automatic tree generation
//! - **Field attributes**: Control rendering of individual fields:
//!   - `#[tree(map)]` - Apply a custom mapper from [`Context`]
//!   - `#[tree(ignore)]` - Exclude a field from the tree
//!   - `#[tree(label = "...")]` - Override the field's display label
//!   - `#[tree(unlabeled)]` - Display the field without a label
//! - **Custom mapping**: Transform values with [`Context`] mappers
//! - **Theming**: Predefined themes (VS Code Dark+, Solarized, etc.)
//! - **Color support**: Optional ANSI color highlighting
//! - **Line styles**: ASCII or Unicode box-drawing characters
//! - **Formatting**: Labels, alignment, and custom content types

pub mod color;
pub mod context;
pub mod format;
pub mod graphics;
pub mod support;
pub mod theme;

use context::Context;
use format::Content;

pub use derive::TreeDisplay;
pub use format::Formatter;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

/// A type that can be displayed as a tree.
///
/// This trait is automatically implemented by `#[derive(TreeDisplay)]`
/// and is also implemented for many standard library types.
pub trait TreeDisplay {
  /// Converts the value into a tree structure using the given context.
  ///
  /// The context provides custom mappers that can transform values before they are displayed.
  fn tree(&self, context: &Context) -> Tree;
}

/// A tree node containing content and child subtrees.
///
/// Trees are built from a root node with zero or more child nodes,
/// each of which can have their own children. Nodes can optionally
/// have labels that appear alongside their content.
///
/// ## Example
/// ```
/// use tree_display::{Tree, format::Member};
///
/// let tree = Tree::leaf("root")
///     .labeled(Member::new("label"));
/// ```
pub struct Tree {
  /// Optional label displayed next to the content
  pub label: Option<Box<dyn Content>>,
  /// The main content of this node
  pub content: Box<dyn Content>,
  /// Child subtrees
  pub subtrees: Vec<Tree>,
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Tree {
  /// Creates a new tree node with content and children.
  pub fn new(content: impl Content, subtrees: Vec<Tree>) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees,
    }
  }

  /// Creates a leaf node (a node with no children).
  pub fn leaf(content: impl Content) -> Tree {
    Tree {
      label: None,
      content: Box::new(content),
      subtrees: Vec::new(),
    }
  }

  /// Adds a label to the node.
  ///
  /// Labels appear before the content, typically as a key or name.
  pub fn labeled(mut self, label: impl Content) -> Self {
    self.label = Some(Box::new(label));
    self
  }

  /// Returns whether this node is a leaf (has no children).
  pub fn is_leaf(&self) -> bool {
    self.subtrees.is_empty()
  }
}
