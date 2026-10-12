+++
id = "tokio.framing.connection"
chapter = "tokio.framing"
requires = []
title = "A Frame at a Time"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/framing#buffered-reads"
summary = "A Connection wraps a stream and a buffer. read_frame loops: parse a frame if one is buffered, otherwise read more. Writing a frame is a length, the payload and a flush."
+++

# A Frame at a Time

The Tokio tutorial wraps a socket in a `Connection` struct, so the rest of the server can forget about bytes and think in frames:

```rust
struct Connection {
    stream: DuplexStream, // a TcpStream in the real thing
    buffer: BytesMut,
}
```

## Reading: parse, or read more

`read_frame` loops between two steps:

1. **Try to parse** a frame from what's already buffered. If one is there, return it. There might be several frames buffered from a single read, so always try this first.
2. **Read more** bytes into the buffer and go around again.

`read_buf(&mut self.buffer)` is like `read`, but it appends to a `BytesMut` and grows it as needed, so you don't manage a `[u8; N]` array yourself. It returns how many bytes it added, and **0 means the other side closed**. That's either a clean ending (the buffer is empty: return `Ok(None)`) or a broken one (half a frame is stuck in the buffer: return an error).

```rust,runnable title="read_frame"
use bytes::{Buf, Bytes, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, DuplexStream};

struct Connection {
    stream: DuplexStream,
    buffer: BytesMut,
}

impl Connection {
    async fn read_frame(&mut self) -> io::Result<Option<Bytes>> {
        loop {
            if let Some(frame) = parse_frame(&mut self.buffer) {
                return Ok(Some(frame));
            }
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "connection closed mid-frame"))
                };
            }
        }
    }
}

fn parse_frame(buf: &mut BytesMut) -> Option<Bytes> {
    if buf.len() < 4 {
        return None;
    }
    let len = (&buf[..4]).get_u32() as usize;
    if buf.len() < 4 + len {
        return None;
    }
    buf.advance(4);
    Some(buf.split_to(len).freeze())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (mut client, server) = io::duplex(3); // a very slow connection
    tokio::spawn(async move {
        for word in ["hello", "framing"] {
            client.write_u32(word.len() as u32).await.unwrap();
            client.write_all(word.as_bytes()).await.unwrap();
        }
    });

    let mut conn = Connection { stream: server, buffer: BytesMut::new() };
    while let Some(frame) = conn.read_frame().await? {
        println!("frame: {frame:?}");
    }
    println!("clean close");
    Ok(())
}
```

Even though the pipe only carries 3 bytes at a time, each frame comes out whole.

## Writing a frame

Writing is simpler: the length, then the payload. `write_u32` writes a big-endian `u32`. Real servers wrap the stream in a `BufWriter` so a frame goes out in one write instead of several tiny ones; with a `BufWriter`, call `flush()` once the frame is complete, or the bytes may sit in its buffer.

```rust
async fn write_frame(&mut self, payload: &[u8]) -> io::Result<()> {
    self.stream.write_u32(payload.len() as u32).await?;
    self.stream.write_all(payload).await?;
    self.stream.flush().await
}
```
