+++
id = "tokio.io.read-write"
chapter = "tokio.io"
requires = []
title = "Reading and Writing"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/io"
summary = "AsyncReadExt and AsyncWriteExt give you read, read_to_end, write and write_all. A single read may return only part of the data; 0 means the other side is done."
+++

# Reading and Writing

Async I/O in Tokio looks a lot like `std::io`. Two traits describe things you can read from and write to, `AsyncRead` and `AsyncWrite`, and you rarely call them directly. Instead you import the **extension traits** `AsyncReadExt` and `AsyncWriteExt`, which add handy `async` methods such as `read` and `write_all`.

Sockets and files implement these traits, but so do plain bytes: `&[u8]` is a reader and `Vec<u8>` is a writer. That makes them perfect for trying things out:

```rust,runnable title="Bytes in, bytes out"
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello, tokio";

    let mut first = [0u8; 5];
    let n = reader.read(&mut first).await?;
    println!("read {n} bytes: {:?}", std::str::from_utf8(&first[..n]).unwrap());

    let mut rest = Vec::new();
    reader.read_to_end(&mut rest).await?;
    println!("the rest: {:?}", String::from_utf8(rest).unwrap());

    let mut out = Vec::new();
    out.write_all(b"written!").await?;
    println!("{}", String::from_utf8(out).unwrap());
    Ok(())
}
```

## `read` gives you *some* bytes

`read(&mut buf)` waits until **some** data is available, copies it into `buf` and returns how many bytes it copied. That may be fewer than the buffer holds, and fewer than the other side sent. Over a real network, a message often arrives in several pieces.

Two return values matter:

- **`Ok(n)` with `n > 0`**: `n` more bytes are in `buf[..n]`. There may be more to come.
- **`Ok(0)`**: the other side closed its end. Nothing more will ever arrive.

So reading a whole message means **looping until `read` returns 0**, or letting `read_to_end` / `read_to_string` loop for you.

## In-memory pipes

`tokio::io::duplex(capacity)` makes two connected ends, like a socket with no network. Bytes written to one end come out of the other. The `capacity` is how many bytes can wait in the pipe, so a small capacity forces data through in small pieces, just like a slow connection:

```rust,runnable title="A message in pieces"
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (mut client, mut server) = io::duplex(4);

    tokio::spawn(async move {
        client.write_all(b"one message").await.unwrap();
        // Dropping `client` closes its end.
    });

    let mut buf = [0u8; 64];
    loop {
        let n = server.read(&mut buf).await?;
        if n == 0 {
            println!("closed");
            break;
        }
        println!("got {n} bytes: {:?}", std::str::from_utf8(&buf[..n]).unwrap());
    }
    Ok(())
}
```

## `write` vs `write_all`

`write` has the same catch in the other direction: it may accept only part of your bytes and returns how many it took. `write_all` keeps writing until everything is sent, so it's almost always what you want.
