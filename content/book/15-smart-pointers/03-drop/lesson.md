+++
id = "book.smart-pointers.drop"
chapter = "book.smart-pointers"
requires = []
title = "The Drop Trait"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch15-03-drop.html"
summary = "Run your own clean-up code when a value goes out of scope, and drop a value early with std::mem::drop."
+++

# Running Code on Cleanup with the `Drop` Trait

When a value goes out of scope, Rust cleans it up: a `Box` frees its heap memory, a `File` closes, a lock is released. You can run your own code at that moment by implementing the **`Drop` trait**, which has one method, `drop`:

```rust,runnable title="Seeing values get dropped"
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

Run it and notice two things:

- You never call `drop` yourself. Rust calls it automatically when each value goes out of scope.
- Values are dropped in **reverse** order of creation: `d` first, then `c`. Later values may depend on earlier ones, so they're cleaned up first.

This pattern, tying clean-up to a value's scope so it can't be forgotten, is called **RAII**. It's how Rust closes files and releases locks without a garbage collector or `finally` blocks.

## Dropping a value early

Sometimes you need clean-up to happen **before** the end of the scope, for example to release a lock so other code can take it. You can't call the `drop` method directly:

```rust,does_not_compile title="Calling drop() yourself"
struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("released");
    }
}

fn main() {
    let g = Guard;
    g.drop();
}
```

The error, `E0040: explicit use of destructor method`, prevents a **double free**: Rust would still call `drop` again at the end of the scope. Instead, call the standard function `std::mem::drop` (it's in the prelude, so just `drop`). It takes the value **by ownership**, so it really is gone:

```rust,editable title="std::mem::drop"
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("released {}", self.0);
    }
}

fn main() {
    let a = Guard("a");
    let b = Guard("b");
    drop(a);
    println!("a was dropped early; b is still here");
    let _ = &b;
}
```

There's nothing special inside `std::mem::drop`: its body is empty! Taking ownership is enough, because the value goes out of scope as the function returns.
