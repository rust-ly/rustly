+++
id = "book.modules.pub-structs-enums"
chapter = "book.modules"
requires = []
title = "Public Structs and Enums"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public"
summary = "pub on a struct leaves its fields private; pub on an enum makes every variant public."
+++

# Making Structs and Enums Public

`pub` works a little differently on structs and enums, and the difference is deliberate.

## Struct fields stay private

Marking a struct `pub` makes the **type** public, but each **field** is still private unless it's marked `pub` too. That lets a module show some fields and hide others.

A restaurant's breakfast comes with toast you choose, and seasonal fruit the chef chooses:

```rust,runnable title="One public field, one private field"
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
```

Because `seasonal_fruit` is private, code outside `back_of_house` can't **set** it, so it also can't build a `Breakfast` with `Breakfast { ... }` directly. It has to go through a public constructor like `summer`. This is how Rust types protect their rules: the module controls how a value is created and changed.

```rust,does_not_compile title="Touching a private field"
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }
}

fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.seasonal_fruit = String::from("blueberries");
}
```

## Enum variants are all public

An enum is the opposite. Make it `pub`, and **all** its variants are public. An enum with private variants wouldn't be much use, because code outside could never match on it:

```rust,editable title="A public enum"
mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{order1:?} and {order2:?}");
}
```

A good rule of thumb: struct fields default to private because they're implementation details, while enum variants **are** the enum's interface.
