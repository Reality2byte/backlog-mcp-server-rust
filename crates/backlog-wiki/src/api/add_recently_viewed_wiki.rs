use crate::models::Wiki;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WikiId;
use serde::Serialize;

pub type AddRecentlyViewedWikiResponse = Wiki;

/// Parameters for adding a recently viewed wiki
///
/// Corresponds to `POST /api/v2/users/myself/recentlyViewedWikis`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddRecentlyViewedWikiParams {
    /// Wiki ID
    pub wiki_id: WikiId,
}

#[cfg(feature = "writable")]
impl From<&AddRecentlyViewedWikiParams> for Vec<(String, String)> {
    fn from(params: &AddRecentlyViewedWikiParams) -> Self {
        vec![("wikiId".to_string(), params.wiki_id.to_string())]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddRecentlyViewedWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedWikis".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;

    #[test]
    fn test_params_with_wiki_id() {
        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(12345),
        };

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0].0, "wikiId");
        assert_eq!(form_params[0].1, "12345");
    }

    #[test]
    fn test_path() {
        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(1),
        };
        assert_eq!(params.path(), "/api/v2/users/myself/recentlyViewedWikis");
    }

    #[test]
    fn test_method() {
        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(1),
        };
        assert_eq!(params.method(), HttpMethod::Post);
    }
}
