//! Color and line graphics bundled into a cohesive visual style.
//!
//! A theme brings together color palettes and line characters to create
//! a consistent look for tree output. Choose from predefined themes or
//! mix and match colors and graphics to suit your preferences.

use super::color::Colors;
use super::graphics::Graphics;

// ──── API ─────────────────────────────────────────────────────────────────────────────────────

/// A complete theme for tree display.
///
/// A theme combines color configuration and line graphics to define the
/// visual appearance of tree output. Themes control everything from syntax
/// highlighting colors to the characters used for drawing tree lines.
///
/// ## Example
/// ```no_run
/// use tree_display::{theme::Theme, color::Colors, graphics::Graphics};
///
/// // Use the default theme
/// let theme = Theme::default();
///
/// // Create a custom theme
/// let custom = Theme::new()
///     .colors(Colors::VSCODE_DARK_PLUS)
///     .lines(Graphics::LIGHT_ROUNDED)
///     .align_to_values(true);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Theme {
  /// Color configuration for syntax highlighting
  pub colors: Colors,
  /// Line characters for tree drawing
  pub lines: Graphics,
  /// Whether to align labels with values vertically
  ///
  /// When `true`: subtrees are aligned with the values they describe.
  ///
  /// When `false`: all subtrees have the same consistent indentation.
  pub align_to_values: bool,
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Theme {
  /// Creates a new [`Theme`] with default settings.
  ///
  /// The default theme uses no colors, light Unicode lines, and no value alignment.
  pub const fn new() -> Self {
    Self {
      colors: Colors::new(),
      lines: Graphics::new(),
      align_to_values: false,
    }
  }

  /// Sets the color configuration for this theme.
  pub const fn colors(mut self, colors: Colors) -> Self {
    self.colors = colors;
    self
  }

  /// Sets the line graphics for this theme.
  pub const fn lines(mut self, lines: Graphics) -> Self {
    self.lines = lines;
    self
  }

  /// Sets whether subtrees should be aligned with values.
  ///
  /// When `true`: subtrees are aligned with the values they describe.
  ///
  /// When `false`: all subtrees have the same consistent indentation.
  pub const fn align_to_values(mut self, align_to_values: bool) -> Self {
    self.align_to_values = align_to_values;
    self
  }
}

impl Default for Theme {
  /// Creates a new [`Theme`] with default settings.
  ///
  /// The default theme uses no colors, light Unicode lines, and no value alignment.
  fn default() -> Self {
    Self::new()
  }
}
