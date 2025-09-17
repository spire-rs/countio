//! Progress tracking examples
//!
//! This example demonstrates the `Progress` wrapper for tracking
//! percentage completion and processing statistics.

use std::io::{Read, Write};

use countio::Progress;

fn main() -> std::io::Result<()> {
    println!("=== Progress Examples ===\n");

    // Example 1: Progress with known total
    known_total_example()?;

    // Example 2: Progress with unknown total
    unknown_total_example()?;

    // Example 3: Reading progress
    reading_example()?;

    Ok(())
}

/// Example: Track progress when total size is known
fn known_total_example() -> std::io::Result<()> {
    println!("Example: Progress with known total");

    let mut progress = Progress::with_total(Vec::new(), 100);

    progress.write_all(b"Hello")?;
    println!(
        "  After 'Hello': {:.1}%",
        progress.percentage().unwrap_or(0.0) * 100.0
    );

    progress.write_all(b", World!")?;
    println!(
        "  After ', World!': {:.1}%",
        progress.percentage().unwrap_or(0.0) * 100.0
    );

    println!("  Total processed: {} bytes\n", progress.bytes_processed());
    Ok(())
}

/// Example: Track progress when total size is unknown
fn unknown_total_example() -> std::io::Result<()> {
    println!("Example: Progress with unknown total");

    let mut progress = Progress::new(Vec::new());

    progress.write_all(b"Processing data")?;
    println!("  Processed: {} bytes", progress.bytes_processed());
    println!("  Percentage: {:?}", progress.percentage());

    // Discover total size during processing
    progress.set_total_expected(Some(50));
    println!("  After setting total to 50:");
    println!(
        "  Percentage: {:.1}%\n",
        progress.percentage().unwrap_or(0.0) * 100.0
    );

    Ok(())
}

/// Example: Track reading progress
fn reading_example() -> std::io::Result<()> {
    println!("Example: Reading progress");

    let data = b"This is test data for reading progress tracking.";
    let total_size = data.len() as u64;
    let mut progress = Progress::with_total(&data[..], total_size);

    let mut buffer = [0u8; 10];
    while progress.bytes_read() < data.len() {
        let bytes_read = progress.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let pct = progress.percentage().unwrap_or(0.0) * 100.0;
        println!(
            "  Read: {}/{} bytes ({:.1}%)",
            progress.bytes_read(),
            total_size,
            pct
        );
    }

    println!("  Reading complete: {} bytes\n", progress.bytes_read());
    Ok(())
}
