//! Utility functions.

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Reads a file and returns its contents as lines in Vec<String>.
/// Each string returned will not have an ending newline.
pub fn read_file_vec(fname: &str) -> io::Result<Vec<String>> {
    let path = Path::new(fname);
    // Open a file in read-only mode
    let file = File::open(&path)?;
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

/// Writes a Vec<String> to a file with a given path.
pub fn write_file_vec(fname: &str, contents: &[String]) -> io::Result<()> {
    let path = Path::new(fname);
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;
    let newline = b"\n";

    // Write each string to `file`, returns `io::Result<()>`
    for line in contents {
        file.write_all(line.as_bytes())?;
        file.write_all(newline)?;
    }

    Ok(())
}
