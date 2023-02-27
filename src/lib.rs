#![warn(clippy::unwrap_used, clippy::expect_used)]

mod metadata;
mod parsers;

use crate::metadata::Metadata;

use std::fmt::Display;

use error_stack::{IntoReport, Report, Result, ResultExt};
use parsers::pre_parser::PreParser;

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing filename")
    }
}

impl std::error::Error for ParseError {}

pub fn parse_filename(filename: &str) -> Result<Metadata, ParseError> {
    let pre_metadata = PreParser::parse_filename(filename)
        .change_context(ParseError)
        .attach_printable(format!("Failed to pre-parse filename: {}", filename))?;

    let mut builder = Metadata::new(pre_metadata);
    let metadata = builder
        .title("test".to_owned())
        .episode(metadata::EpisodeSpec::Single(1))
        .build()
        .into_report()
        .change_context(ParseError)
        .attach_printable(format!("Failed to build metadata from pre-metadata"))?;

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename_keep_filename() {
        let filename = "test";
        let metadata = parse_filename(filename).unwrap();
        assert_eq!(metadata.filename(), filename);
    }

    #[test]
    fn test_parse_filename_submitter_is_correct() {
        let filename = "[test] test";
        let metadata = parse_filename(filename).unwrap();
        assert_eq!(metadata.filename(), filename);
    }
}
