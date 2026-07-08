#[cfg(feature = "color")]
pub use anstyle::{Color, Style};

#[cfg(not(feature = "color"))]
pub type Color = ();
#[cfg(not(feature = "color"))]
pub type Style = ();

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors {
  pub lines: Option<Color>,
  pub types: Option<Color>,
  pub fields: Option<Color>,
  pub values: Option<Color>,
  pub strings: Option<Color>,
}

// ──── Predefined themes ─────────────────────────────────────────────────────────────────────────

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
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Colors {
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

const fn ansi256(color: u8) -> Color {
  #[cfg(feature = "color")]
  {
    Color::Ansi256(anstyle::Ansi256Color(color))
  }
  #[cfg(not(feature = "color"))]
  {
    _ = color;
    ()
  }
}

pub(crate) trait Colored {
  fn fg(&self, color: Option<Color>) -> String;
  fn stripped(&self) -> String;
}

impl<T: AsRef<str>> Colored for T {
  fn fg(&self, color: Option<Color>) -> String {
    #[cfg(feature = "color")]
    {
      let style = Style::new().fg_color(color);
      let style_intro = style.render().to_string();
      let style_reset = style.render_reset().to_string();
      format!("{}{}{}", style_intro, self.as_ref(), style_reset)
    }
    #[cfg(not(feature = "color"))]
    {
      _ = color;
      self.as_ref().to_string()
    }
  }

  fn stripped(&self) -> String {
    #[cfg(feature = "color")]
    {
      use anstream::adapter::strip_str;
      strip_str(self.as_ref()).to_string()
    }
    #[cfg(not(feature = "color"))]
    {
      self.as_ref().to_string()
    }
  }
}
