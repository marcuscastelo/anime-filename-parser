#![warn(clippy::unwrap_used, clippy::expect_used)]

mod metadata;
mod parsers;

use crate::metadata::Metadata;
use error_stack::{IntoReport, Report, Result, ResultExt};
use metadata::PreMetadata;
use parsers::pre_parser::PreParser;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing filename")
    }
}

impl Error for ParseError {}

pub fn parse_filename(filename: String) -> Result<Metadata, ParseError> {
    let pre_metadata = PreParser::parse_filename(filename);

    let metadata = match pre_metadata {
        PreMetadata {
            filename,
            submitter,
        } => Metadata::builder()
            .filename(filename)
            .submitter(submitter)
            .title("aaaa".to_owned())
            .episode(metadata::EpisodeSpec::Single(1))
            .build(),
    };

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
