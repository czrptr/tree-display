#![allow(dead_code)]

use anstyle::{Ansi256Color, Color, Style};
use std::rc::Rc;
use std::sync::Arc;

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
  pub label: String,
  pub subtrees: Vec<Tree>,
}

impl Tree {
  pub fn new(label: impl Into<String>, subtrees: Vec<Tree>) -> Tree {
    Tree {
      label: label.into(),
      subtrees,
    }
  }

  pub fn leaf(label: impl Into<String>) -> Tree {
    Tree {
      label: label.into(),
      subtrees: Vec::new(),
    }
  }

  pub fn is_leaf(&self) -> bool {
    self.subtrees.is_empty()
  }
}

// ──── Impl ──────────────────────────────────────────────────────────────────────────────────────

trait Styled {
  fn styled(&self, style: &Style) -> String;
  fn fg(&self, color: Option<Color>) -> String;
  fn bg(&self, color: Option<Color>) -> String;
}

impl<T: AsRef<str>> Styled for T {
  fn styled(&self, style: &Style) -> String {
    let text = self.as_ref();

    let style_intro = style.render().to_string();
    let style_reset = style.render_reset().to_string();

    let mut out = String::with_capacity(text.len() + style_intro.len() + style_reset.len());

    out.push_str(&style_intro);
    out.push_str(text);
    out.push_str(&style_reset);

    out
  }

  fn fg(&self, color: Option<Color>) -> String {
    self.styled(&Style::new().fg_color(color))
  }

  fn bg(&self, color: Option<Color>) -> String {
    self.styled(&Style::new().bg_color(color))
  }
}

fn split_at_colon(s: &str) -> (String, Option<String>) {
  if let Some((base, content)) = s.split_once(':') {
    (base.trim().to_string(), Some(content.trim().to_string()))
  } else {
    (s.trim().to_string(), None)
  }
}

const LINE_COLOR: Option<Color> = Some(Color::Ansi256(Ansi256Color(243)));
const BLACK: Option<Color> = Some(Color::Ansi256(Ansi256Color(0)));
const LINE: &Style = &Style::new().fg_color(LINE_COLOR);

impl Tree {
  fn write_root(&self, out: &mut String) {
    out.push_str(&self.label);

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, "", index == self.subtrees.len());
    }
  }

  fn write(&self, out: &mut String, prefix: &str, last: bool) {
    let connector = if self.is_leaf() { "─" } else { "╼" };
    let line = if last { "╰" } else { "├" };
    let connector = format!("{line}{connector} ");

    out.push_str(prefix);
    #[rustfmt::skip]
    out.push_str(&connector.styled(LINE));

    match split_at_colon(&self.label) {
      (value, None) => out.push_str(&value),
      (label, Some(value)) => {
        out.push_str(&label.styled(LINE));
        out.push_str(" ");
        out.push_str(&value);
      }
    }

    let next_prefix = format!(
      "{}{}",
      prefix,
      (if last { "   " } else { "│  " }).styled(LINE)
    );

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, &next_prefix, index == self.subtrees.len());
    }
  }
}

impl TreeDisplay for u32 {
  fn tree(&self) -> Tree {
    Tree::leaf(format!("{:?}", self))
  }
}

// 1-element tuple
impl<T0: TreeDisplay> TreeDisplay for (T0,) {
  fn tree(&self) -> Tree {
    let mut tree = Tree::leaf("tuple");
    let child = self.0.tree();
    tree.subtrees.push(child);
    tree
  }
}

// 2-element tuple
impl<T0: TreeDisplay, T1: TreeDisplay> TreeDisplay for (T0, T1) {
  fn tree(&self) -> Tree {
    let mut tree = Tree::leaf("tuple");
    tree.subtrees.push(self.0.tree());
    tree.subtrees.push(self.1.tree());
    tree
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Box<T> {
  fn tree(&self) -> Tree {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for &T {
  fn tree(&self) -> Tree {
    (**self).tree()
  }
}

impl<T: TreeDisplay> TreeDisplay for Option<T> {
  fn tree(&self) -> Tree {
    match self {
      Some(value) => value.tree(),
      None => Tree::leaf("None").into(),
    }
  }
}

impl<T: TreeDisplay> TreeDisplay for Vec<T> {
  fn tree(&self) -> Tree {
    Tree::new(
      "Vec",
      self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect(),
    )
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Rc<T> {
  fn tree(&self) -> Tree {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Arc<T> {
  fn tree(&self) -> Tree {
    (**self).tree()
  }
}
