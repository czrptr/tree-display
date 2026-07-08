use super::{Content, Tree};
use std::any::Any;

pub struct Member(pub String);

impl Member {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

pub struct TypeName(pub String);

impl TypeName {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

pub struct Index(pub Box<dyn Content>);

impl Index {
  pub fn new(value: impl Content) -> Self {
    Self(Box::new(value))
  }
}

#[cfg(not(feature = "color"))]
mod no_color {
  use super::*;

  impl Content for TypeName {
    fn to_string(&self) -> String {
      self.0.clone()
    }
  }
  impl Content for Member {
    fn to_string(&self) -> String {
      self.0.clone()
    }
  }

  impl Content for Index {
    fn to_string(&self) -> String {
      format!("[{}]", self.0.to_string())
    }
  }

  impl<T: std::fmt::Debug + Any> Content for T {
    fn to_string(&self) -> String {
      if let Some(s) = (self as &dyn Any).downcast_ref::<String>() {
        format!("\"{}\"", s)
      } else if let Some(s) = (self as &dyn Any).downcast_ref::<&str>() {
        format!("\"{}\"", s)
      } else {
        format!("{:?}", self)
      }
    }
  }

  impl Tree {
    pub(crate) fn write_root(&self, out: &mut String) {
      out.push_str(&format!("{}", self.content.to_string()));

      let mut index = 0;
      for child in &self.subtrees {
        index += 1;
        out.push('\n');
        child.write(out, "", index == self.subtrees.len());
      }
    }

    pub(crate) fn write(&self, out: &mut String, prefix: &str, last: bool) {
      let connector = if self.is_leaf() { "─" } else { "╼" };
      let line = if last { "╰" } else { "├" };
      let connector = format!("{line}{connector} ");

      out.push_str(prefix);
      out.push_str(&connector);
      if let Some(label) = &self.label {
        out.push_str(&label.to_string());
        out.push_str(": ");
      }
      out.push_str(&format!("{}", self.content.to_string()));

      let padding = if last { "   " } else { "│  " };
      let next_prefix = format!("{prefix}{padding}");

      let mut index = 0;
      for child in &self.subtrees {
        index += 1;
        out.push('\n');
        child.write(out, &next_prefix, index == self.subtrees.len());
      }
    }
  }
}

#[cfg(feature = "color")]
mod color {
  use super::*;
  use crate::theme::*;

  impl Content for TypeName {
    fn to_string(&self) -> String {
      self.0.clone().fg(get_theme().colors.types)
    }
  }

  impl Content for Member {
    fn to_string(&self) -> String {
      self.0.clone().fg(get_theme().colors.fields)
    }
  }

  impl Content for Index {
    fn to_string(&self) -> String {
      format!("[{}]", self.0.to_string())
    }
  }

  impl<T: std::fmt::Debug + Any> Content for T {
    fn to_string(&self) -> String {
      let colors = get_theme().colors;
      if let Some(s) = (self as &dyn Any).downcast_ref::<String>() {
        format!("\"{}\"", s).fg(colors.strings)
      } else if let Some(s) = (self as &dyn Any).downcast_ref::<&str>() {
        format!("\"{}\"", s).fg(colors.strings)
      } else {
        format!("{:?}", self).fg(colors.values)
      }
    }
  }

  impl Tree {
    pub(crate) fn write_root(&self, out: &mut String) {
      out.push_str(&self.content.to_string());

      let mut index = 0;
      for child in &self.subtrees {
        index += 1;
        out.push('\n');
        child.write(out, "", index == self.subtrees.len());
      }
    }

    pub(crate) fn write(&self, out: &mut String, prefix: &str, is_last: bool) {
      let theme = get_theme();

      out.push_str(prefix);
      let connector = theme.lines.connector_str(self.is_leaf(), is_last);
      out.push_str(&connector.fg(theme.colors.lines));

      if let Some(label) = &self.label {
        out.push_str(&label.to_string());
        out.push_str(": ");
      }
      out.push_str(&self.content.to_string());

      let padding = theme.lines.continuation_str(is_last);
      let next_prefix = format!("{}{}", prefix, padding.fg(theme.colors.lines));

      let mut index = 0;
      for child in &self.subtrees {
        index += 1;
        out.push('\n');
        child.write(out, &next_prefix, index == self.subtrees.len());
      }
    }
  }
}
