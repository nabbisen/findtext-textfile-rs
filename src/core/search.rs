use std::{fmt, io};

use super::file::filepath_content;

const TRIM_PRESERVED_CHARS_LEN: usize = 7;

/// search result
#[derive(Debug)]
pub struct TextFileSearchResult {
    pub charset: String,
    pub matched: Vec<MatchedPart>,
}

/// search result
#[derive(Debug, PartialEq)]
pub struct MatchedPart {
    pub line_num: usize,
    pub col_pos: usize,
    pub text: String,
}

impl fmt::Display for MatchedPart {
    /// format for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}): {}", self.line_num, self.col_pos, self.text)
    }
}

/// trimmed matched text
#[derive(Debug)]
struct TrimmedMatchedText {
    matched_start_pos: usize,
    matched_end_pos: usize,
    text: String,
}

/// search main handler
pub fn handle(keyword: &str, filepath: &str) -> Result<TextFileSearchResult, io::Error> {
    let mut ret: Vec<MatchedPart> = vec![];

    let content = filepath_content(filepath)?;

    for (line_num, line) in content.text.lines().enumerate() {
        let mut col_pos = 0;
        loop {
            let matched = match trimmed_matched_text(line, keyword, col_pos) {
                Some(matched) => matched,
                None => break,
            };

            let found = MatchedPart {
                line_num: line_num + 1,
                col_pos: matched.matched_start_pos + 1,
                text: matched.text,
            };
            ret.push(found);

            col_pos = matched.matched_end_pos;
        }
    }

    Ok(TextFileSearchResult {
        charset: content.charset,
        matched: ret,
    })
}

/// optimize search result by limiting line text into keyword and the surroundings only
fn trimmed_matched_text(
    input: &str,
    keyword: &str,
    start_pos: usize,
) -> Option<TrimmedMatchedText> {
    if input.len() < start_pos {
        return None;
    }

    let pos = match input[start_pos..].find(keyword) {
        Some(pos) => start_pos + pos,
        None => return None,
    };

    let start = if TRIM_PRESERVED_CHARS_LEN <= pos {
        pos - TRIM_PRESERVED_CHARS_LEN
    } else {
        0
    };
    let end = if pos + keyword.len() + TRIM_PRESERVED_CHARS_LEN <= input.len() {
        pos + keyword.len() + TRIM_PRESERVED_CHARS_LEN
    } else {
        input.len()
    };

    Some(TrimmedMatchedText {
        matched_start_pos: pos,
        matched_end_pos: pos + keyword.len(),
        text: input[start..end].to_owned(),
    })
}
