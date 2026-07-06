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

  macro_rules! impl_tuple_tree_display {
    ($($ty:ident $idx:tt),*) => {
      impl<$($ty: TreeDisplay),*> TreeDisplay for ($($ty,)*) {
        fn tree(&self) -> Tree {
          #[allow(unused_mut)] // erroneous warning
          let mut tree = Tree::leaf("tuple");
          $(
            let mut node = self.$idx.tree();
            node.label = format!(".{}: {}", $idx, node.label);
            tree.subtrees.push(node);
          )*
          tree
        }
      }
    };
  }

  // Manual expansion for each tuple size
  impl_tuple_tree_display!();
  impl_tuple_tree_display!(T0 0);
  impl_tuple_tree_display!(T0 0, T1 1);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10);
  impl_tuple_tree_display!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11);

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
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("Vec", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for [T] {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("Slice", children)
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
      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("HashMap", children)
    }
  }

  impl<K: TreeDisplay, V: TreeDisplay> TreeDisplay for std::collections::BTreeMap<K, V> {
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

      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("BTreeMap", children)
    }
  }

  impl<T: TreeDisplay, S> TreeDisplay for std::collections::HashSet<T, S> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();

      children.sort_by(|a, b| a.label.cmp(&b.label));
      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("HashSet", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BTreeSet<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();
      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("BTreeSet", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::VecDeque<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("VecDeque", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::LinkedList<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = format!("[{}]: {}", i, node.label);
          node
        })
        .collect();

      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("LinkedList", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BinaryHeap<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self.iter().map(|item| item.tree()).collect();

      children.sort_by(|a, b| b.label.cmp(&a.label)); // Max heap order
      children.insert(0, Tree::leaf(format!("len: {}", self.len())));

      Tree::new("BinaryHeap", children)
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
