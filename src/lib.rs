use anstyle::{Ansi256Color, Color, Style};
pub use derive::TreeDisplay;
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

const LINE_COLOR: Option<Color> = Some(Color::Ansi256(Ansi256Color(74)));
const TYPE_COLOR: Option<Color> = Some(Color::Ansi256(Ansi256Color(79)));
const FIELD_COLOR: Option<Color> = Some(Color::Ansi256(Ansi256Color(153)));
const VALUE_COLOR: Option<Color> = Some(Color::Ansi256(Ansi256Color(187)));
const LINE: &Style = &Style::new().fg_color(LINE_COLOR);
const TYPE: &Style = &Style::new().fg_color(TYPE_COLOR);
const FIELD: &Style = &Style::new().fg_color(FIELD_COLOR);
const VALUE: &Style = &Style::new().fg_color(VALUE_COLOR);
const NONE: &Style = &Style::new();

impl Tree {
  fn write_root(&self, out: &mut String) {
    let style = if self.is_leaf() { NONE } else { TYPE };
    out.push_str(&self.label.styled(style));

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
      (value, None) => out.push_str(&value.styled(VALUE)),
      (label, Some(value)) => {
        out.push_str(&label.styled(FIELD));
        out.push_str(": ");

        let style = if self.is_leaf() { VALUE } else { TYPE };
        out.push_str(&value.styled(style));
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
