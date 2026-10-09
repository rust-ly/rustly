+++
id = "book.enums.match"
chapter = "book.enums"
requires = []
title = "The match Expression"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch06-02-match.html"
summary = "Compare a value against a list of patterns and run the code for the one that fits."
+++

# The `match` Expression

`match` compares a value against a list of **patterns** and runs the code for the first pattern that fits. Picture a coin-sorting machine: a coin rolls down a track with holes of different sizes and drops through the first hole it fits.

```rust,runnable title="Sorting coins"
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("A dime is worth {} cents", value_in_cents(Coin::Dime));
}
```

## Reading a match

- `match coin` names the value being checked.
- Each line inside is an **arm**: a pattern, `=>`, then the code to run.
- Arms are tried **top to bottom**; the first one that matches wins.
- The value of the winning arm becomes the value of the whole `match`, so here the function returns `1`, `5`, `10` or `25`.

This is a big difference from `if`. An `if` condition must be a `bool`, but `match` works on any type, including your own enums.

## Arms with more than one line

If an arm needs several statements, wrap them in a block. The last expression in the block is the arm's value:

```rust,editable title="A block in an arm"
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let total = value_in_cents(Coin::Penny) + value_in_cents(Coin::Quarter);
    println!("total: {total}");
}
```

## Every arm must give the same type

Because the `match` produces one value, all arms must produce the same type. Mixing a number and some text won't compile:

```rust,does_not_compile title="Arms that disagree on the type"
enum Coin {
    Penny,
    Dime,
}

fn main() {
    let coin = Coin::Penny;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Dime => "ten",
    };
}
```

The error, `E0308: match arms have incompatible types`, points at the arm that doesn't fit.
