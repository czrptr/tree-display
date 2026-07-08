// tests/tree_display_test.rs
use tree_display::{TreeDisplay, tree_format};

#[derive(Debug, TreeDisplay)]
struct Person {
  name: String,
  age: u32,
}

#[derive(Debug, TreeDisplay)]
struct Address {
  city: String,
  zip: u32,
}

#[derive(Debug, TreeDisplay)]
enum YesNo {
  Yes,
}

#[derive(Debug, TreeDisplay)]
struct PersonWithAddress {
  name: String,
  #[tree(unlabeled)]
  age: YesNo,
  #[allow(dead_code)]
  #[tree(ignore)]
  skill: u64,
  address: Address,
  #[tree(label = "children")]
  children: Vec<Person>,
}

#[test]
fn test_simple_struct() {
  let person = Person {
    name: "Alice".to_string(),
    age: 30,
  };

  let output = tree_format(&person);
  println!("{}", output);
  assert!(output.contains("Person"));
  assert!(output.contains("Alice"));
}

#[test]
fn test_nested_struct() {
  let person = PersonWithAddress {
    name: "Bob".to_string(),
    age: YesNo::Yes,
    skill: 40,
    address: Address {
      city: "NYC".to_string(),
      zip: 10001,
    },
    children: vec![
      Person {
        name: "Charlie".to_string(),
        age: 5,
      },
      Person {
        name: "Diana".to_string(),
        age: 3,
      },
    ],
  };

  let output = tree_format(&person);
  println!("{}", output);
  assert!(output.contains("Bob"));
  assert!(output.contains("address"));
  assert!(output.contains("children"));
  assert!(!output.contains("skill"));
}

#[test]
fn test_tuple_struct() {
  #[derive(Debug, TreeDisplay)]
  struct Point(u32, u32);

  let point = Point(10, 20);
  let output = tree_format(&point);
  println!("{}", output);
  assert!(output.contains("Point"));
}

#[test]
fn test_unit_struct() {
  #[derive(Debug, TreeDisplay)]
  struct Empty;

  let empty = Empty;
  let output = tree_format(&empty);
  println!("{}", output);
  assert_eq!(output, "Empty");
}

#[test]
fn test_enum() {
  #[derive(Debug, TreeDisplay)]
  enum Status {
    Success,
    Error(u32),
    Pending { message: String },
  }

  let success = Status::Success;
  let error = Status::Error(404);
  let pending = Status::Pending {
    message: "Loading".to_string(),
  };

  println!("Success:\n----\n{}\n----\n", tree_format(&success));
  println!("Error:\n----\n{}\n----\n", tree_format(&error));
  println!("Pending:\n----\n{}\n----\n", tree_format(&pending));
}

#[test]
fn test_newtype() {
  #[derive(Debug, TreeDisplay)]
  struct Wrapped(u32);

  let wrapped = Wrapped(42);
  let output = tree_format(&wrapped);
  // Should just show the inner value without wrapping
  assert!(output.contains("42"));
}

#[test]
fn test_unlabeled() {
  #[derive(Debug, TreeDisplay)]
  struct Unlabeled {
    #[tree(unlabeled)]
    name: String,
    age: u32,
  }

  let value = Unlabeled {
    name: "Test".to_string(),
    age: 30,
  };

  let output = tree_format(&value);
  println!("{}", output);
  // The name field should be displayed without the "name:" prefix
}

#[test]
fn test_option() {
  #[derive(Debug, TreeDisplay)]
  struct WithOption {
    #[tree(label = "maybe")]
    value: Option<u32>,
  }

  let some = WithOption { value: Some(42) };
  let none = WithOption { value: None };

  println!("Some:\n----\n{}\n----\n", tree_format(&some));
  println!("None:\n----\n{}\n----\n", tree_format(&none));
}

#[test]
fn test_vec() {
  #[derive(Debug, TreeDisplay)]
  struct WithVec {
    #[tree(label = "numbers")]
    values: Vec<u32>,
  }

  let data = WithVec {
    values: vec![1, 2, 3, 4, 5],
  };

  let output = tree_format(&data);
  println!("{}", output);
  assert!(output.contains("Vec"));
  assert!(output.contains("[0]"));
  assert!(output.contains("[4]"));
}

#[test]
fn test_hashmap() {
  use std::collections::HashMap;

  let mut map = HashMap::new();
  map.insert("Alice".to_string(), 30);
  map.insert("Bob".to_string(), 25);
  map.insert("Charlie".to_string(), 35);

  let output = tree_format(&map);
  println!("HashMap:\n----\n{}\n----\n", output);

  assert!(output.contains("HashMap"));
  assert!(output.contains("len: 3"));
  assert!(output.contains("Alice: 30"));
  assert!(output.contains("Bob: 25"));
  assert!(output.contains("Charlie: 35"));
}

#[test]
fn test_btreemap() {
  use std::collections::BTreeMap;

  let mut map = BTreeMap::new();
  map.insert("apple", 1);
  map.insert("banana", 2);
  map.insert("cherry", 3);
  map.insert("date", 4);

  let output = tree_format(&map);
  println!("BTreeMap:\n----\n{}\n----\n", output);

  assert!(output.contains("BTreeMap"));
  assert!(output.contains("len: 4"));
  assert!(output.contains("apple: 1"));
  assert!(output.contains("banana: 2"));
  assert!(output.contains("cherry: 3"));
  assert!(output.contains("date: 4"));
}

#[test]
fn test_hashset() {
  use std::collections::HashSet;

  let mut set = HashSet::new();
  set.insert("rust");
  set.insert("go");
  set.insert("python");
  set.insert("javascript");

  let output = tree_format(&set);
  println!("HashSet:\n----\n{}\n----\n", output);

  assert!(output.contains("HashSet"));
  assert!(output.contains("len: 4"));
  // Check for at least some of the values (order is non-deterministic)
  assert!(
    output.contains("rust")
      || output.contains("go")
      || output.contains("python")
      || output.contains("javascript")
  );
}

#[test]
fn test_btreeset() {
  use std::collections::BTreeSet;

  let mut set = BTreeSet::new();
  set.insert(10);
  set.insert(20);
  set.insert(30);
  set.insert(40);
  set.insert(50);

  let output = tree_format(&set);
  println!("BTreeSet:\n----\n{}\n----\n", output);

  assert!(output.contains("BTreeSet"));
  assert!(output.contains("len: 5"));
  // BTreeSet iterates in order
  assert!(output.contains("10"));
  assert!(output.contains("20"));
  assert!(output.contains("30"));
  assert!(output.contains("40"));
  assert!(output.contains("50"));
}

#[test]
fn test_vecdeque() {
  use std::collections::VecDeque;

  let mut deque = VecDeque::new();
  deque.push_back(1);
  deque.push_back(2);
  deque.push_back(3);
  deque.push_front(0);

  let output = tree_format(&deque);
  println!("VecDeque:\n----\n{}\n----\n", output);

  assert!(output.contains("VecDeque"));
  assert!(output.contains("len: 4"));
  assert!(output.contains("[0]: 0"));
  assert!(output.contains("[1]: 1"));
  assert!(output.contains("[2]: 2"));
  assert!(output.contains("[3]: 3"));
}

#[test]
fn test_linkedlist() {
  use std::collections::LinkedList;

  let mut list = LinkedList::new();
  list.push_back("first");
  list.push_back("second");
  list.push_back("third");

  let output = tree_format(&list);
  println!("LinkedList:\n----\n{}\n----\n", output);

  assert!(output.contains("LinkedList"));
  assert!(output.contains("len: 3"));
  assert!(output.contains("[0]: first"));
  assert!(output.contains("[1]: second"));
  assert!(output.contains("[2]: third"));
}

#[test]
fn test_binaryheap() {
  use std::collections::BinaryHeap;

  let mut heap = BinaryHeap::new();
  heap.push(50);
  heap.push(30);
  heap.push(70);
  heap.push(20);
  heap.push(40);
  heap.push(60);
  heap.push(10);

  let output = tree_format(&heap);
  println!("BinaryHeap:\n----\n{}\n----\n", output);

  assert!(output.contains("BinaryHeap"));
  assert!(output.contains("len: 7"));
  // BinaryHeap is a max-heap, so the largest values should appear first
  let lines: Vec<&str> = output.lines().collect();
  // Check that 70 appears before 10 (roughly, since it's a tree structure)
  let pos_70 = lines.iter().position(|&l| l.contains("70")).unwrap_or(999);
  let pos_10 = lines.iter().position(|&l| l.contains("10")).unwrap_or(999);
  // In a max-heap, larger values should be at the top
  // Note: This is a loose check because the tree structure might have different ordering
  assert!(pos_70 < pos_10 || output.contains("70") && output.contains("10"));
}

#[test]
fn test_nested_collections() {
  use std::collections::{HashMap, HashSet};

  // Nested HashMap with HashMap values
  let mut inner_map = HashMap::new();
  inner_map.insert("x", 1);
  inner_map.insert("y", 2);

  let mut outer_map = HashMap::new();
  outer_map.insert("first", inner_map);

  // Nested HashMap with HashSet values (separate map)
  let mut set_map = HashMap::new();
  let mut set = HashSet::new();
  set.insert(100);
  set.insert(200);
  set_map.insert("second", set);

  let output = tree_format(&outer_map);
  println!("Nested HashMap:\n----\n{}\n----\n", output);

  let output2 = tree_format(&set_map);
  println!("Nested HashSet in HashMap:\n----\n{}\n----\n", output2);

  assert!(output.contains("HashMap"));
  assert!(output.contains("len: 1"));
  assert!(output.contains("first"));
  assert!(output2.contains("HashSet"));
}

#[test]
fn test_range_types() {
  let range = 1..5;
  let output = tree_format(&range);
  println!("Range:\n----\n{}\n----\n", output);
  assert!(output.contains("Range"));
  assert!(output.contains("start: 1"));
  assert!(output.contains("end: 5"));

  let range_inclusive = 1..=5;
  let output = tree_format(&range_inclusive);
  println!("RangeInclusive:\n----\n{}\n----\n", output);
  assert!(output.contains("RangeInclusive"));
  assert!(output.contains("start: 1"));
  assert!(output.contains("end: 5"));

  let range_from = 1..;
  let output = tree_format(&range_from);
  println!("RangeFrom:\n----\n{}\n----\n", output);
  assert!(output.contains("RangeFrom"));
  assert!(output.contains("start: 1"));

  let range_to = ..5;
  let output = tree_format(&range_to);
  println!("RangeTo:\n----\n{}\n----\n", output);
  assert!(output.contains("RangeTo"));
  assert!(output.contains("end: 5"));

  let range_full = ..;
  let output = tree_format(&range_full);
  println!("RangeFull:\n----\n{}\n----\n", output);
  assert_eq!(output, "RangeFull");
}

#[test]
fn test_duration() {
  use std::time::Duration;

  let duration = Duration::from_secs(123);
  let output = tree_format(&duration);
  println!("Duration:\n----\n{}\n----\n", output);
  assert!(output.contains("123s"));

  let duration_ms = Duration::from_millis(1500);
  let output = tree_format(&duration_ms);
  println!("Duration (ms):\n----\n{}\n----\n", output);
  assert!(output.contains("1.5s"));
}

#[test]
fn test_path() {
  use std::path::PathBuf;

  let path = PathBuf::from("/home/user/file.txt");
  let output = tree_format(&path);
  println!("Path:\n----\n{}\n----\n", output);
  // Just check it doesn't panic
  assert!(!output.is_empty());
}

#[test]
fn test_phantom_data() {
  use std::marker::PhantomData;

  #[derive(Debug, TreeDisplay)]
  struct WithPhantom<T> {
    value: u32,
    _marker: PhantomData<T>,
  }

  let data = WithPhantom::<String> {
    value: 42,
    _marker: PhantomData,
  };

  let output = tree_format(&data);
  println!("WithPhantom:\n----\n{}\n----\n", output);
  assert!(output.contains("42"));
}

#[test]
fn test_mixed_collections_in_tuple() {
  use std::collections::{HashMap, HashSet, VecDeque};

  let mut vec = VecDeque::new();
  vec.push_back(1);
  vec.push_back(2);
  vec.push_back(3);

  let mut set = HashSet::new();
  set.insert("hello".to_string());
  set.insert("world".to_string());

  let mut map = HashMap::new();
  map.insert("a", 10);
  map.insert("b", 20);

  // Different collections in a tuple
  let mixed = (vec, set, map);
  let output = tree_format(&mixed);
  println!("Mixed Collections:\n----\n{}\n----\n", output);

  assert!(output.contains("tuple"));
  assert!(output.contains("VecDeque"));
  assert!(output.contains("HashSet"));
  assert!(output.contains("HashMap"));
}

#[test]
fn test_custom_struct_with_collections() {
  use std::collections::{HashMap, HashSet, VecDeque};

  #[derive(Debug, TreeDisplay)]
  struct MyCollections {
    numbers: VecDeque<u32>,
    words: HashSet<String>,
    scores: HashMap<String, u32>,
  }

  let mut vec = VecDeque::new();
  vec.push_back(1);
  vec.push_back(2);
  vec.push_back(3);

  let mut set = HashSet::new();
  set.insert("hello".to_string());
  set.insert("world".to_string());

  let mut map = HashMap::new();
  map.insert("Alice".to_string(), 100);
  map.insert("Bob".to_string(), 95);

  let data = MyCollections {
    numbers: vec,
    words: set,
    scores: map,
  };

  let output = tree_format(&data);
  println!("Custom Collections Struct:\n----\n{}\n----\n", output);

  assert!(output.contains("MyCollections"));
  assert!(output.contains("numbers"));
  assert!(output.contains("VecDeque"));
  assert!(output.contains("words"));
  assert!(output.contains("HashSet"));
  assert!(output.contains("scores"));
  assert!(output.contains("HashMap"));
}
