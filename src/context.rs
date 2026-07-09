use super::format::Content;
use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

type Mapper = dyn Fn(&dyn Content) -> Box<dyn Content>;

pub struct Context {
  pub mappers: HashMap<TypeId, Box<Mapper>>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      mappers: HashMap::new(),
    }
  }

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
