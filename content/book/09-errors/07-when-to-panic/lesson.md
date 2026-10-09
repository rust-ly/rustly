+++
id = "book.errors.when-to-panic"
chapter = "book.errors"
requires = []
title = "To panic! or Not to panic!"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html"
summary = "When to return a Result and when to panic, and how a type can guarantee its value is always valid."
+++

# To `panic!` or Not to `panic!`

You now have two tools: return a `Result`, or panic. How do you choose?

**Return a `Result` by default.** It leaves the decision to the caller, who knows more about the situation. They can retry, fall back to a default, show a message, or panic themselves if they really want to. Panicking takes that choice away.

**Panic when continuing would be wrong.** That means:

- A **bug**: the code has reached a state that should be impossible, like an index you computed being out of range.
- A **broken contract**: the caller passed a value your function clearly documents as invalid, and there's no sensible way to continue.
- **Examples, prototypes and tests**, where `unwrap` and `expect` keep the code short.

A useful test: is this failure something a correct program could run into? A missing file, bad user input or a network timeout: yes, so return a `Result`. Your own logic contradicting itself: no, so panic.

## Making invalid values impossible

Checking a value in every function that uses it is tedious and easy to forget. A better approach is a **type** that can only ever hold valid values. The check happens once, when the value is created, and every function that receives one can trust it.

In the Book's guessing game, guesses must be between 1 and 100:

```rust,runnable title="A Guess is always 1-100"
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let g = Guess::new(42);
    println!("guessed {}", g.value());
}
```

Because the `value` field is private (as in chapter 7), the **only** way to make a `Guess` is through `new`, so a `Guess` holding 500 can't exist. A function that takes a `Guess` never needs to check the range again.

## The same idea, recoverable

If an out-of-range value is something users might reasonably type, return a `Result` from the constructor instead of panicking, and let the caller ask again:

```rust,editable title="A constructor that returns Result"
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("{value} is out of range, pick 1-100"));
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    for n in [50, 0, 101] {
        match Guess::new(n) {
            Ok(g) => println!("ok: {}", g.value()),
            Err(e) => println!("error: {e}"),
        }
    }
}
```

Both versions guarantee that every `Guess` is valid. They differ only in what happens to bad input.
