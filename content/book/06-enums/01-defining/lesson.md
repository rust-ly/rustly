+++
id = "book.enums.defining"
chapter = "book.enums"
requires = []
title = "Defining an Enum"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html"
summary = "A type whose value is exactly one of a fixed list of variants."
+++

# Defining an Enum

A struct says "a value has **all** of these fields": a rectangle has a width **and** a height. An **enum** (short for *enumeration*) says the opposite: a value is **exactly one of** a fixed list of possibilities, called **variants**.

Think of the suits in a deck of cards. A card's suit is clubs, diamonds, hearts **or** spades. It's never two at once, and there's no fifth option. That's an enum:

```rust,runnable title="Four suits, one type"
#[derive(Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn main() {
    let card = Suit::Hearts;
    println!("This card is a {card:?}");
}
```

## Variants live inside the enum's name

You write a variant with the enum's name, two colons, then the variant: `Suit::Hearts`. This keeps names tidy: `Suit::Clubs` and a golf `Club` type can't clash.

All four variants have the **same type**, `Suit`. That's the point: one function can accept any of them.

```rust,editable title="One function, any suit"
#[derive(Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn announce(suit: Suit) {
    println!("You drew a {suit:?}!");
}

fn main() {
    announce(Suit::Spades);
    announce(Suit::Diamonds);
}
```

Try adding a fifth variant, `Jokers`, and announcing it.

## Comparing variants

To check which variant you have, the next concepts introduce `match`, Rust's main tool for enums. For simple yes/no checks, you can also derive `PartialEq` (as you did for structs) and compare with `==`:

```rust,runnable title="Comparing with =="
#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn main() {
    let suit = Suit::Clubs;
    println!("Is it clubs? {}", suit == Suit::Clubs);
    println!("Is it hearts? {}", suit == Suit::Hearts);
}
```

## A common mistake

Forgetting the enum's name in front of a variant is the most common slip. Rust doesn't know what a bare `Hearts` is:

```rust,does_not_compile title="Missing the Suit:: prefix"
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn main() {
    let card: Suit = Hearts;
}
```

The error, `E0425: cannot find value Hearts in this scope`, even suggests the fix: `Suit::Hearts`.
