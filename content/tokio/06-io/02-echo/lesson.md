+++
id = "tokio.io.echo"
chapter = "tokio.io"
requires = []
title = "An Echo Server"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/io#echo-server"
summary = "io::copy moves bytes from a reader to a writer. To read and write the same stream at once, split it into a read half and a write half."
+++

# An Echo Server

An echo server sends back everything it receives. The Tokio tutorial builds one over TCP; we'll use a `duplex` pipe as the socket, since the Playground has no network. The ideas are exactly the same.

## `io::copy`

`tokio::io::copy(&mut reader, &mut writer)` reads until the reader returns 0 and writes everything to the writer. It returns how many bytes it copied. That's an echo server in one line, if only we could pass the same socket as both arguments:

```rust,does_not_compile title="Two mutable borrows"
use tokio::io::{self, DuplexStream};

async fn echo(mut socket: DuplexStream) -> io::Result<u64> {
    io::copy(&mut socket, &mut socket).await
}

#[tokio::main]
async fn main() {}
```

The compiler refuses: `socket` can't be borrowed mutably twice at the same time.

## Splitting a stream

`io::split` turns one stream into a **read half** and a **write half** that can be used separately, even from different tasks:

```rust,runnable title="Echo with io::split"
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, DuplexStream};

async fn echo(socket: DuplexStream) -> io::Result<u64> {
    let (mut reader, mut writer) = io::split(socket);
    io::copy(&mut reader, &mut writer).await
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (mut client, server) = io::duplex(64);
    let server_task = tokio::spawn(echo(server));

    client.write_all(b"ping").await?;
    client.shutdown().await?; // "I'm done sending."

    let mut reply = String::new();
    client.read_to_string(&mut reply).await?;
    println!("reply: {reply}");
    println!("echoed {} bytes", server_task.await.unwrap()?);
    Ok(())
}
```

`shutdown()` closes the client's **sending** side, so the server's next `read` returns 0 and `copy` finishes. The client can still read the reply. When `echo` returns, both halves are dropped, which closes the server's end, and the client's `read_to_string` finishes too.

TCP sockets have a cheaper `into_split()` / `split()` of their own, but `io::split` works on any stream.

## Reading lines

Many protocols send one message per line. Wrap a reader in `BufReader` and the `AsyncBufReadExt` trait adds `lines()`, which hands you one line at a time without the line ending:

```rust,runnable title="Line by line"
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let input: &[u8] = b"first\nsecond\nthird";
    let mut lines = BufReader::new(input).lines();
    while let Some(line) = lines.next_line().await? {
        println!("line: {line}");
    }
    Ok(())
}
```

`next_line` returns `Ok(None)` once the reader is closed. A last line without a trailing `\n` still counts.
