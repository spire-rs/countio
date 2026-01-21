//! Counter usage examples
//!
//! This example demonstrates the basic functionality of the `Counter` wrapper
//! for tracking bytes read and written through I/O operations.

use std::io::{Read, Write};

use countio::Counter;

fn main() -> std::io::Result<()> {
    println!("=== Counter Examples ===\n");

    // Example 1: Reading with Counter
    reading_example()?;

    // Example 2: Writing with Counter
    writing_example()?;

    // Example 3: Large numbers (u128 total)
    large_numbers_example()?;

    Ok(())
}

/// Example: Track bytes read from a data source
fn reading_example() -> std::io::Result<()> {
    println!("Example: Reading with Counter");

    let data = b"Hello, World! This is test data.";
    let mut reader = Counter::new(&data[..]);

    let mut buffer = [0u8; 10];
    let bytes_read = reader.read(&mut buffer)?;

    println!("  Read: {} bytes", bytes_read);
    println!("  Total read: {}", reader.reader_bytes());
    println!("  Total (u128): {}\n", reader.total_bytes());

    Ok(())
}

/// Example: Track bytes written to a destination
fn writing_example() -> std::io::Result<()> {
    println!("Example: Writing with Counter");

    let mut writer = Counter::new(Vec::new());

    writer.write_all(b"Hello, ")?;
    writer.write_all(b"World!")?;

    println!("  Written: {} bytes", writer.writer_bytes());
    println!("  Total (u128): {}", writer.total_bytes());

    let data = writer.into_inner();
    println!("  Data: {:?}\n", String::from_utf8(data).unwrap());

    Ok(())
}

/// Example: Demonstrate u128 support for very large counts
fn large_numbers_example() -> std::io::Result<()> {
    println!("Example: Large numbers (u128 support)");

    let large_count = usize::MAX / 2;
    let counter = Counter::with_bytes(Vec::<u8>::new(), large_count, large_count);

    println!("  Reader: {} bytes", counter.reader_bytes());
    println!("  Writer: {} bytes", counter.writer_bytes());
    println!("  Total (u128): {}", counter.total_bytes());
    println!("  No overflow: {}", counter.total_bytes() < u128::MAX);

    Ok(())
}
