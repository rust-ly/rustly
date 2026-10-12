+++
id = "tokio.framing.frames"
chapter = "tokio.framing"
requires = []
title = "Frames and Buffers"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/framing"
summary = "A byte stream has no message boundaries, so protocols add them. A length prefix says how many bytes follow; a BytesMut buffer holds bytes until a whole frame is there."
+++

# Frames and Buffers

In the I/O chapter you saw that `read` hands you whatever bytes have arrived. Two messages sent one after the other can come out glued together, or split in odd places. A byte stream has **no message boundaries**.

So protocols add their own. A **frame** is one complete message on the wire, and **framing** is turning a stream of bytes into a sequence of frames. Lines (ending in `\n`) are one way to do it. Another, common in binary protocols, is a **length prefix**: first 4 bytes giving the length as a big-endian `u32`, then exactly that many bytes of payload.

```text
00 00 00 05  h e l l o   00 00 00 02  h i
└─ len 5 ─┘  └payload┘   └─ len 2 ─┘  └─┘
```

## `BytesMut`: a buffer that grows and shrinks

The `bytes` crate, which Tokio uses everywhere, has `BytesMut`: a byte buffer you can append to at the end and cut pieces off the front, cheaply. `Bytes` is its frozen, read-only, cheap-to-clone sibling. The `Buf` and `BufMut` traits add helpers like `get_u32`, `put_u32` and `advance`:

```rust,runnable title="Writing and cutting frames"
use bytes::{Buf, BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::new();
    for word in ["hello", "hi"] {
        buf.put_u32(word.len() as u32);
        buf.put_slice(word.as_bytes());
    }
    println!("on the wire: {:?}", &buf[..]);

    let len = buf.get_u32() as usize; // reads 4 bytes and removes them
    let frame = buf.split_to(len).freeze(); // cuts off the next `len` bytes
    println!("frame: {frame:?}, still buffered: {:?}", &buf[..]);
}
```

## Don't take what isn't there yet

A reader fills the buffer a piece at a time, so at any moment it may hold half a length, or a length and half a payload. Parsing has to **look before it takes**:

1. Fewer than 4 bytes? Not enough to know the length yet.
2. Peek at the length **without removing it**: `(&buf[..4]).get_u32()` reads from a copy of the slice, so `buf` is unchanged.
3. Fewer than `4 + len` bytes? The payload isn't complete yet.
4. Only now remove the prefix and split off the payload.

If any check fails, leave the buffer exactly as it was and wait for more bytes. Taking half a frame would lose the rest's place in the stream.

Calling `get_u32` on a buffer with fewer than 4 bytes panics, and `split_to(len)` panics if `len` is more than what's there:

```rust,panics title="Taking too early"
use bytes::{Buf, BytesMut};

fn main() {
    let mut buf = BytesMut::from(&[0u8, 0, 0, 5, b'h', b'e'][..]);
    let len = buf.get_u32() as usize;
    let frame = buf.split_to(len); // only 2 of the 5 bytes are here
    println!("{frame:?}");
}
```
