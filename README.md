# `tree-display`

[![Crates.io](https://img.shields.io/crates/v/tree-display.svg)](https://crates.io/crates/tree-display)
[![Docs.rs](https://docs.rs/tree-display/badge.svg)](https://docs.rs/tree-display)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Display Rust data structures as syntax-highlighted trees in the terminal with a single derive macro.
Features full color theming, context-aware value mapping, and intelligent formatting for collections.
Perfect for debugging, compiler diagnostics, and AST inspection.

## Features

- **Simple derive macro** - `#[derive(TreeDisplay)]` for automatic tree generation
- **Field attributes** - Control rendering of individual fields:
  - `#[tree(map)]` - Apply a custom mapper from `Context`
  - `#[tree(ignore)]` - Exclude a field from the tree
  - `#[tree(label = "...")]` - Override the field's display label
  - `#[tree(unlabeled)]` - Display the field without a label
- **Custom mapping** - Transform values with `Context` mappers
- **Theming** - Predefined themes (VS Code Dark+, Solarized, etc.)
- **Color support** - Optional ANSI color highlighting
- **Line styles** - ASCII or Unicode box-drawing characters
- **Formatting** - Labels, alignment, and custom content types
- **Comprehensive support** - Works with most standard library types

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
tree-display = "1.0"
```

Then derive `TreeDisplay` for your types:

```rust
use tree_display::{TreeDisplay, Formatter};

#[derive(Debug, TreeDisplay)]
struct Person {
    name: String,
    age: u32,
    #[tree(label = "children")]
    children: Vec<Person>,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
    children: vec![
        Person { name: "Bob".to_string(), age: 5, children: vec![] },
        Person { name: "Charlie".to_string(), age: 3, children: vec![] },
    ],
};

println!("{}", Formatter::of(&person).format());
```

Output:

```
Person
├─ name: "Alice"
├─ age: 30
└─ children: Vec
   ├─ len: 2
   ├─ [0]: Person
   │  ├─ name: "Bob"
   │  └─ age: 5
   └─ [1]: Person
      ├─ name: "Charlie"
      └─ age: 3
```

## Field Attributes

### `#[tree(map)]`

Apply a custom mapper from the context. Requires a `Context` with a registered mapper for the field's type.

```rust
use tree_display::{TreeDisplay, Formatter, Context};

#[derive(TreeDisplay)]
struct Person {
    #[tree(map)]
    name: String,
}

let context = Context::new().map(|s: &String| s.len());

let person = Person { name: "Alice".to_string() };
println!("{}", Formatter::of(&person).context(&context).format());
```

Output:

```
Person
└─ name: 5
```

### `#[tree(ignore)]`

Exclude a field from the tree entirely.

```rust
#[derive(TreeDisplay)]
struct Person {
    name: String,
    #[tree(ignore)]
    id: u64,
}

let person = Person { name: "Bob".to_string(), id: 0 };
println!("{}", Formatter::of(&person).context(&context).format());
```

Output:

```
Person
└─ name: "Bob"
```

### `#[tree(label = "...")]`

Override the field's display label.

```rust
#[derive(TreeDisplay)]
struct Person {
    #[tree(label = "full_name")]
    name: String,
}
```

Output:

```
Person
└─ full_name: "Alice"
```

### `#[tree(unlabeled)]`

Display the field without a label.

```rust
#[derive(TreeDisplay)]
struct Person {
    #[tree(unlabeled)]
    name: String,
}
```

Output:

```
Person
└─ "Alice"
```

## Theming

`tree-display` comes with several predefined themes:

```rust
use tree_display::{Formatter, Theme, Graphics, Colors};

let theme = Theme::default()
    .colors(Colors::VSCODE_DARK_PLUS)
    .lines(Graphics::LIGHT_ROUNDED)
    .align_to_values(true);

let output = Formatter::of(&person).theme(theme).format();
```

Available themes:

- `Colors::VSCODE_DARK_PLUS`
- `Colors::VSCODE_LIGHT_PLUS`
- `Colors::SOLARIZED_DARK`
- `Colors::SOLARIZED_LIGHT`
- `Colors::DRACULA`
- `Colors::MONOKAI`
- `Colors::NORD`
- `Colors::GITHUB_DARK`
- `Colors::GITHUB_LIGHT`
- `Colors::NONE` (no colors)

Available line styles:

- `Graphics::ASCII` - Pure ASCII (terminal compatible)
- `Graphics::LIGHT` - Unicode box-drawing
- `Graphics::LIGHT_ROUNDED` - Rounded corners
- `Graphics::LIGHT_DOTTED` - Dotted lines
- `Graphics::DOUBLE` - Double lines
- `Graphics::HEAVY` - Heavy lines
- `Graphics::BLANK` - No lines

## Custom Mappers

Use `Context` to transform values before display:

```rust
use tree_display::{Context, Formatter, TreeDisplay};

#[derive(TreeDisplay)]
struct Data {
    value: i32,
}

let context = Context::new()
    .map(|n: &i32| format!("0x{:X}", n))
    .map(|s: &String| s.len());

let data = Data { value: 42 };
println!("{}", Formatter::of(&data).context(&context).format());
```

## Supported Types

`tree-display` automatically implements `TreeDisplay` for many standard library types:

- Primitives (bool, char, integers, floats)
- Strings and `&str`
- Tuples (up to 12 elements)
- Arrays, slices, `Vec`, `VecDeque`, `LinkedList`
- `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `BinaryHeap`
- `Option`, `Result`
- Smart pointers (`Box`, `Rc`, `Arc`, `RefCell`, `Mutex`, `RwLock`)
- Ranges (`Range`, `RangeInclusive`, `RangeFrom`, `RangeTo`, `RangeFull`)
- `Duration`, `Instant`, `SystemTime`
- `Path`, `PathBuf`, `OsString`, `OsStr`
- `PhantomData`

## Feature Flags

- `color` - Enables ANSI color support (enabled by default)
- `chumsky` - Adds support for `chumsky::span::SimpleSpan`

To disable default features:

```toml
[dependencies.tree-display]
version = "1.0"
default-features = false
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
