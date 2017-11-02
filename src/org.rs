//! Primary module containing outside-facing API.

use std::fmt;
use std::io;
use util::{read_file_vec, write_file_vec};

/// Org data structure.
#[derive(Clone, Debug, PartialEq)]
pub struct Org {
    /// The depth of the subtree.
    /// Depth is equal to the number of asterisks in the header.
    /// The root subtree therefore has a depth of 0.
    depth: usize,
    /// The heading of the subtree.
    /// This heading does not include beginning asterisks.
    heading: String,
    /// The content of the subtree.
    content: Vec<String>,
    /// The subtrees of the subtree.
    subtrees: Vec<Org>,
}

impl Org {
    /// Returns an empty root-level Org struct.
    pub fn new() -> Org {
        Org {
            depth: 0,
            heading: String::new(),
            content: Vec::new(),
            subtrees: Vec::new(),
        }
    }

    /// Reads an Org struct from a given file path.
    pub fn from_file(fname: &str) -> io::Result<Org> {
        let file_contents: Vec<String> = match read_file_vec(fname) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Self::from_vec(&file_contents)
    }

    /// Reads an Org struct from `contents`.
    pub fn from_vec(contents: &[String]) -> io::Result<Org> {
        let mut org = Default::default();

        process_subtree(&mut org, contents, 0);

        Ok(org)
    }

    /// Writes an Org struct to a file.
    pub fn to_file(&self, fname: &str) -> io::Result<()> {
        let contents = self.to_vec();

        write_file_vec(fname, &contents)
    }

    /// Writes an Org struct to a Vec of Strings.
    pub fn to_vec(&self) -> Vec<String> {
        let mut contents = Vec::new();

        if self.depth > 0 {
            contents.push(self.full_heading());
        }

        for line in &self.content {
            contents.push(line.clone());
        }

        for subtree in &self.subtrees {
            contents.append(&mut subtree.to_vec());
        }

        contents
    }

    /// Returns the depth of the subtree.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the heading of the subtree.
    pub fn heading(&self) -> &str {
        &self.heading
    }

    /// Sets the heading of the subtree.
    pub fn set_heading(&mut self, heading: &str) {
        self.heading = String::from(heading)
    }

    /// Gets the full heading for the subtree, including beginning asterisks.
    pub fn full_heading(&self) -> String {
        if self.depth == 0 {
            String::new()
        } else {
            format!("{} {}", "*".repeat(self.depth), self.heading)
        }
    }

    /// Returns a reference to the content of the subtree.
    pub fn content_as_ref(&self) -> &Vec<String> {
        &self.content
    }

    /// Returns a mutable reference to the subtrees of the subtree.
    pub fn content_as_mut(&mut self) -> &mut Vec<String> {
        &mut self.content
    }

    /// Returns a reference to the subtrees of the subtree.
    pub fn subtrees_as_ref(&self) -> &Vec<Org> {
        &self.subtrees
    }

    /// Returns a mutable reference to the subtrees of the subtree.
    pub fn subtrees_as_mut(&mut self) -> &mut Vec<Org> {
        &mut self.subtrees
    }
}

impl Default for Org {
    fn default() -> Org {
        Org::new()
    }
}

impl fmt::Display for Org {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let contents = self.to_vec();
        let len = contents.len();
        let mut res = String::new();

        for (i, line) in contents.into_iter().enumerate() {
            res += &line;

            if i < len {
                res += "\n";
            }
        }

        write!(f, "{}", res)
    }
}

// Recursively processes subtrees, converting from strings to Org structs.
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
            };
            i = process_subtree(&mut subtree, contents, i + 1);
            org.subtrees.push(subtree);
        }
    }

    // Return the index we stopped at so the caller can continue processing at this location
    i
}

// Gets the heading title and level from a line.
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

#[cfg(test)]
mod tests {
    use org::get_heading;

    // Tests `get_heading`.
    #[test]
    fn test_get_heading() {
        assert_eq!(get_heading(""), (String::from(""), 0));
        assert_eq!(get_heading("Test"), (String::from("Test"), 0));
        assert_eq!(get_heading("* Test"), (String::from("Test"), 1));
        assert_eq!(get_heading("***Test"), (String::from("Test"), 3));
        assert_eq!(get_heading("*****"), (String::new(), 5));
    }
}
