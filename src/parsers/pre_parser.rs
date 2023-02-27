use std::{error::Error, fmt::Display};

use crate::metadata::PreMetadata;

pub struct PreParser;

#[derive(Debug)]
pub struct PreParseError;

impl Display for PreParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred while pre-parsing filename")
    }
}

impl Error for PreParseError {}

lazy_static::lazy_static! {
    static ref SUBMITTER_REGEX: regex::Regex = regex::Regex::new(r#"\[(?P<submitter>[^\]]+)\]"#).unwrap();
}

impl PreParser {
    pub fn parse_filename(filename: String) -> PreMetadata {
        let submitter = PreParser::get_submitter(filename.as_str()).map(String::from);

        PreMetadata::builder()
            .filename(filename)
            .submitter(submitter)
            .build()
    }

    pub fn get_submitter(filename: &str) -> Option<&str> {
        let caps = SUBMITTER_REGEX.captures(filename)?;
        let submitter = caps.name("submitter")?.as_str();
        Some(submitter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_submitter() {
        let filename = "[test] asdf";
        let submitter = PreParser::get_submitter(filename);
        assert_eq!(submitter, Some("test"));
    }

    #[test]
    fn test_get_submitter_no_submitter() {
        let filename = "test";
        let submitter = PreParser::get_submitter(filename);
        assert_eq!(submitter, None);
    }
}
