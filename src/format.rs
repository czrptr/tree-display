use super::{Tree, TreeDisplay, color::Colored, context::Context, theme::Theme};
use std::any::Any;

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

pub struct Formatter<'value, 'context, T: TreeDisplay> {
  value: &'value T,
  theme: Option<Theme>,
  context: Option<&'context Context>,
}

pub trait Content: Any {
  fn to_string(&self, theme: &Theme) -> String;
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl<'value, 'context, T: TreeDisplay> Formatter<'value, 'context, T> {
  pub fn of(value: &'value T) -> Self {
    Self {
      value,
      theme: None,
      context: None,
    }
  }

  pub fn with_theme(&mut self, theme: &Theme) -> &mut Self {
    self.theme = Some(theme.clone());
    self
  }

  pub fn with_context(&mut self, context: &'context Context) -> &mut Self {
    self.context = Some(context);
    self
  }

  pub fn format(&self) -> String {
    let mut result = String::new();
    let theme = self.theme.unwrap_or(Theme::default());
    let empty_context = Context::new();
    let context = self.context.unwrap_or(&empty_context);
    self.value.tree(&context).write_root(&mut result, &theme);
    result
  }
}

pub struct TypeName(pub String);

pub struct Keyword(pub String);

pub struct Member(pub String);

pub struct Index(pub Box<dyn Content>);

impl TypeName {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Member {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Keyword {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

impl Index {
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

impl Tree {
  fn write_root(&self, out: &mut String, theme: &Theme) {
    out.push_str(&self.content.to_string(theme));

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, "", index == self.subtrees.len(), theme);
    }
  }

  fn write(&self, out: &mut String, prefix: &str, is_last: bool, theme: &Theme) {
    out.push_str(prefix);
    let connector = theme.connector_str(self.is_leaf(), is_last);
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

    let padding = theme.continuation_str(is_last);
    // TODO: use darker line color and dots for this case
    let next_prefix = format!("{}{}{}", prefix, padding, " ".repeat(offset),);

    let mut index = 0;
    for child in &self.subtrees {
      index += 1;
      out.push('\n');
      child.write(out, &next_prefix, index == self.subtrees.len(), theme);
    }
  }
}
