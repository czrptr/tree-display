use anstyle::{Ansi256Color, Color, Style};
use std::rc::Rc;
use std::sync::Arc;

pub trait Styled {
  fn styled(&self, style: &Style) -> String;
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
}

const LINE: &Style = &Style::new().fg_color(Some(Color::Ansi256(Ansi256Color(243))));

pub struct Field {
  pub name: String,
  pub value: String,
}

pub struct TreeNode {
  pub label: String,
  pub fields: Vec<Field>,
  pub children: Vec<TreeNode>,
}

impl Field {
  fn write(&self, out: &mut String, prefix: &str, last: bool) {
    out.push_str(prefix);
    out.push_str(&(if last { "╰─ " } else { "├─ " }).styled(LINE));
    out.push_str(&self.name);
    out.push_str(&self.value);
  }
}

impl TreeNode {
  pub fn write_root(&self, out: &mut String) {
    out.push_str(&self.label);

    let total = self.fields.len() + self.children.len();
    let mut index = 0;

    for field in &self.fields {
      index += 1;
      out.push('\n');
      field.write(out, "", index == total);
    }

    for child in &self.children {
      index += 1;
      out.push('\n');
      child.write(out, "", index == total);
    }
  }

  fn write(&self, out: &mut String, prefix: &str, last: bool) {
    out.push_str(prefix);
    out.push_str(&(if last { "╰╼ " } else { "├╼ " }).styled(LINE));
    out.push_str(&self.label);

    let next_prefix = format!(
      "{}{}",
      prefix,
      (if last { "   " } else { "│  " }).styled(LINE)
    );

    let total = self.fields.len() + self.children.len();
    let mut index = 0;

    for field in &self.fields {
      index += 1;
      out.push('\n');
      field.write(out, &next_prefix, index == total);
    }

    for child in &self.children {
      index += 1;
      out.push('\n');
      child.write(out, &next_prefix, index == total);
    }
  }
}

pub trait TreeDisplay {
  fn tree(&self) -> TreeNode;
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Box<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for &T {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay> TreeDisplay for Option<T> {
  fn tree(&self) -> TreeNode {
    match self {
      Some(value) => value.tree(),
      None => TreeNode {
        label: "None".into(),
        fields: Vec::new(),
        children: Vec::new(),
      },
    }
  }
}

impl<T: TreeDisplay> TreeDisplay for Vec<T> {
  fn tree(&self) -> TreeNode {
    TreeNode {
      label: String::from("Vec"),
      fields: vec![Field {
        name: "len".into(),
        value: self.len().to_string(),
      }],
      children: self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect(),
    }
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Rc<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

impl<T: TreeDisplay + ?Sized> TreeDisplay for Arc<T> {
  fn tree(&self) -> TreeNode {
    (**self).tree()
  }
}

pub fn tree_format<T: TreeDisplay>(value: &T) -> String {
  let mut result = String::new();
  value.tree().write_root(&mut result);
  result
}
