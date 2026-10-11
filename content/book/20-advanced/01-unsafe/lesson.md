+++
id = "book.advanced.unsafe"
chapter = "book.advanced"
requires = []
title = "Unsafe Rust"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html"
summary = "The five extra things an unsafe block allows, raw pointers, and wrapping unsafe code in a safe API."
+++

# Unsafe Rust

The borrow checker is **conservative**: it rejects some programs that are actually fine, because it can't prove they're fine. And some low-level work (talking to hardware or the operating system, building data structures like `Vec`) is inherently impossible to check. For these cases, Rust has **unsafe** code.

`unsafe` doesn't turn off the borrow checker. An `unsafe` block lets you do exactly **five** extra things:

1. Dereference a **raw pointer**.
2. Call an **unsafe function** or method.
3. Access or modify a **mutable static** variable.
4. Implement an **unsafe trait** (like `Send`, chapter 16).
5. Access fields of a **union**.

Everything else is still checked. The point of `unsafe` is to put the code that **you** must check by hand in small, clearly marked blocks, so if a memory bug appears, you know where to look.

## Raw pointers

Raw pointers, `*const T` and `*mut T`, are like references without the rules: they can be null, can dangle, and several `*mut` can point to the same thing. **Creating** them is safe; **dereferencing** them needs `unsafe`:

```rust,runnable title="Raw pointers"
fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r1 is now: {}", *r1);
    }
}
```

`&raw const` and `&raw mut` create raw pointers. Here we created them from a valid variable, so we know they're fine. The compiler doesn't, which is why the dereference is in an `unsafe` block.

## Safe abstractions over unsafe code

The standard library's `split_at_mut` gives you two **mutable** halves of one slice. The borrow checker can't tell that the halves don't overlap, so this version doesn't compile:

```rust,does_not_compile title="Two mutable borrows of one slice"
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    (&mut values[..mid], &mut values[mid..])
}

fn main() {}
```

Using raw pointers and an unsafe function, `slice::from_raw_parts_mut`, it works. The function as a whole is **safe to call**, because the `assert!` guarantees the halves never overlap:

```rust,editable title="A safe function built on unsafe code"
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    a[0] = 100;
    b[0] = 400;
    println!("{v:?}");
}
```

This is the pattern to aim for: a small `unsafe` block with a clear reason it's correct, wrapped in a function anyone can call safely. Other uses you'll meet: `extern "C"` blocks for calling C libraries (calling them is always `unsafe`), and `static mut` globals (accessing them is `unsafe` because threads could race on them).
