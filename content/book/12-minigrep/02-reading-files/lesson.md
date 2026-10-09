+++
id = "book.minigrep.reading-files"
chapter = "book.minigrep"
requires = []
title = "Reading a File"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch12-02-reading-a-file.html"
summary = "Read a whole file into a String with fs::read_to_string, then work through it line by line."
+++

# Reading a File

minigrep needs the text it's going to search. On your machine, `std::fs::read_to_string` reads a whole file into a `String`. It returns a `Result`, because the file might not exist:

```rust
use std::fs;

fn main() {
    let file_path = "poem.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
```

The Book searches a short Emily Dickinson poem. Since the Playground has no files, we'll keep it in a constant instead. A string literal can span several lines, and starting it with a backslash right after the opening quote skips the first newline:

```rust,runnable title="The poem, as a constant"
const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

fn main() {
    println!("With text:\n{POEM}");
}
```

## Working line by line

grep works on **lines**. `.lines()` splits text at each line ending (and handles both `\n` and Windows-style `\r\n`):

```rust,editable title="Numbered lines"
const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

fn main() {
    let mut number = 1;
    for line in POEM.lines() {
        println!("{number:>2}: {line}");
        number += 1;
    }
}
```

`{number:>2}` right-aligns the number in two characters, so the lines line up.

Right now `main` would do everything: read arguments, read the file, and (soon) search. That's fine for ten lines of code, but it's about to get messy. The next concept splits it up.
