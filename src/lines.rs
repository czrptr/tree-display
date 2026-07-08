// ──── API ───────────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lines {
  pub vertical: char,
  pub horizontal: char,
  pub connector: char,
  pub corner: char,
  pub end: char,
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
}

// ──── Utility ───────────────────────────────────────────────────────────────────────────────────

impl Lines {
  pub const fn new() -> Self {
    Self::ASCII
  }

  pub const fn vertical(mut self, c: char) -> Self {
    self.vertical = c;
    self
  }

  pub const fn horizontal(mut self, c: char) -> Self {
    self.horizontal = c;
    self
  }

  pub const fn connector(mut self, c: char) -> Self {
    self.connector = c;
    self
  }

  pub const fn corner(mut self, c: char) -> Self {
    self.corner = c;
    self
  }

  pub const fn end(mut self, c: char) -> Self {
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
