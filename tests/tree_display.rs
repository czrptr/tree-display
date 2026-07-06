// tests/tree_display_test.rs
use tree_display::{TreeDisplay, tree_format};

#[derive(TreeDisplay)]
struct Person {
  name: String,
  age: u32,
}

#[derive(TreeDisplay)]
struct Address {
  city: String,
  zip: u32,
}

#[derive(TreeDisplay)]
struct PersonWithAddress {
  name: String,
  #[tree(unlabeled)]
  age: u32,
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
    age: 25,
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
  #[derive(TreeDisplay)]
  struct Point(u32, u32);

  let point = Point(10, 20);
  let output = tree_format(&point);
  println!("{}", output);
  assert!(output.contains("Point"));
}

#[test]
fn test_unit_struct() {
  #[derive(TreeDisplay)]
  struct Empty;

  let empty = Empty;
  let output = tree_format(&empty);
  println!("{}", output);
  assert_eq!(output, "Empty");
}

#[test]
fn test_enum() {
  #[derive(TreeDisplay)]
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
  #[derive(TreeDisplay)]
  struct Wrapped(u32);

  let wrapped = Wrapped(42);
  let output = tree_format(&wrapped);
  // Should just show the inner value without wrapping
  assert!(output.contains("42"));
}

#[test]
fn test_unlabeled() {
  #[derive(TreeDisplay)]
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
  #[derive(TreeDisplay)]
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
  #[derive(TreeDisplay)]
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
