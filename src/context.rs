//! Custom value transformations for tree display.
//!
//! Register mappers to modify how values are displayed without changing
//! the underlying data. Useful for formatting, redaction, or debugging
//! transformations.

use super::format::Content;
use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

/// A function that maps one content value to another.
pub type Mapper = dyn Fn(&dyn Content) -> Box<dyn Content>;

/// Context for custom value transformations.
///
/// A context holds a collection of mappers keyed by the [`TypeId`] of the
/// input type. When displaying a tree, the context is used to apply
/// transformations to values before they are rendered.
///
/// ## Example
/// ```no_run
/// use tree_display::Context;
///
/// let context = Context::new()
///     .map(|s: &String| format!("'{}'", s))
///     .map(|n: &i32| format!("{}", n * 2));
/// ```
#[derive(Default)]
pub struct Context {
  /// Map from [`TypeId`] to a mapper function for that type.
  pub mappers: HashMap<TypeId, Box<Mapper>>,
}

impl Context {
  /// Creates a new empty context with no mappers.
  pub fn new() -> Self {
    Self {
      mappers: HashMap::new(),
    }
  }

  /// Registers a mapper for a specific type.
  ///
  /// The mapper function takes a reference to a value and returns another
  /// value that implements `Content`. The transformed value will
  /// be used when displaying values of type `T` in the tree.
  ///
  /// To apply a mapper to a specific field in a struct or enum, use the
  /// `#[tree(map)]` attribute on that field when deriving [`TreeDisplay`](`super::TreeDisplay`).
  ///
  /// ## Type Parameters
  /// - `T`: The input type to map from
  /// - `R`: The output type that implements [`Content`]
  ///
  /// ## Example
  /// ```no_run
  /// use tree_display::context::Context;
  ///
  /// let context = Context::new()
  ///     .map(|s: &String| format!("'{}'", s))
  ///     .map(|n: &i32| format!("{}", n * 2));
  /// ```
  pub fn map<T: 'static, R: Content + 'static>(mut self, f: impl Fn(&T) -> R + 'static) -> Self {
    self.mappers.insert(
      TypeId::of::<T>(),
      Box::new(move |content| {
        let t = (content as &dyn Any)
          .downcast_ref::<T>()
          .expect("TypeId guarantees this matches");
        Box::new(f(t))
      }),
    );
    self
  }
}
