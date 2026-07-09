//! Tree rendering and formatting.
//!
//! This module handles the conversion of `Tree` structures into formatted
//! string output. It manages indentation, line drawing, coloring, and
//! alignment of tree nodes.

use super::{color::Colored, context::Context, theme::Theme, Tree, TreeDisplay};
use std::any::Any;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

/// Configurable formatter for tree display.
///
/// A formatter holds a value that implements `TreeDisplay` along with
/// optional theme and context settings. It produces a formatted string
/// representation of the tree.
///
/// ## Example
/// ```no_run
/// use tree_display::Formatter;
///
/// let output = Formatter::of(&my_value)
///     .with_theme(&Theme::default())
///     .format();
/// ```
pub struct Formatter<'value, 'context, T: TreeDisplay> {
  value: &'value T,
  theme: Option<Theme>,
  context: Option<&'context Context>,
}

/// A value that can be displayed as content in a tree node.
///
/// This trait is implemented for all types that can appear as leaf
/// content or labels in a tree. It provides a way to convert a value
/// to a colored string representation.
pub trait Content: Any {
  fn to_string(&self, theme: &Theme) -> String;
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl<'value, 'context, T: TreeDisplay> Formatter<'value, 'context, T> {
  /// Creates a new formatter for the given value.
  pub fn of(value: &'value T) -> Self {
    Self {
      value,
      theme: None,
      context: None,
    }
  }

  /// Sets the theme for this formatter.
  pub fn theme(&mut self, theme: Theme) -> &mut Self {
    self.theme = Some(theme);
    self
  }

  /// Sets the context for this formatter.
  ///
  /// The context provides custom mappers for transforming values during tree display.
  pub fn context(&mut self, context: &'context Context) -> &mut Self {
    self.context = Some(context);
    self
  }

  /// Formats the tree and returns a string.
  pub fn format(&self) -> String {
    let mut result = String::new();
    let theme = self.theme.unwrap_or(Theme::default());
    let empty_context = Context::new();
    let context = self.context.unwrap_or(&empty_context);
    self.value.tree(&context).write_root(&mut result, &theme);
    result
  }
}

/// A type name for tree nodes.
///
/// Wraps a string that represents a type name, displayed with the type color from the theme.
pub struct TypeName(pub String);

/// A keyword for tree nodes.
///
/// Wraps a string that represents a keyword-like value, displayed with the keyword color from the theme.
pub struct Keyword(pub String);

/// A member name for tree nodes.
///
/// Wraps a string that represents a member/field name, displayed with the member color from the theme.
pub struct Member(pub String);

/// An index value for tree nodes.
///
/// Wraps a value that is displayed as an index, typically used for array or tuple indexing.
pub struct Index(pub Box<dyn Content>);

impl TypeName {
  /// Creates a new type name from a string.
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Member {
  /// Creates a new member name from a string.
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Keyword {
  /// Creates a new keyword from a string.
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Index {
  /// Creates a new index from a content value.
  pub fn new(value: impl Content) -> Self {
    Self(Box::new(value))
  }
}

// ──── Impl ──────────────────────────────────────────────────────────────────────────────────────

impl Content for TypeName {
  fn to_string(&self, theme: &Theme) -> String {
    self.0.clone().fg(theme.colors.types)
  }
}

impl Content for Keyword {
  fn to_string(&self, theme: &Theme) -> String {
    self.0.clone().fg(theme.colors.keywords)
  }
}

impl Content for Member {
  fn to_string(&self, theme: &Theme) -> String {
    self.0.clone().fg(theme.colors.members)
  }
}

impl Content for Index {
  fn to_string(&self, theme: &Theme) -> String {
    format!("[{}]", self.0.to_string(theme))
  }
}

impl Content for Box<dyn Content> {
  fn to_string(&self, theme: &Theme) -> String {
    (**self).to_string(theme)
  }
}

/// Blanket implementation of `Content` for any `Debug + Any` type.
///
/// This provides automatic content conversion for most Rust types:
/// - Strings and `&str` are displayed with string coloring
/// - All other types are displayed with value coloring
impl<T: std::fmt::Debug + Any> Content for T {
  fn to_string(&self, theme: &Theme) -> String {
    if let Some(s) = (self as &dyn Any).downcast_ref::<String>() {
      format!("\"{}\"", s).fg(theme.colors.strings)
    } else if let Some(s) = (self as &dyn Any).downcast_ref::<&str>() {
      format!("\"{}\"", s).fg(theme.colors.strings)
    } else {
      format!("{:?}", self).fg(theme.colors.values)
    }
  }
}

// ──── Rendering ─────────────────────────────────────────────────────────────────────────────────

impl Theme {
  /// Returns the connector string for a tree node.
  ///
  /// The connector is the line segment that connects a node to its parent:
  fn connector(&self, is_leaf: bool, is_last: bool) -> String {
    let vertical = if is_last {
      self.lines.end
    } else {
      self.lines.junction
    };
    let horizontal = if is_leaf {
      self.lines.horizontal
    } else {
      self.lines.tip
    };
    format!("{}{} ", vertical, horizontal).fg(self.colors.branches)
  }

  /// Returns the indentation string for the next level of the tree.
  ///
  /// This determines what prefix is added to child nodes when rendering
  /// the tree. It creates the visual "spine" that connects siblings.
  fn continuation(&self, is_last: bool) -> String {
    if is_last {
      "   ".to_string()
    } else {
      format!("{}  ", self.lines.vertical).fg(self.colors.vertical)
    }
  }
}

impl Tree {
  /// Writes the root of the tree to the output string.
  ///
  /// This is the entry point for tree rendering. It writes the root
  /// content and then recursively renders all children.
  fn write_root(&self, out: &mut String, theme: &Theme) {
    out.push_str(&self.content.to_string(theme));

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, "", index == self.subtrees.len(), theme);
    }
  }

  /// Recursively writes a tree node and its descendants.
  ///
  /// This handles indentation, connector drawing, labels, and proper alignment of child nodes.
  fn write(&self, out: &mut String, prefix: &str, is_last: bool, theme: &Theme) {
    out.push_str(prefix);
    let connector = theme.connector(self.is_leaf(), is_last);
    out.push_str(&connector);

    let mut offset = 0;
    if let Some(label) = &self.label {
      let label_string = label.to_string(theme);
      if theme.align_to_values {
        offset = label_string.stripped().len() + 2;
      }
      out.push_str(&label_string);
      out.push_str(": ");
    }
    out.push_str(&self.content.to_string(theme));

    let padding = theme.continuation(is_last);
    let next_prefix = format!("{}{}{}", prefix, padding, " ".repeat(offset),);

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, &next_prefix, index == self.subtrees.len(), theme);
    }
  }
}
