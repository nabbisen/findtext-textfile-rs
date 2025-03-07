// todo: make private
pub mod core;

use core::search::{handle, TextFileSearchResult};
use std::io;

/// entry point
pub fn search(keyword: &str, filepath: &str) -> Result<TextFileSearchResult, io::Error> {
    handle(keyword, filepath)
}
