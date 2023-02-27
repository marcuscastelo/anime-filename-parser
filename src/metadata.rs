use derive_more::Constructor;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct PreMetadata {
    pub filename: String,
    pub submitter: Option<String>,
}

impl PreMetadata {
    pub fn new() -> PreMetadataBuilder<((), ())> {
        PreMetadata::builder()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EpisodeSpec {
    Single(u32),
    Range { start: u32, end: u32 },
    Unspecified,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Metadata {
    pub filename: String,
    pub submitter: Option<String>,
    pub title: String,
    pub episode: EpisodeSpec,
}

impl Metadata {
    pub fn new() -> MetadataBuilder<((), (), (), ())> {
        Metadata::builder()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_metadata() {
        let pre_metadata = PreMetadata::new()
            .filename("test.mp4".to_string())
            .submitter(Some("test".to_string()))
            .build();

        assert_eq!(pre_metadata.filename, "test.mp4");
        assert_eq!(pre_metadata.submitter, Some("test".to_string()));
    }

    #[test]
    fn test_metadata() {
        let metadata = Metadata::new()
            .filename("test.mp4".to_string())
            .submitter(Some("test".to_string()))
            .title("test".to_string())
            .episode(EpisodeSpec::Single(1))
            .build();

        assert_eq!(metadata.filename, "test.mp4");
        assert_eq!(metadata.submitter, Some("test".to_owned()));
        assert_eq!(metadata.title, "test");
        assert_eq!(metadata.episode, EpisodeSpec::Single(1));
    }
}
