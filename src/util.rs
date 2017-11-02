//! Utility functions.

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

/// Reads a file and returns its contents as lines in Vec<String>.
/// Each string returned will not have an ending newline.
pub fn read_file_vec(fname: &str) -> io::Result<Vec<String>> {
    // Open a file in read-only mode
    let file = File::open(&fname)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();

    // Add each line to the output Vec
    for line in reader.lines() {
        match line {
            Ok(line) => vec.push(line),
            Err(e) => return Err(e),
        }
    }

    Ok(vec)
}

/// Writes a `contents` to a file with a given path.
pub fn write_file_vec(fname: &str, contents: &[String]) -> io::Result<()> {
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&fname)?;
    let newline = b"\n";

    // Write each string to `file`, returns `io::Result<()>`
    for line in contents {
        file.write_all(line.as_bytes())?;
        file.write_all(newline)?;
    }

    Ok(())
}
