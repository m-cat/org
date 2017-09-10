//! Module for processing .org files

use std::io;
use util::{read_file_vec, write_file_vec};

/// Org data structure
pub struct Org {
    depth: usize,
    heading: String,
    content: Vec<String>,
    subtrees: Vec<Org>,
    closed: bool,
}

impl Org {
    /// Get the full heading for the subtree, including beginning asterisks
    fn full_heading(&self) -> String {
        if self.depth == 0 {
            String::from("")
        } else {
            format!("{} {}", "*".repeat(self.depth), self.heading)
        }
    }
}

/// Given a file path, return Org struct
pub fn process_org(fname: &str) -> io::Result<Org> {
    let file_contents: Vec<String> = match read_file_vec(fname) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let mut org = Org {
        depth: 0,
        heading: String::new(),
        content: Vec::new(),
        subtrees: Vec::new(),
        closed: false,
    };

    process_subtree(&mut org, &file_contents, 0);

    Ok(org)
}

/// Recursively process subtrees, converting from strings to Org struct representation
fn process_subtree(org: &mut Org, contents: &[String], index: usize) -> usize {
    let depth = org.depth;
    let mut i = index;

    while i < contents.len() {
        let line = &contents[i];
        let (heading, level) = get_heading(line);

        if level == 0 {
            // Found content
            org.content.push(line.clone());
            i += 1;
        } else if level <= depth {
            // Return if new heading found at equal or lesser depth
            return i;
        } else {
            // Start processing a new subtree
            let mut subtree = Org {
                depth: depth + 1,
                heading: heading,
                content: Vec::new(),
                subtrees: Vec::new(),
                closed: false,
            };
            i = process_subtree(&mut subtree, contents, i + 1);
            org.subtrees.push(subtree);
        }
    }

    // Return the index we stopped at so the caller can continue processing at this location
    i
}

/// Get the heading title and level from a line
fn get_heading(line: &str) -> (String, usize) {
    let mut level = 0;

    // Get the heading level
    for c in line.chars() {
        if c == '*' {
            level += 1;
        } else {
            break;
        }
    }

    // Extract the heading title
    let heading = if level < line.chars().count() {
        String::from(&line[level..])
    } else {
        String::new()
    };

    (heading.trim().to_string(), level)
}

/// Write an Org struct to a file
pub fn write_org(fname: &str, org: &Org) -> io::Result<()> {
    let mut contents: Vec<String> = Vec::new();

    write_subtree(org, &mut contents);

    write_file_vec(fname, &contents)
}

/// Push an Org struct to a Vec of Strings
fn write_subtree(org: &Org, mut contents: &mut Vec<String>) {
    if org.depth > 0 {
        contents.push(org.full_heading());
    }

    for line in &org.content {
        contents.push(line.clone());
    }

    for subtree in &org.subtrees {
        write_subtree(subtree, &mut contents);
    }
}

#[cfg(test)]
mod tests {
    use org::*;

    #[test]
    fn test_get_heading() {
        assert_eq!(get_heading(""), (String::from(""), 0));
        assert_eq!(get_heading("Test"), (String::from("Test"), 0));
        assert_eq!(get_heading("* Test"), (String::from("Test"), 1));
        assert_eq!(get_heading("***Test"), (String::from("Test"), 3));
        assert_eq!(get_heading("*****"), (String::new(), 5));
    }
}
