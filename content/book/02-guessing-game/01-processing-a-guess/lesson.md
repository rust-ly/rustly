+++
id = "book.guessing-game.processing-a-guess"
chapter = "book.guessing-game"
requires = []
title = "Turning Text into a Number"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#processing-a-guess"
summary = "A player's guess arrives as text. Trim it, parse it into a number, and handle input that isn't a number."
+++

# Turning Text into a Number

In the Book's guessing game, the program picks a secret number from 1 to 100, and the player keeps guessing until they get it. On your own machine the guesses come from the keyboard, read with `io::stdin().read_line(&mut guess)`.

`read_line` adds whatever the player typed to `guess`, **including the Enter key**, so typing `42` gives you the text `"42\n"`. Rustly can't read from a keyboard, so in this chapter your guesses are plain strings. The logic is exactly the same.

## From `String` to number

Text and numbers are different types in Rust, and you can't compare one with the other. To get a number, `trim` the text to remove spaces and the newline, then `parse` it:

```rust,editable title="Trim, then parse"
fn main() {
    let guess = "42\n";
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
}
```

Two things to notice:

- `let guess: u32 = ...` **shadows** the text version of `guess` with a number. Reusing the name is common when you convert a value from one type to another.
- The `: u32` annotation tells `parse` which type to produce. Without it, Rust can't tell what you want.

## When parsing fails

`parse` can't turn `"forty-two"` into a number, so it doesn't return a number directly. It returns a **`Result`**, which is either `Ok(number)` or `Err(error)`. Calling `.expect(...)` says "give me the number, or crash with this message":

```rust,panics title="expect on bad input"
fn main() {
    let guess = "forty-two";
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
}
```

Crashing because a player made a typo isn't friendly. Use `match` instead to handle both cases. `match` checks a value against patterns, top to bottom, and runs the arm that fits:

```rust,editable title="Handling bad input with match"
fn main() {
    for input in ["42", " 7\n", "forty-two"] {
        match input.trim().parse::<u32>() {
            Ok(num) => println!("{input:?} is the number {num}"),
            Err(_) => println!("{input:?} is not a number"),
        }
    }
}
```

The `_` means "any error, I don't need the details". The `::<u32>` after `parse` (called the "turbofish") is another way to say which type you want.

## `Option`: maybe a value

Sometimes you only care whether you got a value or not. That's what **`Option`** is for: `Some(value)` or `None`. Calling `.ok()` turns a `Result` into an `Option`, dropping the error details:

```rust,runnable title="Result to Option"
fn main() {
    let good: Option<u32> = "42".parse().ok();
    let bad: Option<u32> = "abc".parse().ok();
    println!("{good:?} and {bad:?}");
}
```

## Quick recap

- Input arrives as text, often with a trailing newline. `trim` it first.
- `parse` returns a `Result`. Use `match` to handle `Ok` and `Err`.
- `Option` holds `Some(value)` or `None`, and `.ok()` turns a `Result` into one.

The checkpoint below has a parser that rejects real guesses. Find out why and fix it.
