+++
id = "book.smart-pointers.rc"
chapter = "book.smart-pointers"
requires = []
title = "Rc<T>: Shared Ownership"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch15-04-rc.html"
summary = "Rc counts how many owners a value has, and frees it when the last one is gone."
+++

# `Rc<T>`, the Reference Counted Smart Pointer

Normally a value has exactly **one** owner. But some data really is shared. In a graph, several edges point at the same node, and the node should live as long as **any** of them still does.

`Rc<T>` (**r**eference **c**ounted) allows multiple owners. It keeps a count of how many `Rc`s point at the value, and frees the value when the count reaches zero. Think of a TV in a family room: the last person to leave turns it off.

## A list with a shared tail

Two lists, `b` and `c`, both continue into list `a`:

```text
b: 3 ──┐
       ├──> a: 5 -> 10 -> Nil
c: 4 ──┘
```

With `Box`, this is impossible: moving `a` into `b` means `c` can't have it too.

```rust,does_not_compile title="Two owners with Box"
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

With `Rc`, each list gets its **own** `Rc` pointing at the shared tail. `Rc::clone(&a)` doesn't copy the list; it just adds one to the count:

```rust,editable title="Sharing a tail with Rc"
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

You could write `a.clone()`, but the convention is `Rc::clone(&a)`, which makes it obvious this is a cheap count bump, not a deep copy.

## Read-only sharing

`Rc<T>` only gives out **shared** references to its value. If several owners could each change the data, that would break the borrowing rules, so it isn't allowed. (The next concept shows how to get mutation back, safely.)

`Rc<T>` is also for **single-threaded** code only. For threads there's `Arc<T>`, which works the same way and is covered in chapter 16.
