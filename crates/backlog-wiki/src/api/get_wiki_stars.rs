use backlog_api_core::IntoRequest;
use backlog_core::identifier::WikiId;
use backlog_domain_models::Star;
use serde::Serialize;

/// Parameters for getting wiki page stars.
///
/// # Example
/// ```no_run
/// # use backlog_wiki::api::GetWikiStarsParams;
/// # use backlog_core::identifier::WikiId;
/// let params = GetWikiStarsParams::new(12345u32);
/// ```
#[derive(Debug, Clone)]
pub struct GetWikiStarsParams {
    /// Wiki page ID
    pub wiki_id: WikiId,
}

impl GetWikiStarsParams {
    /// Creates a new instance with the specified wiki ID.
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
        }
    }
}

impl IntoRequest for GetWikiStarsParams {
    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/stars", self.wiki_id)
    }

    fn to_query(&self) -> impl Serialize {
        // No query parameters for this endpoint
        std::collections::HashMap::<String, String>::new()
    }
}

/// Type alias for the Get Wiki Stars API response.
pub type GetWikiStarsResponse = Vec<Star>;

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_get_wiki_stars_params_basic() {
        let params = GetWikiStarsParams::new(12345u32);

        assert_eq!(params.wiki_id.value(), 12345);
        assert_eq!(params.path(), "/api/v2/wikis/12345/stars");
    }

    #[test]
    fn test_get_wiki_stars_params_with_wiki_id() {
        let wiki_id = WikiId::new(54321);
        let params = GetWikiStarsParams::new(wiki_id);

        assert_eq!(params.wiki_id.value(), 54321);
        assert_eq!(params.path(), "/api/v2/wikis/54321/stars");
    }

    #[test]
    fn test_get_wiki_stars_params_path_generation() {
        // Test various wiki ID values
        let test_cases = vec![
            (1u32, "/api/v2/wikis/1/stars"),
            (999u32, "/api/v2/wikis/999/stars"),
            (1234567890u32, "/api/v2/wikis/1234567890/stars"),
        ];

        for (id, expected_path) in test_cases {
            let params = GetWikiStarsParams::new(id);
            assert_eq!(params.path(), expected_path);
        }
    }
}
