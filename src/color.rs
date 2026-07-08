#[cfg(feature = "color")]
pub use anstyle::{Color, Style};

#[cfg(not(feature = "color"))]
pub type Color = ();
#[cfg(not(feature = "color"))]
pub type Style = ();

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors {
  pub types: Option<Color>,
  pub fields: Option<Color>,
  pub values: Option<Color>,
  pub strings: Option<Color>,
  pub lines: Option<Color>,
  pub vline: Option<Color>,
}

// ──── Predefined themes ─────────────────────────────────────────────────────────────────────────

impl Colors {
  pub const NONE: Self = Self {
    types: None,
    fields: None,
    values: None,
    strings: None,
    lines: None,
    vline: None,
  };

  pub const VSCODE_DARK_PLUS: Self = Self {
    types: Some(ansi256(79)),
    fields: Some(ansi256(153)),
    values: Some(ansi256(187)),
    strings: Some(ansi256(173)),
    lines: Some(ansi256(32)),
    vline: Some(ansi256(24)),
  };
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Colors {
  pub const fn new() -> Self {
    Self::NONE
  }

  pub const fn types(mut self, color: Color) -> Self {
    self.types = Some(color);
    self
  }

  pub const fn fields(mut self, color: Color) -> Self {
    self.fields = Some(color);
    self
  }

  pub const fn values(mut self, color: Color) -> Self {
    self.values = Some(color);
    self
  }

  pub const fn strings(mut self, color: Color) -> Self {
    self.strings = Some(color);
    self
  }

  pub const fn lines(mut self, color: Color) -> Self {
    self.lines = Some(color);
    self
  }

  pub const fn vline(mut self, color: Color) -> Self {
    self.vline = Some(color);
    self
  }

  pub const fn all(mut self, color: Color) -> Self {
    let c = Some(color);
    self.lines = c;
    self.types = c;
    self.fields = c;
    self.fields = c;
    self.strings = c;
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
