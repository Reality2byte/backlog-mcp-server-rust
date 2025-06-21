use crate::{
    requests::{GetWikiCountParams, GetWikiListParams},
    responses::{GetWikiCountResponse, GetWikiDetailResponse, GetWikiListResponse},
};
use backlog_api_core::Result;
use backlog_core::identifier::{Identifier, WikiId};
use client::Client;

pub struct WikiApi(Client);

impl WikiApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get wiki page count
    /// Corresponds to `GET /api/v2/wikis/count`.
    pub async fn get_wiki_count(&self, params: GetWikiCountParams) -> Result<GetWikiCountResponse> {
        let query_params: Vec<(String, String)> = params.into();
        self.0
            .get_with_params("/api/v2/wikis/count", &query_params)
            .await
    }

    /// Get wiki page details
    /// Corresponds to `GET /api/v2/wikis/:wikiId`.
    pub async fn get_wiki_detail(
        &self,
        wiki_id: impl Into<WikiId>,
    ) -> Result<GetWikiDetailResponse> {
        let wiki_id = wiki_id.into();
        self.0
            .get(&format!("/api/v2/wikis/{}", wiki_id.value()))
            .await
    }

    /// Get wiki page list
    /// Corresponds to `GET /api/v2/wikis`.
    pub async fn get_wiki_list(&self, params: GetWikiListParams) -> Result<GetWikiListResponse> {
        let query_params: Vec<(String, String)> = params.into();
        self.0.get_with_params("/api/v2/wikis", &query_params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{Wiki, WikiAttachment, WikiCount, WikiDetail, WikiTag},
        requests::{GetWikiCountParamsBuilder, GetWikiListParamsBuilder},
    };
    use backlog_core::identifier::SharedFileId;
    use backlog_core::{
        Language, ProjectKey, Role, Star, User,
        identifier::{Identifier, ProjectId, StarId, UserId, WikiAttachmentId, WikiId, WikiTagId},
    };
    use backlog_file::models::{FileContent, SharedFile};
    use chrono::{TimeZone, Utc};
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_mock_user(id: u32, name: &str) -> User {
        User {
            id: UserId::new(id),
            user_id: Some(name.to_string()),
            name: name.to_string(),
            role_type: Role::User,
            lang: Some(Language::Japanese),
            mail_address: format!("{}@example.com", name),
            last_login_time: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        }
    }

    fn create_mock_wiki(
        id: u32,
        project_id: u32,
        name: &str,
        user_id: u32,
        user_name: &str,
    ) -> Wiki {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        Wiki {
            id: WikiId::new(id),
            project_id: ProjectId::new(project_id),
            name: name.to_string(),
            tags: vec![WikiTag {
                id: WikiTagId::new(1),
                name: "proceedings".to_string(),
            }],
            created_user: create_mock_user(user_id, user_name),
            created: created_time,
            updated_user: create_mock_user(user_id, user_name),
            updated: created_time,
        }
    }

    #[tokio::test]
    async fn test_get_wiki_list_empty_params_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_wikis = vec![
            create_mock_wiki(112, 103, "Home", 1, "john"),
            create_mock_wiki(113, 103, "Documentation", 2, "alice"),
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_ok());
        let wikis = result.unwrap();
        assert_eq!(wikis.len(), 2);
        assert_eq!(wikis[0].name, "Home");
        assert_eq!(wikis[1].name, "Documentation");
    }

    #[tokio::test]
    async fn test_get_wiki_list_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_wikis = vec![create_mock_wiki(112, 123, "Home", 1, "john")];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .and(query_param("projectIdOrKey", "MYPROJECT"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default()
            .project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap())
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_ok());
        let wikis = result.unwrap();
        assert_eq!(wikis.len(), 1);
        assert_eq!(wikis[0].name, "Home");
    }

    #[tokio::test]
    async fn test_get_wiki_list_with_keyword() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_wikis = vec![create_mock_wiki(113, 103, "Documentation", 2, "alice")];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .and(query_param("keyword", "doc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default()
            .keyword("doc")
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_ok());
        let wikis = result.unwrap();
        assert_eq!(wikis.len(), 1);
        assert_eq!(wikis[0].name, "Documentation");
    }

    #[tokio::test]
    async fn test_get_wiki_list_with_both_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_wikis = vec![create_mock_wiki(113, 123, "Documentation", 2, "alice")];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .and(query_param("projectIdOrKey", "MYPROJECT"))
            .and(query_param("keyword", "doc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default()
            .project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap())
            .keyword("doc")
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_ok());
        let wikis = result.unwrap();
        assert_eq!(wikis.len(), 1);
        assert_eq!(wikis[0].name, "Documentation");
    }

    #[tokio::test]
    async fn test_get_wiki_list_empty_result() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_wikis: Vec<Wiki> = vec![];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_wiki_list_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_list_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis"))
            .and(query_param("projectIdOrKey", "NONEXISTENT"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = GetWikiListParamsBuilder::default()
            .project_id_or_key("NONEXISTENT".parse::<ProjectKey>().unwrap())
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_list(params).await;
        assert!(result.is_err());
    }

    // Tests for get_wiki_count
    #[tokio::test]
    async fn test_get_wiki_count_without_project() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_count = WikiCount { count: 42 };

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/count"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
            .mount(&mock_server)
            .await;

        let params = GetWikiCountParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_count(params).await;
        assert!(result.is_ok());
        let count = result.unwrap();
        assert_eq!(count.count, 42);
    }

    #[tokio::test]
    async fn test_get_wiki_count_with_project() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_count = WikiCount { count: 15 };

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/count"))
            .and(query_param("projectIdOrKey", "MYPROJECT"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
            .mount(&mock_server)
            .await;

        let params = GetWikiCountParamsBuilder::default()
            .project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap())
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_count(params).await;
        assert!(result.is_ok());
        let count = result.unwrap();
        assert_eq!(count.count, 15);
    }

    #[tokio::test]
    async fn test_get_wiki_count_zero_count() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_count = WikiCount { count: 0 };

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/count"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
            .mount(&mock_server)
            .await;

        let params = GetWikiCountParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_count(params).await;
        assert!(result.is_ok());
        let count = result.unwrap();
        assert_eq!(count.count, 0);
    }

    #[tokio::test]
    async fn test_get_wiki_count_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/count"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = GetWikiCountParamsBuilder::default().build().unwrap();
        let result = wiki_api.get_wiki_count(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_count_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/count"))
            .and(query_param("projectIdOrKey", "NONEXISTENT"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = GetWikiCountParamsBuilder::default()
            .project_id_or_key("NONEXISTENT".parse::<ProjectKey>().unwrap())
            .build()
            .unwrap();
        let result = wiki_api.get_wiki_count(params).await;
        assert!(result.is_err());
    }

    // Helper function for creating mock WikiDetail
    fn create_mock_wiki_detail(id: u32, project_id: u32, name: &str) -> WikiDetail {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
        let updated_time = Utc.with_ymd_and_hms(2024, 1, 2, 15, 30, 0).unwrap();

        WikiDetail {
            id: WikiId::new(id),
            project_id: ProjectId::new(project_id),
            name: name.to_string(),
            content: format!("# {}\n\nThis is the content of {}.", name, name),
            tags: vec![WikiTag {
                id: WikiTagId::new(1),
                name: "documentation".to_string(),
            }],
            attachments: vec![WikiAttachment {
                id: WikiAttachmentId::new(100),
                name: "attachment.pdf".to_string(),
                size: 1024,
                created_user: create_mock_user(1, "john"),
                created: created_time,
            }],
            shared_files: vec![SharedFile {
                id: SharedFileId::new(200),
                project_id: ProjectId::new(project_id),
                dir: "/docs".to_string(),
                name: "shared.png".to_string(),
                created_user: create_mock_user(2, "alice"),
                created: created_time,
                updated_user: None,
                updated: None,
                content: FileContent::File { size: 2048 },
            }],
            stars: vec![Star {
                id: StarId::new(300),
                comment: Some("Great documentation!".to_string()),
                url: format!("https://example.backlog.jp/view/PROJ-{}", id),
                presenter: create_mock_user(3, "bob"),
                created: created_time,
            }],
            created_user: create_mock_user(1, "john"),
            created: created_time,
            updated_user: create_mock_user(2, "alice"),
            updated: updated_time,
        }
    }

    // Tests for get_wiki_detail
    #[tokio::test]
    async fn test_get_wiki_detail_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_detail = create_mock_wiki_detail(123, 456, "API Documentation");

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let result = wiki_api.get_wiki_detail(WikiId::new(123)).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.id.value(), 123);
        assert_eq!(detail.project_id.value(), 456);
        assert_eq!(detail.name, "API Documentation");
        assert!(detail.content.contains("API Documentation"));
        assert_eq!(detail.tags.len(), 1);
        assert_eq!(detail.attachments.len(), 1);
        assert_eq!(detail.shared_files.len(), 1);
        assert_eq!(detail.stars.len(), 1);
    }

    #[tokio::test]
    async fn test_get_wiki_detail_with_u32_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_detail = create_mock_wiki_detail(789, 101, "User Guide");

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/789"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        // Test using u32 directly (Into<WikiId> conversion)
        let result = wiki_api.get_wiki_detail(789u32).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.id.value(), 789);
        assert_eq!(detail.name, "User Guide");
    }

    #[tokio::test]
    async fn test_get_wiki_detail_minimal_response() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let minimal_detail = WikiDetail {
            id: WikiId::new(999),
            project_id: ProjectId::new(777),
            name: "Minimal Wiki".to_string(),
            content: "Simple content".to_string(),
            tags: vec![],
            attachments: vec![],
            shared_files: vec![],
            stars: vec![],
            created_user: create_mock_user(1, "creator"),
            created: Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap(),
            updated_user: create_mock_user(1, "creator"),
            updated: Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap(),
        };

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/999"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&minimal_detail))
            .mount(&mock_server)
            .await;

        let result = wiki_api.get_wiki_detail(WikiId::new(999)).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.id.value(), 999);
        assert_eq!(detail.name, "Minimal Wiki");
        assert!(detail.tags.is_empty());
        assert!(detail.attachments.is_empty());
        assert!(detail.shared_files.is_empty());
        assert!(detail.stars.is_empty());
    }

    #[tokio::test]
    async fn test_get_wiki_detail_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/404"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = wiki_api.get_wiki_detail(WikiId::new(404)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_detail_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/500"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = wiki_api.get_wiki_detail(WikiId::new(500)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_detail_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/403"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let result = wiki_api.get_wiki_detail(WikiId::new(403)).await;
        assert!(result.is_err());
    }
}
