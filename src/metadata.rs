use derive_more::Constructor;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct PreMetadata {
    pub filename: String,
    pub submitter: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum EpisodeSpec {
    Single(u32),

    Range {
        start: u32,
        end: u32,
    },

    #[default]
    Unspecified,
}

#[derive(Debug, Clone, TypedBuilder, Default)]
pub struct Metadata {
    pub filename: String,
    pub submitter: Option<String>,
    pub title: String,
    pub episode: EpisodeSpec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_metadata() {
        let pre_metadata = PreMetadata::builder()
            .filename("test.mp4".to_string())
            .submitter("test".to_string().into())
            .build();

        assert_eq!(pre_metadata.filename, "test.mp4");
        assert_eq!(pre_metadata.submitter, Some("test".to_string()));
    }

    #[test]
    fn test_metadata() {
        let metadata = Metadata::builder()
            .filename("test.mp4".to_string())
            .submitter("test".to_string().into())
            .title("test".to_string())
            .episode(EpisodeSpec::Single(1))
            .build();

        assert_eq!(metadata.filename, "test.mp4");
        assert_eq!(metadata.submitter, Some("test".to_owned()));
        assert_eq!(metadata.title, "test");
        assert_eq!(metadata.episode, EpisodeSpec::Single(1));
    }
}
