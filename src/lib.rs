//! Crate for processing .org files
//!
//! This does not yet completely capture the functionality of org-mode.
//! The following aspects of org-mode are currently supported:
//! * Document fields such as TITLE and AUTHOR
//! * Headings and subheadings
//! * Content text

/// Primary module containing outside-facing API.
pub mod org;

pub use org::*;

mod util;
