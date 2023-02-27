use derive_builder::Builder;
use derive_getters::Getters;
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub struct PreMetadata {
    filename: String,
    submitter: Option<String>,
}

#[derive(Debug, Clone)]
pub enum EpisodeSpec {
    Single(u32),
    Range { start: u32, end: u32 },
    Unspecified,
}

#[derive(Debug, Clone, Builder, Getters)]
pub struct Metadata {
    filename: String,
    submitter: Option<String>,
    title: String,
    episode: EpisodeSpec,
}

impl Metadata {
    pub fn new(pre_metadata: PreMetadata) -> MetadataBuilder {
        let mut builder = MetadataBuilder::default();
        builder
            .filename(pre_metadata.filename)
            .submitter(pre_metadata.submitter);
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename_keep_filename() {
        let filename = "test";
        let mut metadata_builder = Metadata::new(PreMetadata {
            filename: filename.to_string(),
            submitter: None,
        });

        let metadata = metadata_builder
            .title("test".to_string())
            .episode(EpisodeSpec::Unspecified)
            .build()
            .unwrap();
        assert_eq!(metadata.filename, filename);
    }

    #[test]
    fn test_parse_filename_submitter_is_correct() {
        let filename = "[test] test";
        let mut metadata_builder = Metadata::new(PreMetadata {
            filename: filename.to_string(),
            submitter: Some("test".to_string()),
        });

        let metadata = metadata_builder
            .title("test".to_string())
            .episode(EpisodeSpec::Unspecified)
            .build()
            .unwrap();
        assert_eq!(metadata.filename, filename);
    }
}
