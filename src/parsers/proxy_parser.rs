use crate::metadata::{PreMetadata, EpisodeSpec, Metadata};
use error_stack::{IntoReport, Report, Result, ResultExt};

use super::submitter::ohys_raws_parser::{OhysRawsParser, OhysRawsParserError};

pub struct ProxyParser;
impl ProxyParser {
    pub fn parse_filename(self, pre_metadata: PreMetadata) -> Result<Metadata, OhysRawsParserError> {
        OhysRawsParser.parse_filename(pre_metadata)
    }
}