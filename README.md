## countio

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

**Check out other `spire` projects [here](https://github.com/spire-rs).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/spire-rs/countio/build.yml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/spire-rs/countio/actions/workflows/build.yml
[crates-badge]: https://img.shields.io/crates/v/countio.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/countio
[docs-badge]: https://img.shields.io/docsrs/countio?logo=Docs.rs&style=flat-square
[docs-url]: https://docs.rs/countio

The wrapper struct to enable byte counting for `std::io::{Read, Write, Seek}`
and its asynchronous variants from `futures` and `tokio` crates.

Supports bidirectional I/O objects (like `TcpStream` or `Cursor`) that implement
both read and write traits, tracking read and write byte counts independently.

### Features

- `std` to enable `std::io::{Read, BufRead, Write, Seek}`. **Enabled by default**.
- `futures` to enable `futures_io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek}`.
- `tokio` to enable `tokio::io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek}`.
- `full` to enable all features (`std`, `futures`, and `tokio`).

### Examples

- `std::io::Read`:

```rust
use std::io::{BufRead, BufReader, Result};
use countio::Counter;

fn main() -> Result<()> {
    let reader = "Hello World!".as_bytes();
    let reader = BufReader::new(reader);
    let mut reader = Counter::new(reader);

    let mut buf = String::new();
    let len = reader.read_line(&mut buf)?;

    assert_eq!(len, reader.reader_bytes());
    Ok(())
}
```

- `std::io::Write`:

```rust
use std::io::{BufWriter, Write, Result};
use countio::Counter;

fn main() -> Result<()> {
    let writer = Vec::new();
    let writer = BufWriter::new(writer);
    let mut writer = Counter::new(writer);

    let buf = "Hello World!".as_bytes();
    let len = writer.write(buf)?;
    writer.flush()?;

    assert_eq!(len, writer.writer_bytes());
    Ok(())
}
```

- Bidirectional I/O with `Progress`:

```rust
use std::io::{Cursor, Read, Write, Result};
use countio::Progress;

fn main() -> Result<()> {
    let mut data = vec![0u8; 100];
    let cursor = Cursor::new(&mut data);
    let mut progress = Progress::with_expected_bytes(cursor, 50, 50);

    // Write some data
    progress.write_all(b"Hello")?;
    assert_eq!(progress.writer_bytes(), 5);
    assert_eq!(progress.writer_percentage(), Some(0.1));

    // Seek back and read
    progress.get_mut().set_position(0);
    let mut buf = [0u8; 5];
    progress.read_exact(&mut buf)?;
    assert_eq!(progress.reader_bytes(), 5);
    assert_eq!(progress.reader_percentage(), Some(0.1));

    Ok(())
}
```

### Other crates

- [SOF3/count-write](https://crates.io/crates/count-write)
