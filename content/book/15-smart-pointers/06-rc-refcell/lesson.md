+++
id = "book.smart-pointers.rc-refcell"
chapter = "book.smart-pointers"
requires = []
title = "Rc<RefCell<T>>: Shared and Mutable"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt"
summary = "Combine Rc and RefCell to get data with several owners that any of them can change."
+++

# Multiple Owners of Mutable Data with `Rc<RefCell<T>>`

The two pointers from the last concepts solve different halves of a problem:

- `Rc<T>` gives a value **several owners**, but only lets them read it.
- `RefCell<T>` lets a value be **changed** through a shared reference.

Put one inside the other, `Rc<RefCell<T>>`, and you get data with several owners that **any** of them can change.

```rust,runnable title="A shared, changeable value"
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let score = Rc::new(RefCell::new(0));

    let player_one = Rc::clone(&score);
    let player_two = Rc::clone(&score);

    *player_one.borrow_mut() += 10;
    *player_two.borrow_mut() += 5;

    println!("shared score: {}", score.borrow());
}
```

All three variables point at the **same** `RefCell`. `borrow_mut()` gives a mutable borrow, and `*` reaches the number inside it.

## The Book's example: a shared list value

Lists `b` and `c` share list `a`, as in the `Rc` concept, but now the value at the front of `a` can be changed, and both `b` and `c` see the change:

```rust,editable title="Changing a value inside a shared list"
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```

## A common mistake

A `borrow()` or `borrow_mut()` lasts as long as the value it returns is alive. Keeping a borrow in a variable while trying to take another one panics:

```rust,panics title="Holding a borrow too long"
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let reader = shared.borrow();
    shared.borrow_mut().push(4);
    println!("{reader:?}");
}
```

Keep borrows short: borrow, use, and let it drop, ideally all in one expression.
