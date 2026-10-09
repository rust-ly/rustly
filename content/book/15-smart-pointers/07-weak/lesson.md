+++
id = "book.smart-pointers.weak"
chapter = "book.smart-pointers"
requires = []
title = "Reference Cycles and Weak<T>"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch15-06-reference-cycles.html"
summary = "Rc values that point at each other never get freed; Weak references point without owning, which breaks the cycle."
+++

# Reference Cycles Can Leak Memory

Rust's guarantees make memory bugs rare, but one kind of leak is possible: a **reference cycle**. If value A holds an `Rc` to B and B holds an `Rc` to A, neither count ever reaches zero, so neither is ever freed, even after every variable pointing at them is gone. It's like two people each waiting for the other to leave the room before turning off the TV.

`Rc` alone can't build a cycle (it's read-only), but `Rc<RefCell<T>>` can, by changing a pointer after both values exist. The usual fix is to make one direction of the link **not count**.

## `Weak<T>`: a pointer that doesn't own

`Rc::downgrade(&rc)` creates a `Weak<T>`. It points at the value but doesn't keep it alive: it increases the **weak** count, not the strong count, and values are freed when the **strong** count reaches zero.

Because the value might already be gone, you can't use a `Weak` directly. `.upgrade()` returns an `Option<Rc<T>>`: `Some` if the value still exists, `None` if it was dropped.

```rust,runnable title="Weak doesn't keep a value alive"
use std::rc::{Rc, Weak};

fn main() {
    let weak: Weak<String>;
    {
        let strong = Rc::new(String::from("still here"));
        weak = Rc::downgrade(&strong);
        println!("inside: {:?}", weak.upgrade());
    }
    println!("outside: {:?}", weak.upgrade());
}
```

## A tree: children own, parents don't

In a tree, a parent should **own** its children: dropping the parent drops them. But a child also wants to know its parent. If that link were an `Rc`, parent and child would own each other: a cycle. So the child's link to its parent is `Weak`:

```rust,editable title="A tree with Weak parent links"
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
}
```

`Weak::new()` starts as a link to nothing. After the leaf's parent is set, `branch` has one strong owner (the variable) and one weak reference (from the leaf). When `branch` goes out of scope, its strong count drops to zero and it's freed, even though the leaf still has a weak link to it.

A good rule: **owners point down with `Rc`, back-references point up with `Weak`**.
