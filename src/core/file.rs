use std::fs::File;
use std::io::{self, Read};

use chardetng::EncodingDetector;

const UTF8_CHARSET: &str = "UTF-8";

/// content read from file
pub struct ReadContent {
    pub charset: String,
    pub text: String,
}

/// get content from file at file path
pub fn filepath_content(filepath: &str) -> Result<ReadContent, io::Error> {
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    match std::str::from_utf8(&buffer) {
        Ok(x) => {
            return Ok(ReadContent {
                charset: UTF8_CHARSET.to_owned(),
                text: x.to_owned(),
            })
        }
        Err(_) => (),
    }

    let mut detector = EncodingDetector::new();
    detector.feed(&buffer, true);
    let encoding = detector.guess(None, false);
    let (decoded, _, had_errors) = encoding.decode(&buffer);
    if had_errors {
        eprint!("not binary, not utf-8 text and not any other encoded text.")
    }
    Ok(ReadContent {
        charset: encoding.name().to_owned(),
        text: decoded.to_string(),
    })
}
