use crate::{Tree, TreeDisplay};

mod support {
  use super::*;

  macro_rules! impl_tree_display_for_primitive {
    ($($ty:ty),* $(,)?) => {
      $(
        impl TreeDisplay for $ty {
          fn tree(&self) -> Tree {
            Tree::leaf(format!("{:?}", self))
          }
        }
      )*
    };
  }

  impl_tree_display_for_primitive!(
    bool,
    char,
    String,
    &str,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize,
  );

  // Empty tuple (unit)
  impl TreeDisplay for () {
    fn tree(&self) -> Tree {
      Tree::leaf("()")
    }
  }

  // 1-element tuple
  impl<T0: TreeDisplay> TreeDisplay for (T0,) {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree
    }
  }

  // 2-element tuple
  impl<T0: TreeDisplay, T1: TreeDisplay> TreeDisplay for (T0, T1) {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree
    }
  }

  // 3-element tuple
  impl<T0: TreeDisplay, T1: TreeDisplay, T2: TreeDisplay> TreeDisplay for (T0, T1, T2) {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree
    }
  }

  // 4-element tuple
  impl<T0: TreeDisplay, T1: TreeDisplay, T2: TreeDisplay, T3: TreeDisplay> TreeDisplay
    for (T0, T1, T2, T3)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree
    }
  }

  // 5-element tuple
  impl<T0: TreeDisplay, T1: TreeDisplay, T2: TreeDisplay, T3: TreeDisplay, T4: TreeDisplay>
    TreeDisplay for (T0, T1, T2, T3, T4)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree
    }
  }

  // 6-element tuple
  impl<
    T0: TreeDisplay,
    T1: TreeDisplay,
    T2: TreeDisplay,
    T3: TreeDisplay,
    T4: TreeDisplay,
    T5: TreeDisplay,
  > TreeDisplay for (T0, T1, T2, T3, T4, T5)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree.subtrees.push(self.5.tree());
      tree
    }
  }

  // 7-element tuple
  impl<
    T0: TreeDisplay,
    T1: TreeDisplay,
    T2: TreeDisplay,
    T3: TreeDisplay,
    T4: TreeDisplay,
    T5: TreeDisplay,
    T6: TreeDisplay,
  > TreeDisplay for (T0, T1, T2, T3, T4, T5, T6)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree.subtrees.push(self.5.tree());
      tree.subtrees.push(self.6.tree());
      tree
    }
  }

  // 8-element tuple
  impl<
    T0: TreeDisplay,
    T1: TreeDisplay,
    T2: TreeDisplay,
    T3: TreeDisplay,
    T4: TreeDisplay,
    T5: TreeDisplay,
    T6: TreeDisplay,
    T7: TreeDisplay,
  > TreeDisplay for (T0, T1, T2, T3, T4, T5, T6, T7)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree.subtrees.push(self.5.tree());
      tree.subtrees.push(self.6.tree());
      tree.subtrees.push(self.7.tree());
      tree
    }
  }

  // 9-element tuple
  impl<
    T0: TreeDisplay,
    T1: TreeDisplay,
    T2: TreeDisplay,
    T3: TreeDisplay,
    T4: TreeDisplay,
    T5: TreeDisplay,
    T6: TreeDisplay,
    T7: TreeDisplay,
    T8: TreeDisplay,
  > TreeDisplay for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree.subtrees.push(self.5.tree());
      tree.subtrees.push(self.6.tree());
      tree.subtrees.push(self.7.tree());
      tree.subtrees.push(self.8.tree());
      tree
    }
  }

  // 10-element tuple
  impl<
    T0: TreeDisplay,
    T1: TreeDisplay,
    T2: TreeDisplay,
    T3: TreeDisplay,
    T4: TreeDisplay,
    T5: TreeDisplay,
    T6: TreeDisplay,
    T7: TreeDisplay,
    T8: TreeDisplay,
    T9: TreeDisplay,
  > TreeDisplay for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
  {
    fn tree(&self) -> Tree {
      let mut tree = Tree::leaf("tuple");
      tree.subtrees.push(self.0.tree());
      tree.subtrees.push(self.1.tree());
      tree.subtrees.push(self.2.tree());
      tree.subtrees.push(self.3.tree());
      tree.subtrees.push(self.4.tree());
      tree.subtrees.push(self.5.tree());
      tree.subtrees.push(self.6.tree());
      tree.subtrees.push(self.7.tree());
      tree.subtrees.push(self.8.tree());
      tree.subtrees.push(self.9.tree());
      tree
    }
  }

  impl<T: TreeDisplay, const N: usize> TreeDisplay for [T; N] {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      Tree::new(format!("Array<{}>", N), children)
    }
  }

  impl<T: TreeDisplay + ?Sized> TreeDisplay for &T {
    fn tree(&self) -> Tree {
      (**self).tree()
    }
  }

  impl<T: TreeDisplay + ?Sized> TreeDisplay for &mut T {
    fn tree(&self) -> Tree {
      (**self).tree()
    }
  }

  impl<T: TreeDisplay + ?Sized> TreeDisplay for Box<T> {
    fn tree(&self) -> Tree {
      (**self).tree()
    }
  }

  impl<T: TreeDisplay + ?Sized> TreeDisplay for std::rc::Rc<T> {
    fn tree(&self) -> Tree {
      (**self).tree()
    }
  }

  impl<T: TreeDisplay + ?Sized> TreeDisplay for std::sync::Arc<T> {
    fn tree(&self) -> Tree {
      (**self).tree()
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::cell::RefCell<T> {
    fn tree(&self) -> Tree {
      self.borrow().tree()
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::sync::Mutex<T> {
    fn tree(&self) -> Tree {
      self.lock().unwrap().tree()
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::sync::RwLock<T> {
    fn tree(&self) -> Tree {
      self.read().unwrap().tree()
    }
  }

  impl<T: TreeDisplay> TreeDisplay for Option<T> {
    fn tree(&self) -> Tree {
      match self {
        Some(value) => value.tree(),
        None => Tree::leaf("None"),
      }
    }
  }

  impl<T: TreeDisplay, E: TreeDisplay> TreeDisplay for Result<T, E> {
    fn tree(&self) -> Tree {
      match self {
        Ok(value) => {
          let mut node = value.tree();
          node.label = format!("Ok({})", node.label);
          node
        }
        Err(err) => {
          let mut node = err.tree();
          node.label = format!("Err({})", node.label);
          node
        }
      }
    }
  }

  impl<T: TreeDisplay> TreeDisplay for Vec<T> {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      Tree::new(format!("Vec (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for [T] {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      Tree::new(format!("Slice (len: {})", self.len()), children)
    }
  }

  impl<K: TreeDisplay, V: TreeDisplay, S> TreeDisplay for std::collections::HashMap<K, V, S> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|(key, value)| {
          let key_node = key.tree();
          let mut value_node = value.tree();
          value_node.label = format!("{}: {}", key_node.label, value_node.label);
          value_node
        })
        .collect();

      // Sort by key for deterministic output
      children.sort_by(|a, b| a.label.cmp(&b.label));

      Tree::new(format!("HashMap (len: {})", self.len()), children)
    }
  }

  impl<K: TreeDisplay, V: TreeDisplay> TreeDisplay for std::collections::BTreeMap<K, V> {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .map(|(key, value)| {
          let key_node = key.tree();
          let mut value_node = value.tree();
          value_node.label = format!("{}: {}", key_node.label, value_node.label);
          value_node
        })
        .collect();

      Tree::new(format!("BTreeMap (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay, S> TreeDisplay for std::collections::HashSet<T, S> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();

      children.sort_by(|a, b| a.label.cmp(&b.label));

      Tree::new(format!("HashSet (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BTreeSet<T> {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();

      Tree::new(format!("BTreeSet (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::VecDeque<T> {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      Tree::new(format!("VecDeque (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::LinkedList<T> {
    fn tree(&self) -> Tree {
      let children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      Tree::new(format!("LinkedList (len: {})", self.len()), children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BinaryHeap<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();

      children.sort_by(|a, b| b.label.cmp(&a.label)); // Max heap order

      Tree::new(format!("BinaryHeap (len: {})", self.len()), children)
    }
  }

  impl TreeDisplay for std::path::Path {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  impl TreeDisplay for std::path::PathBuf {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  impl TreeDisplay for std::ffi::OsString {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  impl TreeDisplay for std::ffi::OsStr {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  // ──── Duration ─────────────────────────────────────────────────────────────────────────────────

  impl TreeDisplay for std::time::Duration {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{}s", self.as_secs_f64()))
    }
  }

  impl TreeDisplay for std::time::Instant {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  impl TreeDisplay for std::time::SystemTime {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{:?}", self))
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::Range<T> {
    fn tree(&self) -> Tree {
      Tree::new(
        "Range",
        vec![
          Tree::leaf(format!("start: {:?}", self.start)),
          Tree::leaf(format!("end: {:?}", self.end)),
        ],
      )
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeInclusive<T> {
    fn tree(&self) -> Tree {
      Tree::new(
        "RangeInclusive",
        vec![
          Tree::leaf(format!("start: {:?}", self.start())),
          Tree::leaf(format!("end: {:?}", self.end())),
        ],
      )
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeFrom<T> {
    fn tree(&self) -> Tree {
      Tree::new(
        "RangeFrom",
        vec![Tree::leaf(format!("start: {:?}", self.start))],
      )
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeTo<T> {
    fn tree(&self) -> Tree {
      Tree::new("RangeTo", vec![Tree::leaf(format!("end: {:?}", self.end))])
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeToInclusive<T> {
    fn tree(&self) -> Tree {
      Tree::new(
        "RangeToInclusive",
        vec![Tree::leaf(format!("end: {:?}", self.end))],
      )
    }
  }

  impl TreeDisplay for std::ops::RangeFull {
    fn tree(&self) -> Tree {
      Tree::leaf("RangeFull")
    }
  }

  impl<T> TreeDisplay for std::marker::PhantomData<T> {
    fn tree(&self) -> Tree {
      Tree::leaf("PhantomData")
    }
  }
}

#[cfg(feature = "chumsky")]
mod chumsky_support {
  use super::*;
  use chumsky::span::SimpleSpan;

  impl TreeDisplay for SimpleSpan {
    fn tree(&self) -> Tree {
      Tree::leaf(format!("{}..{}", self.start, self.end))
    }
  }
}
