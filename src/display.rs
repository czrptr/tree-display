use super::Tree;

#[cfg(not(feature = "color"))]
impl Tree {
  pub(crate) fn write_root(&self, out: &mut String) {
    out.push_str(&self.content);

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
      out.push_str(label);
      out.push_str(": ");
    }
    out.push_str(&self.content);

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

#[cfg(feature = "color")]
pub use super::theme::*;

#[cfg(feature = "color")]
impl Tree {
  pub(crate) fn write_root(&self, out: &mut String) {
    let content = if self.is_leaf() {
      &self.content
    } else {
      &self.content.fg(get_theme().colors.types)
    };
    out.push_str(content);

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
      out.push_str(&label.fg(theme.colors.fields));
      out.push_str(": ");
    }
    out.push_str(&self.content.fg(theme.colors.values));

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
