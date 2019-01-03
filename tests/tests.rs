//! Integration tests for [`org`] crate.

extern crate org;

use org::Org;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

const TEST_ORG_DIR: &str = "tests/files/";
const TEST_FILE_1: &str = "test1.org";
const TEST_EXT: &str = ".bk";

// Tests reading and writing org files.
#[test]
fn test_read_write_org() {
    let fname1 = format!("{}{}", TEST_ORG_DIR, TEST_FILE_1);
    let fname2 = format!("{}{}{}", TEST_ORG_DIR, TEST_FILE_1, TEST_EXT);

    let org = Org::from_file(&fname1).unwrap();

    org.to_file(&fname2).unwrap();

    // Test that when we process and write back an org file, we get the same result.
    assert!(files_equal(&fname1, &fname2).unwrap());

    fs::remove_file(fname2).unwrap();
}

// Tests the Display trait implementation for `Org`.
#[test]
fn test_display() {
    let fname = format!("{}{}", TEST_ORG_DIR, TEST_FILE_1);
    let result = read_file_str(&fname).unwrap();
    let org = Org::from_file(&fname).unwrap();

    assert_eq!(result, format!("{}", org));
}

// Reads a file and returns its contents in a string.
fn read_file_str(fname: &str) -> io::Result<String> {
    // Open a file in read-only mode
    let mut file = File::open(fname)?;

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;

    Ok(contents)
}

// Compares two files for equality.
fn files_equal(fname1: &str, fname2: &str) -> io::Result<bool> {
    let s1 = read_file_str(fname1)?;
    let s2 = read_file_str(fname2)?;

    Ok(s1 == s2)
}
