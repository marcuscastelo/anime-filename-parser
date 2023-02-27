use error_stack::{IntoReport, Report, Result, ResultExt};
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

impl PreParser {
    pub fn parse_filename(filename: String) -> Result<PreMetadata, PreParseError> {
        let submitter = PreParser::get_submitter(filename.as_str())?;
        Ok(PreMetadata::builder()
            .filename(filename)
            .submitter(submitter)
            .build())
    }

    pub fn get_submitter(filename: &str) -> Result<Option<String>, PreParseError> {
        const SUBMITTER_REGEX: &str = r"^\[([^]]+)\]|\[([^\]]+)\]";

        let re = regex::Regex::new(SUBMITTER_REGEX)
            .into_report()
            .attach(format!("Failed to compile regex: {}", SUBMITTER_REGEX))
            .change_context(PreParseError)?;

        let caps = match re.captures(filename) {
            Some(caps) => caps,
            None => return Ok(None),
        };

        let submitter = caps.get(1).or_else(|| caps.get(2)).ok_or(
            Report::new(PreParseError)
                .attach_printable(format!("This should never happen. None of the regex groups participated in the match. Filename: {}", filename)),
        )?;

        let submitter = submitter.as_str();
        Ok(Some(submitter.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_submitter() -> Result<(), PreParseError> {
        let filename = "[test] test";
        let submitter = PreParser::get_submitter(filename)?;
        assert_eq!(submitter, Some("test".to_string()));
        Ok(())
    }

    #[test]
    fn test_get_submitter_no_submitter() -> Result<(), PreParseError> {
        let filename = "test";
        let submitter = PreParser::get_submitter(filename)?;
        assert_eq!(submitter, None);
        Ok(())
    }
}
