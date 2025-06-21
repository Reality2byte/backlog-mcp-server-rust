use serde::{Deserialize, Serialize};

/// Represents the count of wiki pages in a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiCount {
    /// The number of wiki pages
    pub count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiki_count_serialization() {
        let wiki_count = WikiCount { count: 42 };
        let json = serde_json::to_string(&wiki_count).unwrap();
        assert_eq!(json, r#"{"count":42}"#);
    }

    #[test]
    fn test_wiki_count_deserialization() {
        let json = r#"{"count":42}"#;
        let wiki_count: WikiCount = serde_json::from_str(json).unwrap();
        assert_eq!(wiki_count.count, 42);
    }
}
