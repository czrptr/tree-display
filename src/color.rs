//! Colors used for drawing tree structures in the terminal.
//!
//! This module provides color themes for tree display elements including types,
//! keywords, strings, values, and tree line characters.

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

pub use inner::Color;

/// Color configuration for tree display elements.
///
/// Each element of the tree can be individually colored, allowing you
/// to highlight different syntactic categories like types, keywords, strings,
/// and tree lines. Colors are optional, so you can enable only the highlighting
/// you want.
///
/// ## Example
/// ```no_run
/// use tree_display::color::Colors;
///
/// // Use the VS Code Dark+ theme
/// let colors = Colors::VSCODE_DARK_PLUS;
///
/// // Customize individual colors
/// let custom = Colors::new()
///     .types(Colors::ansi256(79))
///     .keywords(Colors::ansi256(74))
///     .strings(Colors::ansi256(173));
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors {
  /// Color for branch lines (horizontal connectors between nodes)
  pub branches: Option<Color>,
  /// Color for vertical tree lines
  pub vertical: Option<Color>,
  /// Color for keyword like values (e.g. None)
  pub keywords: Option<Color>,
  /// Color for struct/enum member names
  pub members: Option<Color>,
  /// Color for string literals
  pub strings: Option<Color>,
  /// Color for literal values (e.g. integers, floats, bools)
  pub values: Option<Color>,
  /// Color for type names (e.g., [`String`], [`Vec`], [`Option`])
  pub types: Option<Color>,
}

// ──── Predefined themes ─────────────────────────────────────────────────────────────────────────

impl Colors {
  /// No colors (all elements use default terminal color).
  pub const NONE: Self = Self::new();

  /// Visual Studio Code Dark+ theme.
  pub const VSCODE_DARK_PLUS: Self = Self::new()
    .branches(inner::ansi256(32))
    .vertical(inner::ansi256(24))
    .keywords(inner::ansi256(74))
    .members(inner::ansi256(153))
    .strings(inner::ansi256(173))
    .values(inner::ansi256(187))
    .types(inner::ansi256(79));

  /// Visual Studio Code Light+ theme.
  pub const VSCODE_LIGHT_PLUS: Self = Self::new()
    .branches(inner::ansi256(28))
    .vertical(inner::ansi256(23))
    .keywords(inner::ansi256(133))
    .members(inner::ansi256(25))
    .strings(inner::ansi256(88))
    .values(inner::ansi256(94))
    .types(inner::ansi256(26));

  /// Solarized Dark theme.
  pub const SOLARIZED_DARK: Self = Self::new()
    .branches(inner::ansi256(101))
    .vertical(inner::ansi256(60))
    .keywords(inner::ansi256(33))
    .members(inner::ansi256(112))
    .strings(inner::ansi256(106))
    .values(inner::ansi256(179))
    .types(inner::ansi256(68));

  /// Solarized Light theme.
  pub const SOLARIZED_LIGHT: Self = Self::new()
    .branches(inner::ansi256(101))
    .vertical(inner::ansi256(60))
    .keywords(inner::ansi256(33))
    .members(inner::ansi256(112))
    .strings(inner::ansi256(106))
    .values(inner::ansi256(179))
    .types(inner::ansi256(68));

  /// Dracula theme.
  pub const DRACULA: Self = Self::new()
    .branches(inner::ansi256(102))
    .vertical(inner::ansi256(59))
    .keywords(inner::ansi256(141))
    .members(inner::ansi256(147))
    .strings(inner::ansi256(114))
    .values(inner::ansi256(186))
    .types(inner::ansi256(147));

  /// Monokai theme.
  pub const MONOKAI: Self = Self::new()
    .branches(inner::ansi256(144))
    .vertical(inner::ansi256(59))
    .keywords(inner::ansi256(204))
    .members(inner::ansi256(147))
    .strings(inner::ansi256(113))
    .values(inner::ansi256(186))
    .types(inner::ansi256(147));

  /// Nord theme.
  pub const NORD: Self = Self::new()
    .branches(inner::ansi256(109))
    .vertical(inner::ansi256(66))
    .keywords(inner::ansi256(117))
    .members(inner::ansi256(148))
    .strings(inner::ansi256(150))
    .values(inner::ansi256(179))
    .types(inner::ansi256(148));

  /// GitHub Dark theme.
  pub const GITHUB_DARK: Self = Self::new()
    .branches(inner::ansi256(102))
    .vertical(inner::ansi256(59))
    .keywords(inner::ansi256(204))
    .members(inner::ansi256(117))
    .strings(inner::ansi256(142))
    .values(inner::ansi256(186))
    .types(inner::ansi256(117));

  /// GitHub Light theme.
  pub const GITHUB_LIGHT: Self = Self::new()
    .branches(inner::ansi256(102))
    .vertical(inner::ansi256(59))
    .keywords(inner::ansi256(204))
    .members(inner::ansi256(26))
    .strings(inner::ansi256(142))
    .values(inner::ansi256(186))
    .types(inner::ansi256(26));
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Colors {
  /// Creates a new [`Colors`] instance with `Visual Studio Code Dark+` colors.
  pub const fn new() -> Self {
    Self {
      branches: None,
      vertical: None,
      keywords: None,
      members: None,
      strings: None,
      values: None,
      types: None,
    }
  }

  /// Sets the color for branch lines.
  ///
  /// Branch lines are the horizontal connectors that extend from vertical lines
  /// to node labels. They form the "arms" of the tree structure.
  pub const fn branches(mut self, color: Color) -> Self {
    self.branches = Some(color);
    self
  }

  /// Sets the color for vertical tree lines.
  ///
  /// Vertical lines connect parent nodes to their children, forming the main
  /// backbone of the tree hierarchy.
  pub const fn vertical(mut self, color: Color) -> Self {
    self.vertical = Some(color);
    self
  }

  /// Sets the color for language keywords.
  ///
  /// Keywords are refer to keyword-like values such as enum variant names.
  pub const fn keywords(mut self, color: Color) -> Self {
    self.keywords = Some(color);
    self
  }

  /// Sets the color for struct/enum member names.
  ///
  /// Member names are the fields of structs and variants of enums. They are
  /// typically displayed alongside their containing type.
  pub const fn members(mut self, color: Color) -> Self {
    self.members = Some(color);
    self
  }

  /// Sets the color for string literals.
  ///
  /// String literals are quoted text values, such as `"hello"` or multi-line string blocks.
  pub const fn strings(mut self, color: Color) -> Self {
    self.strings = Some(color);
    self
  }

  /// Sets the color for literal values.
  ///
  /// Literal values include numbers (`42`, `3.14`), booleans (`true`, `false`),
  /// characters (`'a'`), and other primitive constant values.
  pub const fn values(mut self, color: Color) -> Self {
    self.values = Some(color);
    self
  }

  /// Sets the color for type names.
  ///
  /// Type names are identifiers that refer to types, such as [`String`], [`Vec`],
  /// [`Option`], [`i32`], and user-defined structs and enums.
  pub const fn types(mut self, color: Color) -> Self {
    self.types = Some(color);
    self
  }

  /// Sets all color elements to the same value.
  ///
  /// This is a convenience method for quickly creating a uniform theme
  /// where all elements share the same color.
  pub const fn all(mut self, color: Color) -> Self {
    let c = Some(color);
    self.branches = c;
    self.vertical = c;
    self.keywords = c;
    self.members = c;
    self.strings = c;
    self.values = c;
    self.types = c;
    self
  }
}

impl Default for Colors {
  /// Creates a new [`Colors`] instance with `Visual Studio Code Dark+` colors.
  fn default() -> Self {
    Self::VSCODE_DARK_PLUS
  }
}

// ──── Impl ──────────────────────────────────────────────────────────────────────────────────────

pub(crate) trait Colored {
  fn fg(&self, color: Option<Color>) -> String;
  fn stripped(&self) -> String;
}

#[cfg(feature = "color")]
mod inner {
  pub use anstyle::Color;

  pub(crate) const fn ansi256(color: u8) -> Color {
    Color::Ansi256(anstyle::Ansi256Color(color))
  }

  impl<T: AsRef<str>> super::Colored for T {
    fn fg(&self, color: Option<Color>) -> String {
      let style = anstyle::Style::new().fg_color(color);
      let style_intro = style.render().to_string();
      let style_reset = style.render_reset().to_string();
      format!("{}{}{}", style_intro, self.as_ref(), style_reset)
    }

    fn stripped(&self) -> String {
      use anstream::adapter::strip_str;
      strip_str(self.as_ref()).to_string()
    }
  }
}

#[cfg(not(feature = "color"))]
mod inner {
  /// A placeholder color type used when the `color` feature is disabled.
  ///
  /// All color operations become no-ops, rendering text without ANSI escape codes.
  /// This allows the crate to be used without color dependencies.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Color;

  pub(crate) const fn ansi256(_: u8) -> Color {
    Color
  }

  impl<T: AsRef<str>> super::Colored for T {
    fn fg(&self, _: Option<Color>) -> String {
      self.as_ref().to_string()
    }

    fn stripped(&self) -> String {
      self.as_ref().to_string()
    }
  }
}
