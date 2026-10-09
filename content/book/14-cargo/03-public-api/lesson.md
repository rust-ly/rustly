+++
id = "book.cargo.public-api"
chapter = "book.cargo"
requires = []
title = "A Convenient Public API with pub use"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use"
summary = "Organise a crate however suits you internally, and re-export the important items at the top for users."
+++

# Exporting a Convenient Public API with `pub use`

The module structure that's convenient for **you** while writing a crate isn't always convenient for the people **using** it. Deep nesting means users have to type, and first discover, long paths.

The Book's example is an `art` crate with colours and a mixing function, organised into modules:

```rust
//! # Art
//!
//! A library for modelling artistic concepts.

pub mod kinds {
    /// The primary colours according to the RYB colour model.
    pub enum PrimaryColor { Red, Yellow, Blue }

    /// The secondary colours according to the RYB colour model.
    pub enum SecondaryColor { Orange, Green, Purple }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colours in equal amounts to create a secondary colour.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

A user has to know that `PrimaryColor` is in `kinds` and `mix` is in `utils`:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;
```

## Re-export at the top

Add `pub use` lines at the crate root, and the important items are available directly as `art::PrimaryColor` and `art::mix`. The internal layout doesn't change, and old paths still work:

```rust,editable title="Re-exported items"
mod art {
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        #[derive(Debug)]
        pub enum PrimaryColor { Red, Yellow, Blue }
        #[derive(Debug)]
        pub enum SecondaryColor { Orange, Green, Purple }
    }

    pub mod utils {
        use super::kinds::*;

        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            match (c1, c2) {
                (PrimaryColor::Red, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
                (PrimaryColor::Yellow, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
                _ => SecondaryColor::Purple,
            }
        }
    }
}

use art::{mix, PrimaryColor};

fn main() {
    println!("{:?}", mix(PrimaryColor::Red, PrimaryColor::Yellow));
}
```

The generated documentation lists re-exports on the crate's front page under **Re-exports**, so users find them immediately.

This is the same `pub use` you learned in chapter 7. The difference here is the purpose: designing what other people see when they add your crate as a dependency. Popular crates do this constantly. For example, `tokio::spawn` is really defined deep inside `tokio::task`.
