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
            node.label = Some(format!(".{}", $idx));
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
          node.content = format!("Ok({})", node.content);
          node.label = Some("Ok".to_string());
          node
        }
        Err(err) => {
          let mut node = err.tree();
          node.content = format!("Err({})", node.content);
          node.label = Some("Err".to_string());
          node
        }
      }
    }
  }

  impl<T: TreeDisplay, const N: usize> TreeDisplay for [T; N] {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", i));
          node
        })
        .collect();

      let len_node = Tree::leaf(N).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("Array", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for [T] {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", i));
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("Slice", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for Vec<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .enumerate()
        .map(|(i, item)| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", i));
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("Vec", children)
    }
  }

  impl<K: TreeDisplay, V: TreeDisplay, S> TreeDisplay for std::collections::HashMap<K, V, S> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|(key, value)| {
          let mut node = value.tree();
          let key_content = format!("[{}]: {}", key.tree().content, node.content);
          node.content = key_content;
          node
        })
        .collect();

      children.sort_by(|a, b| a.content.cmp(&b.content));
      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("HashMap", children)
    }
  }

  impl<K: TreeDisplay, V: TreeDisplay> TreeDisplay for std::collections::BTreeMap<K, V> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|(key, value)| {
          let mut node = value.tree();
          let key_content = format!("{}: {}", key.tree().content, node.content);
          node.content = key_content;
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("BTreeMap", children)
    }
  }

  impl<T: TreeDisplay, S> TreeDisplay for std::collections::HashSet<T, S> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|item| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", node.content));
          node
        })
        .collect();

      children.sort_by(|a, b| a.content.cmp(&b.content));
      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("HashSet", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BTreeSet<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|item| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", node.content));
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
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
          node.label = Some(format!("[{}]", i));
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
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
          node.label = Some(format!("[{}]", i));
          node
        })
        .collect();

      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
      Tree::new("LinkedList", children)
    }
  }

  impl<T: TreeDisplay> TreeDisplay for std::collections::BinaryHeap<T> {
    fn tree(&self) -> Tree {
      let mut children: Vec<Tree> = self
        .iter()
        .map(|item| {
          let mut node = item.tree();
          node.label = Some(format!("[{}]", node.content));
          node
        })
        .collect();

      children.sort_by(|a, b| b.content.cmp(&a.content)); // Max heap order
      let len_node = Tree::leaf(self.len()).labeled(Some("len".into()));
      children.insert(0, len_node);
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
      let children = vec![
        Tree::leaf(format!("start: {:?}", self.start)),
        Tree::leaf(format!("end: {:?}", self.end)),
      ];
      let mut tree = Tree::new("Range", children);
      tree.label = Some("Range".to_string());
      tree
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeInclusive<T> {
    fn tree(&self) -> Tree {
      let children = vec![
        Tree::leaf(format!("start: {:?}", self.start())),
        Tree::leaf(format!("end: {:?}", self.end())),
      ];
      let mut tree = Tree::new("RangeInclusive", children);
      tree.label = Some("RangeInclusive".to_string());
      tree
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeFrom<T> {
    fn tree(&self) -> Tree {
      let children = vec![Tree::leaf(format!("start: {:?}", self.start))];
      let mut tree = Tree::new("RangeFrom", children);
      tree.label = Some("RangeFrom".to_string());
      tree
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeTo<T> {
    fn tree(&self) -> Tree {
      let children = vec![Tree::leaf(format!("end: {:?}", self.end))];
      let mut tree = Tree::new("RangeTo", children);
      tree.label = Some("RangeTo".to_string());
      tree
    }
  }

  impl<T: TreeDisplay + std::fmt::Debug> TreeDisplay for std::ops::RangeToInclusive<T> {
    fn tree(&self) -> Tree {
      let children = vec![Tree::leaf(format!("end: {:?}", self.end))];
      let mut tree = Tree::new("RangeToInclusive", children);
      tree.label = Some("RangeToInclusive".to_string());
      tree
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
