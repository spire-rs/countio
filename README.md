<div align="center">

# countio

**Byte counting for readers and writers in Rust.**

Wrap any I/O object to count the bytes read from and written to it, for
`std::io` and the async traits from tokio and futures.

[![Build](https://img.shields.io/github/actions/workflow/status/spire-rs/countio/build.yml?branch=main&label=build&style=flat-square)](https://github.com/spire-rs/countio/actions/workflows/build.yml)
[![Security](https://img.shields.io/github/actions/workflow/status/spire-rs/countio/security.yml?branch=main&label=security&style=flat-square)](https://github.com/spire-rs/countio/actions/workflows/security.yml)
[![Crates.io](https://img.shields.io/crates/v/countio.svg?style=flat-square)](https://crates.io/crates/countio)
[![Docs](https://img.shields.io/docsrs/countio?style=flat-square)](https://docs.rs/countio)

[**docs.rs/countio**](https://docs.rs/countio) · [**crates.io/crates/countio**](https://crates.io/crates/countio) · [**github.com/spire-rs**](https://github.com/spire-rs)

</div>

`Counter` wraps a reader, writer or both and tracks the bytes that pass through
it, with read and write counts kept separately for bidirectional streams such as
`TcpStream` or `Cursor`. `Progress` adds expected totals on top, so it can
report how far along a transfer is.

## Features

- **Sync and async**: `Read`, `BufRead`, `Write` and `Seek` from `std::io`, and `AsyncRead`, `AsyncBufRead`, `AsyncWrite` and `AsyncSeek` from tokio and futures.
- **Bidirectional counting**: read and write bytes are counted independently, with `total_bytes` as a `u128` so the sum cannot overflow.
- **Progress tracking**: set the expected reader or writer bytes and get the completion percentage of each.
- **Transparent wrapper**: reach the inner object with `get_ref`, `get_mut` and `into_inner`, and reset the counts with `reset`.
- **No unsafe code** and no required dependencies.

| Feature | What it enables |
| --- | --- |
| `std` | `std::io::{Read, BufRead, Write, Seek}`. **Enabled by default.** |
| `tokio` | `tokio::io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek}` |
| `futures` | `futures_io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek}` |
| `full` | All of the above. |

## Requirements

- **Rust + Cargo**: 1.85+, Edition 2024

## Quick start

Add countio, with the features for the I/O traits you use:

```toml
[dependencies]
countio = { version = "0.3", features = ["tokio"] }
```

Count the bytes read from a reader:

```rust
use std::io::{BufRead, BufReader, Result};

use countio::Counter;

fn main() -> Result<()> {
    let reader = BufReader::new("Hello World!".as_bytes());
    let mut reader = Counter::new(reader);

    let mut buf = String::new();
    let len = reader.read_line(&mut buf)?;

    assert_eq!(len, reader.reader_bytes());
    Ok(())
}
```

Track progress against an expected size, in both directions:

```rust
use std::io::{Cursor, Read, Result, Write};

use countio::Progress;

fn main() -> Result<()> {
    let cursor = Cursor::new(vec![0u8; 100]);
    let mut progress = Progress::with_expected_bytes(cursor, 50, 50);

    progress.write_all(b"Hello")?;
    assert_eq!(progress.writer_percentage(), Some(0.1));

    progress.get_mut().set_position(0);
    let mut buf = [0u8; 5];
    progress.read_exact(&mut buf)?;
    assert_eq!(progress.reader_percentage(), Some(0.1));

    Ok(())
}
```

## Examples

Each example in [`examples/`](https://github.com/spire-rs/countio/tree/main/examples) prints what it counts:

| Command | What it shows |
| --- | --- |
| `cargo run --example counter` | Counting reads and writes with `Counter`, and the `u128` total |
| `cargo run --example progress` | Expected totals and completion percentages with `Progress` |

## Contributing

See [CONTRIBUTING.md](https://github.com/spire-rs/countio/blob/main/CONTRIBUTING.md)
for development setup and guidelines, and
[CHANGELOG.md](https://github.com/spire-rs/countio/blob/main/CHANGELOG.md) for
release notes.

## License

MIT License, see [LICENSE](https://github.com/spire-rs/countio/blob/main/LICENSE).
