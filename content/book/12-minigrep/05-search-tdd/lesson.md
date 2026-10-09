+++
id = "book.minigrep.search-tdd"
chapter = "book.minigrep"
requires = []
title = "Test-Driven Development: search"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html"
summary = "Write a failing test first, then just enough code to make it pass: the search function."
+++

# Developing the Library's Functionality with Test-Driven Development

With the logic in `lib.rs`, minigrep's heart, the `search` function, can be built **test-first**. Test-driven development (TDD) is a loop:

1. Write a test that **fails**, because the code doesn't exist or doesn't work yet.
2. Write **just enough** code to make it pass.
3. Refactor, keeping the test green.
4. Repeat.

Writing the test first forces you to decide exactly what the function should do before you write it.

## Step 1: a failing test

The test says: searching for `"duct"` in this text returns only the line containing it.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

The first `search` just returns an empty vector, so the code compiles, the test runs and **fails**. That proves the test can fail.

## Step 2: the lifetime in the signature

`search` returns lines **borrowed from `contents`**, not new strings, so the signature ties the result to `contents` with a lifetime (chapter 10). `query` isn't related to the result, so it doesn't get one:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
```

Without `'a`, Rust can't tell which input the returned slices borrow from (E0106).

## Step 3: make it pass

Go through each line, keep the ones that contain the query, and return them:

```rust,editable title="search, implemented"
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn main() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.";
    println!("{:?}", search("duct", contents));
    println!("{:?}", search("Pick", contents));
}
```

`line.contains(query)` does the substring check, and `push(line)` stores a slice of the original text with no copying. Run the test now and it passes.

Because each result is a `&'a str` pointing into `contents`, the results can't outlive the text they came from. If you dropped `contents` and kept the results, the borrow checker would stop you.
