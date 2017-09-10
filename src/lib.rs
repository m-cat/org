//! Crate for processing .org files.
//!
//! This does not yet completely capture the functionality of org-mode.
//! The following aspects of org-mode are currently supported:
//!
//! * Document fields such as TITLE and AUTHOR
//!
//! * Headings and subheadings
//!
//! * Content text

#![deny(missing_docs)]

pub mod org;
mod util;

pub use org::*;
