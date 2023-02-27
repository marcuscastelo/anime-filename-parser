use std::{
    error::Error,
    fmt::{self, Display},
};

use error_stack::{IntoReport, Report, Result, ResultExt};
use regex::{Match, Regex};

use crate::metadata::{EpisodeSpec, Metadata, PreMetadata};

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"^\[Ohys-Raws\]\s(?P<ShowTitle>.+?)\s(?:-\s(?P<Episode>\d+(?:\sEND)?))?\s?\((?P<Quality>.+?)\)(?:\s?v(?P<Version>[^\.]+))?(?:\.(?P<Extension>.+))?$"#).unwrap();
}

pub struct OhysRawsParser;

#[derive(Debug)]
pub struct OhysRawsParserError;

impl Display for OhysRawsParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing filename with OhysRawsParser")
    }
}

impl Error for OhysRawsParserError {}

impl OhysRawsParser {
    pub fn parse_filename(
        &self,
        pre_metadata: PreMetadata,
    ) -> Result<Metadata, OhysRawsParserError> {
        let filename = pre_metadata.filename;
        let submitter = pre_metadata.submitter;

        let captures = REGEX
            .captures(&filename)
            .ok_or(OhysRawsParserError)
            .into_report()
            .attach_printable(format!("Filename does not match OhysRawsParser regex: \n\tActual: '{}'\n\tExpected: '[Ohys-Raws] <ShowTitle> - <Episode> (<Quality>) v<Version>.<FileExtension>'", filename))?;

        let show_title = captures
            .name("ShowTitle")
            .ok_or(OhysRawsParserError)
            .into_report()
            .attach_printable("ShowTitle not found in filename")
            .map(|m| m.as_str())?;

        let episode = captures.name("Episode").map_or(None, |m| m.as_str().into());
        let quality = captures.name("Quality").map_or(None, |m| m.as_str().into());
        let version = captures.name("Version").map_or(None, |m| m.as_str().into());
        let extension = captures
            .name("Extension")
            .map_or(None, |m| m.as_str().into());

        let mut metadata = Metadata::builder()
            .filename(String::default())
            .submitter(submitter)
            .title(show_title.to_owned())
            .episode(EpisodeSpec::Single(
                episode.map(|s| s.parse().unwrap()).unwrap(),
            ))
            .quality(quality.map(String::from))
            .version(version.map(String::from))
            .extension(extension.map(String::from))
            .build();

        metadata.filename = filename;

        Ok(metadata)
    }
}
