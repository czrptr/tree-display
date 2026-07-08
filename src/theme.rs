use super::color::*;
use super::lines::*;

// ──── API ─────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Theme {
  pub colors: Colors,
  pub lines: Lines,
  pub align_to_values: bool,
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Theme {
  pub const fn default() -> Self {
    Self {
      colors: Colors::NONE,
      lines: Lines::ASCII,
      align_to_values: false,
    }
  }

  pub const fn colors(mut self, colors: Colors) -> Self {
    self.colors = colors;
    self
  }

  pub const fn lines(mut self, lines: Lines) -> Self {
    self.lines = lines;
    self
  }

  pub const fn align_to_values(mut self, align_to_values: bool) -> Self {
    self.align_to_values = align_to_values;
    self
  }
}

impl Default for Theme {
  fn default() -> Self {
    Self::default()
  }
}
