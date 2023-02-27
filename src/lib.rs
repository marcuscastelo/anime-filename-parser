#![warn(clippy::unwrap_used, clippy::expect_used)]

mod metadata;
mod parsers;

use crate::metadata::Metadata;
use error_stack::{IntoReport, Report, Result, ResultExt};
use metadata::PreMetadata;
use parsers::pre_parser::PreParser;
use parsers::proxy_parser::ProxyParser;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ParseError {
    PreParser,
    ProxyParser,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::PreParser => write!(f, "Error pre-parsing filename"),
            ParseError::ProxyParser => write!(f, "Error parsing filename"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_filename(filename: String) -> Result<Metadata, ParseError> {
    let pre_metadata = PreParser::parse_filename(filename.clone());
    let metadata = ProxyParser
        .parse_filename(pre_metadata)
        .change_context(ParseError::ProxyParser)
        .attach_printable_lazy(|| format!("Error parsing filename: {}", filename))?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename_keep_filename() {
        let filename = "test";
        let metadata = parse_filename(filename.to_owned()).unwrap();
        assert_eq!(
            metadata.filename, filename,
            "Expected filename to be '{}', but got '{}'",
            filename, metadata.filename
        );
    }

    #[test]
    fn test_parse_filename_submitter_is_correct() {
        let filename = "[test] test".to_owned();
        let metadata = parse_filename(filename).unwrap();
        let expected_submitter = Some("test".to_owned());
        assert_eq!(
            metadata.submitter, expected_submitter,
            "Expected submitter to be '{:?}', but got '{:?}'",
            expected_submitter, metadata.submitter
        );
    }
}
