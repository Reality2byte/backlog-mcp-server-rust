use crate::models::{HistoryOrder, WikiHistory};
use backlog_api_core::IntoRequest;
use backlog_core::identifier::WikiId;
use serde::Serialize;

/// Response type for getting wiki page history.
pub type GetWikiHistoryResponse = Vec<WikiHistory>;

/// Parameters for getting the update history of a wiki page.
///
/// Corresponds to `GET /api/v2/wikis/:wikiId/history`.
#[derive(Debug, Clone)]
pub struct GetWikiHistoryParams {
    pub wiki_id: WikiId,
    pub min_id: Option<u32>,
    pub max_id: Option<u32>,
    pub count: Option<u32>,
    pub order: Option<HistoryOrder>,
}

impl GetWikiHistoryParams {
    /// Create new parameters with the specified wiki ID.
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        }
    }

    /// Set the minimum ID for history entries.
    pub fn min_id(mut self, min_id: u32) -> Self {
        self.min_id = Some(min_id);
        self
    }

    /// Set the maximum ID for history entries.
    pub fn max_id(mut self, max_id: u32) -> Self {
        self.max_id = Some(max_id);
        self
    }

    /// Set the maximum number of history entries to retrieve (1-100).
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Set the sort order for history entries.
    pub fn order(mut self, order: HistoryOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl IntoRequest for GetWikiHistoryParams {
    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/history", self.wiki_id)
    }

    fn to_query(&self) -> impl Serialize {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            min_id: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_id: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            count: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<String>,
        }

        Query {
            min_id: self.min_id,
            max_id: self.max_id,
            count: self.count,
            order: self.order.as_ref().map(|o| match o {
                HistoryOrder::Asc => "asc".to_string(),
                HistoryOrder::Desc => "desc".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;
    use serde_json;

    #[test]
    fn test_get_wiki_history_params_new() {
        let params = GetWikiHistoryParams::new(WikiId::new(123));
        assert_eq!(params.wiki_id.value(), 123);
        assert!(params.min_id.is_none());
        assert!(params.max_id.is_none());
        assert!(params.count.is_none());
        assert!(params.order.is_none());
    }

    #[test]
    fn test_get_wiki_history_params_builders() {
        let params = GetWikiHistoryParams::new(WikiId::new(123))
            .min_id(100)
            .max_id(200)
            .count(50)
            .order(HistoryOrder::Asc);

        assert_eq!(params.wiki_id.value(), 123);
        assert_eq!(params.min_id, Some(100));
        assert_eq!(params.max_id, Some(200));
        assert_eq!(params.count, Some(50));
        assert_eq!(params.order, Some(HistoryOrder::Asc));
    }

    #[test]
    fn test_get_wiki_history_params_path() {
        let params = GetWikiHistoryParams::new(WikiId::new(456));
        assert_eq!(params.path(), "/api/v2/wikis/456/history");
    }

    #[test]
    fn test_get_wiki_history_params_to_query_empty() {
        let params = GetWikiHistoryParams::new(WikiId::new(123));
        let query = params.to_query();
        let json = serde_json::to_string(&query).unwrap();

        // Empty parameters should serialize to empty object
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_get_wiki_history_params_to_query_full() {
        let params = GetWikiHistoryParams::new(WikiId::new(123))
            .min_id(100)
            .max_id(200)
            .count(50)
            .order(HistoryOrder::Asc);

        let query = params.to_query();
        let json = serde_json::to_string(&query).unwrap();

        assert!(json.contains("\"minId\":100"));
        assert!(json.contains("\"maxId\":200"));
        assert!(json.contains("\"count\":50"));
        assert!(json.contains("\"order\":\"asc\""));
    }

    #[test]
    fn test_get_wiki_history_params_to_query_desc_order() {
        let params = GetWikiHistoryParams::new(WikiId::new(123)).order(HistoryOrder::Desc);

        let query = params.to_query();
        let json = serde_json::to_string(&query).unwrap();

        assert!(json.contains("\"order\":\"desc\""));
    }

    #[test]
    fn test_get_wiki_history_params_with_u32_id() {
        let params = GetWikiHistoryParams::new(789u32);
        assert_eq!(params.wiki_id.value(), 789);
        assert_eq!(params.path(), "/api/v2/wikis/789/history");
    }
}
