+++
id = "book.generics.extracting-functions"
chapter = "book.generics"
requires = []
title = "Removing Duplication"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function"
summary = "Spot repeated code and pull it into a function: the first step towards generics."
+++

# Removing Duplication by Extracting a Function

This chapter is about writing code **once** and using it for many situations. Before generics, it's worth looking at the simplest version of that idea: turning repeated code into a function.

Here's a program that finds the largest number in a list:

```rust,runnable title="Finding the largest number"
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
}
```

Now we need the largest of a **second** list. Copying the loop works, but every copy is another place to update and another place for a bug to hide:

```rust,runnable title="The same loop, copied"
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");
}
```

## Extract a function

The two copies differ only in **which list** they look at. So make the list a parameter. `&[i32]` is a slice, which accepts any list of `i32`s, whether it's a vector or an array:

```rust,editable title="One function, any list of i32"
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest(&[34, 50, 25, 100, 65]));
    println!("{}", largest(&vec![102, 34, 6000, 89, 54, 2, 43, 8]));
}
```

The steps were:

1. Notice the duplicated code.
2. Move it into a function, and make the parts that differ into **parameters**.
3. Replace each copy with a call.

Generics are the same steps, but what differs is a **type** instead of a value. What if we want the largest `char` in a list, or the largest `f64`? Right now we'd need `largest_char`, `largest_f64`... all with identical bodies. The next concept fixes that.
