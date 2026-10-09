+++
id = "book.smart-pointers.refcell"
chapter = "book.smart-pointers"
requires = []
title = "RefCell<T> and Interior Mutability"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch15-05-interior-mutability.html"
summary = "Change data through a shared reference by moving the borrow checks from compile time to run time."
+++

# `RefCell<T>` and the Interior Mutability Pattern

The borrowing rules say: at any time, **either** one mutable reference **or** any number of shared ones. Normally the compiler checks this. But sometimes code is safe and the compiler just can't prove it, and you need to change data that's behind a shared reference.

**Interior mutability** is the pattern for that. `RefCell<T>` holds a value and lets you borrow it mutably even through an `&RefCell<T>`, by checking the borrowing rules **at run time** instead of compile time:

- `.borrow()` returns a shared borrow (`Ref<T>`).
- `.borrow_mut()` returns a mutable borrow (`RefMut<T>`).
- Each borrow ends when its `Ref`/`RefMut` is dropped.

```rust,runnable title="Changing a value through a shared reference"
use std::cell::RefCell;

fn add_item(list: &RefCell<Vec<String>>, item: &str) {
    list.borrow_mut().push(item.to_string());
}

fn main() {
    let list = RefCell::new(Vec::new());
    add_item(&list, "milk");
    add_item(&list, "eggs");
    println!("{:?}", list.borrow());
}
```

`add_item` only has a shared reference, `&RefCell<...>`, yet it adds to the vector.

## The rules still apply, at run time

If code breaks the rules, it compiles, but **panics** when it runs:

```rust,panics title="Two mutable borrows at once"
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(5);
    let first = cell.borrow_mut();
    let second = cell.borrow_mut();
    println!("{first} {second}");
}
```

The panic says `already borrowed: BorrowMutError`. That's the trade-off: more flexibility, but mistakes show up while the program runs instead of at compile time. Prefer normal borrowing, and reach for `RefCell` only when you need it.

## A use case: mock objects in tests

The Book's example is a `LimitTracker` that warns users approaching a quota. It sends messages through a `Messenger` trait whose `send` method takes `&self`. To test it, a mock messenger should **record** the messages it's asked to send, but `send` only gets `&self`, so it can't change a normal `Vec`. A `RefCell<Vec<String>>` solves it:

```rust,editable title="A mock messenger"
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

fn main() {
    let mock = MockMessenger { sent_messages: RefCell::new(vec![]) };
    mock.send("You've used up over 75% of your quota!");
    println!("{} message(s): {:?}", mock.sent_messages.borrow().len(), mock.sent_messages.borrow());
}
```

`RefCell<T>`, like `Rc<T>`, is single-threaded. For threads, `Mutex<T>` plays the same role (chapter 16).
