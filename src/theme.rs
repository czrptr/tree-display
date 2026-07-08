use anstyle::Color;
use std::sync::OnceLock;

static THEME: OnceLock<Theme> = OnceLock::new();

pub fn get_theme() -> &'static Theme {
  THEME.get().unwrap_or(&Theme {
    colors: Colors::VSCODE_DARK_PLUS,
    lines: Lines::LIGHT_ROUNDED,
    align_to_values: false,
  })
}

pub fn set_theme(theme: Theme) -> Result<(), Theme> {
  THEME.set(theme)
}

// ──── Theme ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Theme {
  pub colors: Colors,
  pub lines: Lines,
  pub align_to_values: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors {
  pub lines: Option<Color>,
  pub types: Option<Color>,
  pub fields: Option<Color>,
  pub values: Option<Color>,
  pub strings: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lines {
  pub vertical: char,
  pub horizontal: char,
  pub connector: char,
  pub corner: char,
  pub end: char,
}

impl Theme {
  pub fn new() -> Self {
    Self {
      colors: Colors::NONE,
      lines: Lines::ASCII,
      align_to_values: false,
    }
  }

  pub fn colors(mut self, colors: Colors) -> Self {
    self.colors = colors;
    self
  }

  pub fn lines(mut self, lines: Lines) -> Self {
    self.lines = lines;
    self
  }

  pub fn align_to_values(mut self, align_to_values: bool) -> Self {
    self.align_to_values = align_to_values;
    self
  }
}

// ──── Predefined themes ─────────────────────────────────────────────────────────────────────────

impl Lines {
  pub const ASCII: Self = Self {
    vertical: '|',
    horizontal: '-',
    connector: '-',
    corner: '+',
    end: '`',
  };

  pub const LIGHT: Self = Self {
    vertical: '│',
    horizontal: '─',
    connector: '─',
    corner: '├',
    end: '└',
  };

  pub const LIGHT_DOTTED: Self = Self {
    vertical: '╎',
    horizontal: '╌',
    connector: '─',
    corner: '├',
    end: '└',
  };

  pub const LIGHT_ROUNDED: Self = Self {
    vertical: '│',
    horizontal: '─',
    connector: '─',
    corner: '├',
    end: '╰',
  };

  pub const LIGHT_DOTTED_ROUNDED: Self = Self {
    vertical: '╎',
    horizontal: '╌',
    connector: '─',
    corner: '├',
    end: '╰',
  };

  pub const DOUBLE: Self = Self {
    vertical: '║',
    horizontal: '═',
    connector: '╸',
    corner: '╠',
    end: '╚',
  };

  pub const HEAVY: Self = Self {
    vertical: '┃',
    horizontal: '━',
    connector: '╸',
    corner: '┣',
    end: '┗',
  };

  pub const HEAVY_DOTTED: Self = Self {
    vertical: '╏',
    horizontal: '╍',
    connector: '╸',
    corner: '┣',
    end: '┗',
  };

  pub fn new() -> Self {
    Self::ASCII
  }

  pub fn vertical(mut self, c: char) -> Self {
    self.vertical = c;
    self
  }

  pub fn horizontal(mut self, c: char) -> Self {
    self.horizontal = c;
    self
  }

  pub fn connector(mut self, c: char) -> Self {
    self.connector = c;
    self
  }

  pub fn corner(mut self, c: char) -> Self {
    self.corner = c;
    self
  }

  pub fn end(mut self, c: char) -> Self {
    self.end = c;
    self
  }

  pub(crate) fn connector_str(&self, is_leaf: bool, is_last: bool) -> String {
    let connector_char = if is_leaf {
      self.horizontal
    } else {
      self.connector
    };
    let corner_char = if is_last { self.end } else { self.corner };
    format!("{}{} ", corner_char, connector_char)
  }

  pub(crate) fn continuation_str(&self, is_last: bool) -> String {
    if is_last {
      "   ".to_string()
    } else {
      format!("{}  ", self.vertical)
    }
  }
}

impl Colors {
  pub const NONE: Self = Self {
    lines: None,
    types: None,
    fields: None,
    values: None,
    strings: None,
  };

  pub const VSCODE_DARK_PLUS: Self = Self {
    lines: Some(ansi256(32)),
    types: Some(ansi256(79)),
    fields: Some(ansi256(153)),
    values: Some(ansi256(187)),
    strings: Some(ansi256(173)),
  };

  pub fn new() -> Self {
    Self::NONE
  }

  pub fn lines(mut self, color: impl Into<Option<Color>>) -> Self {
    self.lines = color.into();
    self
  }

  pub fn types(mut self, color: impl Into<Option<Color>>) -> Self {
    self.types = color.into();
    self
  }

  pub fn fields(mut self, color: impl Into<Option<Color>>) -> Self {
    self.fields = color.into();
    self
  }

  pub fn values(mut self, color: impl Into<Option<Color>>) -> Self {
    self.values = color.into();
    self
  }

  pub fn strings(mut self, color: impl Into<Option<Color>>) -> Self {
    self.strings = color.into();
    self
  }

  pub fn all(mut self, color: impl Into<Color>) -> Self {
    let c = color.into();
    self.lines = Some(c);
    self.types = Some(c);
    self.fields = Some(c);
    self.fields = Some(c);
    self.strings = Some(c);
    self
  }
}

// ──── Impl ──────────────────────────────────────────────────────────────────────────────────────

pub(crate) use anstyle::Style;

const fn ansi256(color: u8) -> Color {
  Color::Ansi256(anstyle::Ansi256Color(color))
}

pub(crate) trait Styled {
  fn styled(&self, style: &Style) -> String;
  fn fg(&self, color: Option<Color>) -> String;
  fn stripped(&self) -> String;
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

  fn stripped(&self) -> String {
    use anstream::adapter::strip_str;
    strip_str(self.as_ref()).to_string()
  }
}
