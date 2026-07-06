use super::Tree;

#[cfg(not(feature = "color"))]
impl Tree {
  pub(crate) fn write_root(&self, out: &mut String) {
    out.push_str(&self.label);

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
    out.push_str(&self.label);

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
    // Root label: types for non-leaf, plain for leaf (no color)
    let label = if self.is_leaf() {
      &self.label
    } else {
      &self.label.fg(get_theme().colors.types)
    };

    out.push_str(label);

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

    // Split label and write with appropriate colors
    match split_at_colon(&self.label) {
      (value, None) => {
        // Just a value - use value color if leaf, otherwise plain
        if self.is_leaf() {
          out.push_str(&value.fg(theme.colors.values));
        } else {
          out.push_str(&value);
        }
      }
      (label, Some(value)) => {
        // Field label gets field color
        out.push_str(&label.fg(theme.colors.fields));
        out.push_str(": ");

        // Value gets value color if leaf, type color if node
        let color = if self.is_leaf() {
          theme.colors.values
        } else {
          theme.colors.types
        };
        out.push_str(&value.fg(color));
      }
    }

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

fn split_at_colon(s: &str) -> (String, Option<String>) {
  if let Some((base, content)) = s.split_once(':') {
    (base.trim().to_string(), Some(content.trim().to_string()))
  } else {
    (s.trim().to_string(), None)
  }
}
