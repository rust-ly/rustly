+++
id = "book.generics.lifetimes"
chapter = "book.generics"
requires = []
title = "Lifetimes: Keeping References Valid"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
summary = "Why references can't outlive their data, and how a lifetime annotation like 'a explains that to the compiler."
+++

# Validating References with Lifetimes

Every reference has a **lifetime**: the stretch of code where it's valid. Most of the time the compiler works lifetimes out on its own, just as it works out most types. This concept is about the cases where it needs your help.

## The problem lifetimes solve: dangling references

A reference must never outlive the value it points to. Here, `x` is dropped at the end of the inner block, but `r` would still point to it:

```rust,does_not_compile title="A reference that outlives its value"
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
}
```

The error, `E0597: x does not live long enough`, comes from the **borrow checker**. It compares how long `x` lives with how long `r` is used, and sees that `r` is used after `x` is gone. In languages without this check, that's a *dangling pointer*: reading memory that may already hold something else.

## When the compiler can't tell

Now a function that returns the longer of two string slices:

```rust,does_not_compile title="Which input does the result borrow from?"
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("{}", longest("abcd", "xyz"));
}
```

The error is `E0106: missing lifetime specifier`. The result is a reference, but is it borrowed from `x` or from `y`? It depends on the values at run time, so the compiler can't tell how long the result stays valid.

## Lifetime annotations

A **lifetime annotation** names a lifetime with an apostrophe, usually `'a`. It doesn't change how long anything lives; it **describes a relationship** so the compiler can check it:

```rust,editable title="longest with a lifetime"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
```

Read the signature as: "for some lifetime `'a`, both inputs live at least as long as `'a`, and the result is valid for `'a` too." In practice, the result is valid only as long as **the shorter-lived** of the two inputs.

That's why the example prints **inside** the inner block. Move the `println!` after the closing brace and it won't compile: `string2` is gone, and `result` might be borrowing from it, even though here it happens to point at `string1`. The compiler goes by the signature, not by which branch ran.

## Only annotate what's related

If the result only ever comes from `x`, only `x` needs the lifetime. `y` has nothing to do with the result:

```rust,runnable title="The result only borrows from x"
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn main() {
    println!("{}", first("hello", "world"));
}
```
