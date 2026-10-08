+++
id = "book.guessing-game.comparing-guesses"
chapter = "book.guessing-game"
requires = []
title = "Comparing the Guess"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number"
summary = "Compare two numbers with `cmp`, `match` on the `Ordering`, and loop until the player wins."
+++

# Comparing the Guess

Once the guess is a number, the game needs to know whether it's too small, too big, or exactly right. Every number has a `cmp` method that compares it with another one and returns a `std::cmp::Ordering`. An `Ordering` has three possible values: `Less`, `Greater` and `Equal`.

```rust,editable title="cmp and Ordering"
use std::cmp::Ordering;

fn main() {
    let secret = 50;
    let guess = 30;

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

`use std::cmp::Ordering;` at the top brings the type into scope, so you can write `Ordering::Less` instead of the full path. The `&` in `cmp(&secret)` lends `secret` to `cmp` without giving it away. Chapter 4 explains borrowing properly.

`match` must cover **every** possible value. Delete one of the three arms above and run it: the compiler tells you exactly which case you forgot.

## Types have to match

You can only compare values of the same type. Comparing the raw `String` from the keyboard with a number doesn't compile, which is why the last concept parsed the guess first:

```rust,does_not_compile title="Comparing text with a number"
use std::cmp::Ordering;

fn main() {
    let secret: u32 = 50;
    let guess = String::from("30");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

## Looping until the player wins

A game where you only get one guess isn't much fun. `loop` repeats its body forever. `break` leaves the loop, and `continue` skips straight to the next time around. This is the same shape as the Book's game, with a list of guesses standing in for the keyboard:

```rust,runnable title="The game loop"
use std::cmp::Ordering;

fn main() {
    let secret: u32 = 42;
    let mut inputs = ["50", "oops", "25", "42", "99"].into_iter();

    loop {
        let Some(input) = inputs.next() else { break };
        println!("Guess: {input}");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

The line `let Some(input) = inputs.next() else { break };` takes the next guess, or leaves the loop when there are none left. You'll see this `let ... else` pattern again in chapter 6.

In the Book, the secret number comes from the `rand` crate, which picks a different one each time. Your exercises take the secret as an argument, so the tests can check them.

## Quick recap

- `a.cmp(&b)` returns an `Ordering`: `Less`, `Greater` or `Equal`.
- `match` must handle every case, and the compiler checks that it does.
- `loop` repeats, `break` stops, and `continue` skips to the next round.

The checkpoint below is the heart of the game: comparing a guess with the secret.
