use tree_display::{
  color::Colors, context::Context, graphics::Graphics, theme::Theme, Formatter, TreeDisplay,
};

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
  #[tree(map)]
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

#[derive(Debug, TreeDisplay)]
enum Status {
  Success,
  Error(u32),
  Pending { message: String },
}

#[test]
fn test_simple_struct() {
  let person = Person {
    name: "Alice".to_string(),
    age: 30,
  };

  let output = Formatter::of(&person)
    .theme(
      Theme::default()
        .colors(Colors::VSCODE_DARK_PLUS)
        .lines(Graphics::LIGHT),
    )
    .format();

  // Strip ANSI color codes for comparison
  let stripped = output
    .lines()
    .map(|line| {
      // Remove ANSI escape sequences
      let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
      re.replace_all(line, "").to_string()
    })
    .collect::<Vec<_>>()
    .join("\n");

  let expected = "Person
├─ name: \"Alice\"
└─ age: 30";

  assert_eq!(stripped, expected);
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

  // The map function returns the length of the formatted string
  let context = Context::new().map(|string: &String| string.len());

  let output = Formatter::of(&person).context(&context).format();

  // Strip ANSI color codes if present
  let stripped = output
    .lines()
    .map(|line| {
      let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
      re.replace_all(line, "").to_string()
    })
    .collect::<Vec<_>>()
    .join("\n");

  let expected = "PersonWithAddress
├─ name: 3
├─ Yes
├─ address: Address
│  ├─ city: \"NYC\"
│  └─ zip: 10001
└─ children: Vec
   ├─ len: 2
   ├─ [0]: Person
   │  ├─ name: \"Charlie\"
   │  └─ age: 5
   └─ [1]: Person
      ├─ name: \"Diana\"
      └─ age: 3";

  assert_eq!(stripped, expected);
}

#[test]
fn test_tuple_struct() {
  #[derive(Debug, TreeDisplay)]
  struct Point(u32, u32);

  let point = Point(10, 20);
  let output = Formatter::of(&point).format();

  let expected = "Point
├─ .0: 10
└─ .1: 20";

  assert_eq!(output, expected);
}

#[test]
fn test_unit_struct() {
  #[derive(Debug, TreeDisplay)]
  struct Empty;

  let empty = Empty;
  let output = Formatter::of(&empty).format();

  assert_eq!(output, "Empty");
}

#[test]
fn test_enum_success() {
  let success = Status::Success;
  let output = Formatter::of(&success).format();

  assert_eq!(output, "Success");
}

#[test]
fn test_enum_error() {
  let error = Status::Error(404);
  let output = Formatter::of(&error).format();

  // Newtype enum forwards directly without wrapping
  assert_eq!(output, "404");
}

#[test]
fn test_enum_pending() {
  let pending = Status::Pending {
    message: "Loading".to_string(),
  };
  let output = Formatter::of(&pending).format();

  let expected = "Pending
└─ message: \"Loading\"";

  assert_eq!(output, expected);
}

#[test]
fn test_newtype() {
  #[derive(Debug, TreeDisplay)]
  struct Wrapped(u32);

  let wrapped = Wrapped(42);
  let output = Formatter::of(&wrapped).format();

  assert_eq!(output, "42");
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

  let output = Formatter::of(&value).format();

  let expected = "Unlabeled
├─ \"Test\"
└─ age: 30";

  assert_eq!(output, expected);
}

#[test]
fn test_option_some() {
  #[derive(Debug, TreeDisplay)]
  struct WithOption {
    #[tree(label = "maybe")]
    value: Option<u32>,
  }

  let some = WithOption { value: Some(42) };
  let output = Formatter::of(&some).format();

  let expected = "WithOption
└─ maybe: 42";

  assert_eq!(output, expected);
}

#[test]
fn test_option_none() {
  #[derive(Debug, TreeDisplay)]
  struct WithOption {
    #[tree(label = "maybe")]
    value: Option<u32>,
  }

  let none = WithOption { value: None };
  let output = Formatter::of(&none).format();

  let expected = "WithOption
└─ maybe: None";

  assert_eq!(output, expected);
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

  let output = Formatter::of(&data).format();

  let expected = "WithVec
└─ numbers: Vec
   ├─ len: 5
   ├─ [0]: 1
   ├─ [1]: 2
   ├─ [2]: 3
   ├─ [3]: 4
   └─ [4]: 5";

  assert_eq!(output, expected);
}

#[test]
fn test_empty_vec() {
  #[derive(Debug, TreeDisplay)]
  struct WithVec {
    values: Vec<u32>,
  }

  let data = WithVec { values: vec![] };
  let output = Formatter::of(&data).format();

  let expected = "WithVec
└─ values: Vec
   └─ len: 0";

  assert_eq!(output, expected);
}

#[test]
fn test_hashmap() {
  use std::collections::HashMap;

  let mut map = HashMap::new();
  map.insert("Alice".to_string(), 30);
  map.insert("Bob".to_string(), 25);

  let output = Formatter::of(&map).format();

  // HashMap iteration order is non-deterministic
  let expected1 = "HashMap
├─ len: 2
├─ [\"Alice\"]: 30
└─ [\"Bob\"]: 25";

  let expected2 = "HashMap
├─ len: 2
├─ [\"Bob\"]: 25
└─ [\"Alice\"]: 30";

  assert!(output == expected1 || output == expected2);
}

#[test]
fn test_btreemap() {
  use std::collections::BTreeMap;

  let mut map = BTreeMap::new();
  map.insert("apple", 1);
  map.insert("banana", 2);
  map.insert("cherry", 3);

  let output = Formatter::of(&map).format();

  let expected = "BTreeMap
├─ len: 3
├─ [\"apple\"]: 1
├─ [\"banana\"]: 2
└─ [\"cherry\"]: 3";

  assert_eq!(output, expected);
}

#[test]
fn test_hashset() {
  use std::collections::HashSet;

  let mut set = HashSet::new();
  set.insert("rust");
  set.insert("go");

  let output = Formatter::of(&set).format();

  // HashSet iteration order is non-deterministic
  let expected1 = "HashSet
├─ len: 2
├─ \"rust\"
└─ \"go\"";

  let expected2 = "HashSet
├─ len: 2
├─ \"go\"
└─ \"rust\"";

  assert!(output == expected1 || output == expected2);
}

#[test]
fn test_btreeset() {
  use std::collections::BTreeSet;

  let mut set = BTreeSet::new();
  set.insert(10);
  set.insert(20);
  set.insert(30);

  let output = Formatter::of(&set).format();

  let expected = "BTreeSet
├─ len: 3
├─ 10
├─ 20
└─ 30";

  assert_eq!(output, expected);
}

#[test]
fn test_vecdeque() {
  use std::collections::VecDeque;

  let mut deque = VecDeque::new();
  deque.push_back(1);
  deque.push_back(2);
  deque.push_back(3);
  deque.push_front(0);

  let output = Formatter::of(&deque).format();

  let expected = "VecDeque
├─ len: 4
├─ [0]: 0
├─ [1]: 1
├─ [2]: 2
└─ [3]: 3";

  assert_eq!(output, expected);
}

#[test]
fn test_linkedlist() {
  use std::collections::LinkedList;

  let mut list = LinkedList::new();
  list.push_back("first");
  list.push_back("second");
  list.push_back("third");

  let output = Formatter::of(&list).format();

  let expected = "LinkedList
├─ len: 3
├─ [0]: \"first\"
├─ [1]: \"second\"
└─ [2]: \"third\"";

  assert_eq!(output, expected);
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

  let output = Formatter::of(&heap).format();

  // BinaryHeap order is not guaranteed, so just check structure
  assert!(output.contains("BinaryHeap"));
  assert!(output.contains("len: 7"));
  assert!(output.contains("50"));
  assert!(output.contains("30"));
  assert!(output.contains("70"));
  assert!(output.contains("20"));
  assert!(output.contains("40"));
  assert!(output.contains("60"));
  assert!(output.contains("10"));
}

#[test]
fn test_tuple() {
  let tuple = (42, "hello", true);
  let output = Formatter::of(&tuple).format();

  let expected = "tuple
├─ .0: 42
├─ .1: \"hello\"
└─ .2: true";

  assert_eq!(output, expected);
}

#[test]
fn test_range() {
  let range = 1..5;
  let output = Formatter::of(&range).format();

  let expected = "Range
├─ start: 1
└─ end: 5";

  assert_eq!(output, expected);
}

#[test]
fn test_range_inclusive() {
  let range_inclusive = 1..=5;
  let output = Formatter::of(&range_inclusive).format();

  let expected = "RangeInclusive
├─ start: 1
└─ end: 5";

  assert_eq!(output, expected);
}

#[test]
fn test_range_from() {
  let range_from = 1..;
  let output = Formatter::of(&range_from).format();

  let expected = "RangeFrom
└─ start: 1";

  assert_eq!(output, expected);
}

#[test]
fn test_range_to() {
  let range_to = ..5;
  let output = Formatter::of(&range_to).format();

  let expected = "RangeTo
└─ end: 5";

  assert_eq!(output, expected);
}

#[test]
fn test_range_full() {
  let range_full = ..;
  let output = Formatter::of(&range_full).format();

  assert_eq!(output, "RangeFull");
}

#[test]
fn test_duration() {
  use std::time::Duration;

  let duration = Duration::from_secs(123);
  let output = Formatter::of(&duration).format();

  // Duration is displayed as a string with quotes because it's formatted via Debug
  assert_eq!(output, "\"123s\"");
}

#[test]
fn test_result_ok() {
  let result: Result<u32, &str> = Ok(42);
  let output = Formatter::of(&result).format();

  assert_eq!(output, "42");
}

#[test]
fn test_result_err() {
  let result: Result<u32, &str> = Err("error");
  let output = Formatter::of(&result).format();

  assert_eq!(output, "\"error\"");
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

  let output = Formatter::of(&data).format();

  let expected = "WithPhantom
├─ value: 42
└─ _marker: PhantomData";

  assert_eq!(output, expected);
}

#[test]
fn test_custom_label() {
  #[derive(Debug, TreeDisplay)]
  struct CustomLabel {
    #[tree(label = "full_name")]
    name: String,
    age: u32,
  }

  let data = CustomLabel {
    name: "Alice".to_string(),
    age: 30,
  };

  let output = Formatter::of(&data).format();

  let expected = "CustomLabel
├─ full_name: \"Alice\"
└─ age: 30";

  assert_eq!(output, expected);
}

#[test]
fn test_ignore_field() {
  #[derive(Debug, TreeDisplay)]
  struct IgnoredField {
    visible: u32,
    #[tree(ignore)]
    hidden: u32,
  }

  let data = IgnoredField {
    visible: 42,
    hidden: 100,
  };

  let output = Formatter::of(&data).format();

  let expected = "IgnoredField
└─ visible: 42";

  assert_eq!(output, expected);
}

#[test]
fn test_mixed_collections_in_tuple() {
  use std::collections::{HashMap, HashSet, VecDeque};

  let mut vec = VecDeque::new();
  vec.push_back(1);
  vec.push_back(2);

  let mut set = HashSet::new();
  set.insert("hello".to_string());

  let mut map = HashMap::new();
  map.insert("a", 10);

  let mixed = (vec, set, map);
  let output = Formatter::of(&mixed).format();

  // Since HashSet iteration order is non-deterministic but there's only one element,
  // the output is deterministic for HashSet. HashMap has only one entry too.
  let expected = "tuple
├─ .0: VecDeque
│  ├─ len: 2
│  ├─ [0]: 1
│  └─ [1]: 2
├─ .1: HashSet
│  ├─ len: 1
│  └─ \"hello\"
└─ .2: HashMap
   ├─ len: 1
   └─ [\"a\"]: 10";

  assert_eq!(output, expected);
}
