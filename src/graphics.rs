//! Line characters used for drawing tree structures in the terminal.
//!
//! This module provides various line styles for tree rendering, from simple ASCII
//! to Unicode box-drawing characters. You can customize the appearance of tree
//! lines, corners, and connectors to match your preferred aesthetic.

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

/// Character set used for drawing tree graphics in the terminal.
///
/// Each tree display uses a set of five characters to render the hierarchical
/// structure. Different styles are available for ASCII-only terminals,
/// Unicode-capable terminals, and various aesthetic preferences.
///
/// ## Example
/// ```no_run
/// use tree_display::Graphics;
///
/// // Use ASCII characters for maximum compatibility
/// let ascii = Graphics::ASCII;
///
/// // Use modern Unicode box-drawing characters
/// let light = Graphics::LIGHT;
///
/// // Customize individual characters
/// let custom = Graphics::new()
///   .vertical('│')
///   .horizontal('─')
///   .tip('└');
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Graphics {
  /// Character for horizontal lines connecting to labels
  pub horizontal: char,
  /// Character for vertical lines connecting tree levels
  pub vertical: char,
  /// Character for branch junctions (middle child)
  pub junction: char,
  /// Character for the tip/end of a branch (last child)
  pub tip: char,
  /// Character for the end of the tree (root connector)
  pub end: char,
}

// ──── Predefined themes ─────────────────────────────────────────────────────────────────────────

impl Graphics {
  /// Minimal graphics with no visible characters (spaces only).
  ///
  /// Uses ` ` for all characters, effectively disabling tree lines.
  pub const BLANK: Self = Self::new().all(' ');

  /// Pure ASCII characters (compatible with all terminals).
  ///
  /// Uses `-`, `|`, `+`, `-`, and `` ` `` for tree drawing.
  pub const ASCII: Self = Self::new()
    .horizontal('-')
    .vertical('|')
    .junction('+')
    .tip('-')
    .end('`');

  /// Light-weight Unicode box-drawing characters.
  ///
  /// Uses `─`, `│`, `├`, `─`, and `└` for tree drawing.
  pub const LIGHT: Self = Self::new()
    .horizontal('─')
    .vertical('│')
    .junction('├')
    .tip('─')
    .end('└');

  /// Light-weight Unicode box-drawing characters with dotted lines.
  ///
  /// Uses `╌`, `│`, `├`, `─`, and `└` for tree drawing.
  pub const LIGHT_DOTTED: Self = Self::new()
    .horizontal('╌')
    .vertical('╎')
    .junction('├')
    .tip('─')
    .end('└');

  /// Light-weight Unicode box-drawing characters with rounded corners.
  ///
  /// Uses `─`, `│`, `├`, `─`, and `╰` for tree drawing.
  pub const LIGHT_ROUNDED: Self = Self::new()
    .horizontal('─')
    .vertical('│')
    .junction('├')
    .tip('─')
    .end('╰');

  /// Light-weight Unicode box-drawing characters with dotted lines and rounded corners.
  ///
  /// Uses `╌`, `│`, `├`, `─`, and `╰` for tree drawing.
  pub const LIGHT_DOTTED_ROUNDED: Self = Self::new()
    .horizontal('╌')
    .vertical('╎')
    .junction('├')
    .tip('─')
    .end('╰');

  /// Double-line Unicode box-drawing characters.
  ///
  /// Uses `═`, `║`, `╠`, `━`, and `╚` for tree drawing.
  pub const DOUBLE: Self = Self::new()
    .horizontal('═')
    .vertical('║')
    .junction('╠')
    .tip('━')
    .end('╚');

  /// Heavy-weight Unicode box-drawing characters.
  ///
  /// Uses `━`, `┃`, `┣`, `━`, and `┗` for tree drawing.
  pub const HEAVY: Self = Self::new()
    .horizontal('━')
    .vertical('┃')
    .junction('┣')
    .tip('━')
    .end('┗');

  /// Heavy-weight Unicode box-drawing characters with dotted lines.
  ///
  /// Uses `╍`, `╏`, `┣`, `━`, and `┗` for tree drawing.
  pub const HEAVY_DOTTED: Self = Self::new()
    .horizontal('╍')
    .vertical('╏')
    .junction('┣')
    .tip('━')
    .end('┗');
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Graphics {
  /// Creates a new [`Graphics`] instance with `LIGHT` characters.
  pub const fn new() -> Self {
    Self {
      horizontal: '─',
      vertical: '│',
      junction: '├',
      tip: '─',
      end: '└',
    }
  }

  /// Sets the horizontal line character.
  ///
  /// This character is used for horizontal connections extending from vertical lines to labels.
  pub const fn horizontal(mut self, c: char) -> Self {
    self.horizontal = c;
    self
  }

  /// Sets the vertical line character.
  ///
  /// This character is used for vertical lines that connect tree levels.
  pub const fn vertical(mut self, c: char) -> Self {
    self.vertical = c;
    self
  }

  /// Sets the junction character.
  ///
  /// This character is used at branch junctions where horizontal and vertical lines meet.
  /// It represents the "T-junction" where a branch splits.
  pub const fn junction(mut self, c: char) -> Self {
    self.junction = c;
    self
  }

  /// Sets the tip character.
  ///
  /// This character is used for the tip/end of a branch (the last child).
  pub const fn tip(mut self, c: char) -> Self {
    self.tip = c;
    self
  }

  /// Sets the end character.
  ///
  /// This character is used at the root of the tree or as the final
  /// connector in the hierarchy. In some styles, it may be the same
  /// as the tip character.
  pub const fn end(mut self, c: char) -> Self {
    self.end = c;
    self
  }

  /// Sets all graphics characters to the same value.
  ///
  /// This is a convenience method for quickly creating a uniform style
  /// where all characters are identical.
  pub const fn all(mut self, c: char) -> Self {
    self.horizontal = c;
    self.vertical = c;
    self.junction = c;
    self.tip = c;
    self.end = c;
    self
  }
}

impl Default for Graphics {
  /// Creates a new [`Graphics`] instance with `LIGHT` characters.
  fn default() -> Self {
    Self::new()
  }
}
