use crate::{
    requests::{
        DownloadWikiAttachmentParams, GetWikiAttachmentListParams, GetWikiCountParams,
        GetWikiDetailParams, GetWikiListParams,
    },
    responses::{
        GetWikiAttachmentListResponse, GetWikiCountResponse, GetWikiDetailResponse,
        GetWikiListResponse,
    },
};

#[cfg(feature = "writable")]
use crate::requests::{UpdateWikiParams, UpdateWikiRequestParams};
use backlog_api_core::Result;
use backlog_core::identifier::Identifier;
#[cfg(feature = "writable")]
use backlog_core::identifier::WikiId;
use client::Client;

pub struct WikiApi(Client);

impl WikiApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get wiki page count
    /// Corresponds to `GET /api/v2/wikis/count`.
    pub async fn get_wiki_count(&self, params: GetWikiCountParams) -> Result<GetWikiCountResponse> {
        self.0.execute(params).await
    }

    /// Get wiki page details
    /// Corresponds to `GET /api/v2/wikis/:wikiId`.
    pub async fn get_wiki_detail(
        &self,
        params: GetWikiDetailParams,
    ) -> Result<GetWikiDetailResponse> {
        self.0.execute(params).await
    }

    /// Get wiki page list
    /// Corresponds to `GET /api/v2/wikis`.
    pub async fn get_wiki_list(&self, params: GetWikiListParams) -> Result<GetWikiListResponse> {
        self.0.execute(params).await
    }

    /// Get wiki attachment list
    /// Corresponds to `GET /api/v2/wikis/:wikiId/attachments`.
    pub async fn get_wiki_attachment_list(
        &self,
        params: GetWikiAttachmentListParams,
    ) -> Result<GetWikiAttachmentListResponse> {
        self.0.execute(params).await
    }

    /// Download wiki attachment
    /// Corresponds to `GET /api/v2/wikis/:wikiId/attachments/:attachmentId`.
    pub async fn download_wiki_attachment(
        &self,
        params: DownloadWikiAttachmentParams,
    ) -> Result<client::DownloadedFile> {
        let path = format!(
            "/api/v2/wikis/{}/attachments/{}",
            params.wiki_id.value(),
            params.attachment_id.value()
        );
        self.0.download_file_raw(&path).await
    }

    /// Update wiki page
    /// Corresponds to `PATCH /api/v2/wikis/:wikiId`.
    #[cfg(feature = "writable")]
    pub async fn update_wiki(
        &self,
        wiki_id: impl Into<WikiId>,
        params: &UpdateWikiParams,
    ) -> Result<GetWikiDetailResponse> {
        let wiki_id = wiki_id.into();
        let params_vec: Vec<(String, String)> = params.into();
        self.0
            .patch(&format!("/api/v2/wikis/{}", wiki_id.value()), &params_vec)
            .await
    }

    /// Update wiki page using the new IntoRequest pattern
    /// Corresponds to `PATCH /api/v2/wikis/:wikiId`.
    #[cfg(feature = "writable")]
    pub async fn update_wiki_request(
        &self,
        params: UpdateWikiRequestParams,
    ) -> Result<GetWikiDetailResponse> {
        self.0.execute(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{Wiki, WikiAttachment, WikiCount, WikiDetail, WikiTag},
        requests::{GetWikiCountParamsBuilder, GetWikiListParamsBuilder},
    };

    #[cfg(feature = "writable")]
    use crate::requests::UpdateWikiParams;
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

        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(123)))
            .await;
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
        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(789u32))
            .await;
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

        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(999)))
            .await;
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

        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(404)))
            .await;
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

        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(500)))
            .await;
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

        let result = wiki_api
            .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(403)))
            .await;
        assert!(result.is_err());
    }

    // Helper function for creating mock WikiAttachment
    fn create_mock_wiki_attachment(
        id: u32,
        name: &str,
        size: u64,
        user_id: u32,
        user_name: &str,
    ) -> WikiAttachment {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        WikiAttachment {
            id: WikiAttachmentId::new(id),
            name: name.to_string(),
            size,
            created_user: create_mock_user(user_id, user_name),
            created: created_time,
        }
    }

    // Tests for get_wiki_attachment_list
    #[tokio::test]
    async fn test_get_wiki_attachment_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_attachments = vec![
            create_mock_wiki_attachment(1, "document.pdf", 1024, 1, "john"),
            create_mock_wiki_attachment(2, "image.png", 2048, 2, "alice"),
            create_mock_wiki_attachment(3, "spreadsheet.xlsx", 4096, 1, "john"),
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/123/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(123)))
            .await;
        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert_eq!(attachments.len(), 3);
        assert_eq!(attachments[0].name, "document.pdf");
        assert_eq!(attachments[0].size, 1024);
        assert_eq!(attachments[1].name, "image.png");
        assert_eq!(attachments[1].size, 2048);
        assert_eq!(attachments[2].name, "spreadsheet.xlsx");
        assert_eq!(attachments[2].size, 4096);
    }

    #[tokio::test]
    async fn test_get_wiki_attachment_list_with_u32_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_attachments =
            vec![create_mock_wiki_attachment(10, "readme.txt", 512, 3, "bob")];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/456/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        // Test using u32 directly (Into<WikiId> conversion)
        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(456u32))
            .await;
        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].name, "readme.txt");
        assert_eq!(attachments[0].size, 512);
    }

    #[tokio::test]
    async fn test_get_wiki_attachment_list_empty_result() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let expected_attachments: Vec<WikiAttachment> = vec![];

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/789/attachments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(789)))
            .await;
        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert!(attachments.is_empty());
    }

    #[tokio::test]
    async fn test_get_wiki_attachment_list_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/404/attachments"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(404)))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_attachment_list_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/500/attachments"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(500)))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_attachment_list_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/403/attachments"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(403)))
            .await;
        assert!(result.is_err());
    }

    // Tests for download_wiki_attachment
    #[tokio::test]
    async fn test_download_wiki_attachment_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let attachment_content = "This is a test attachment content.";

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/123/attachments/456"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(attachment_content)
                    .insert_header("Content-Type", "application/octet-stream")
                    .insert_header("Content-Disposition", "attachment; filename=\"test.txt\""),
            )
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(123),
                WikiAttachmentId::new(456),
            ))
            .await;
        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "test.txt");
        assert_eq!(downloaded_file.content_type, "application/octet-stream");
        assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_with_u32_ids() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let attachment_content = "Test content for u32 ID test.";

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/789/attachments/101"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(attachment_content)
                    .insert_header("Content-Type", "image/png")
                    .insert_header("Content-Disposition", "attachment; filename=\"image.png\""),
            )
            .mount(&mock_server)
            .await;

        // Test using u32 directly (Into<WikiId> and Into<WikiAttachmentId> conversions)
        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(789u32, 101u32))
            .await;
        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "image.png");
        assert_eq!(downloaded_file.content_type, "image/png");
        assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_binary_content() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        let binary_content = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/555/attachments/777"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(binary_content.clone())
                    .insert_header("Content-Type", "image/png")
                    .insert_header("Content-Disposition", "attachment; filename=\"binary.png\""),
            )
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(555),
                WikiAttachmentId::new(777),
            ))
            .await;
        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "binary.png");
        assert_eq!(downloaded_file.content_type, "image/png");
        assert_eq!(downloaded_file.bytes, binary_content);
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/999/attachments/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(999),
                WikiAttachmentId::new(999),
            ))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/404/attachments/1"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(404),
                WikiAttachmentId::new(1),
            ))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/123/attachments/456"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(123),
                WikiAttachmentId::new(456),
            ))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_wiki_attachment_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let wiki_api = WikiApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/wikis/123/attachments/456"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let result = wiki_api
            .download_wiki_attachment(DownloadWikiAttachmentParams::new(
                WikiId::new(123),
                WikiAttachmentId::new(456),
            ))
            .await;
        assert!(result.is_err());
    }

    // Tests for update_wiki
    #[cfg(all(test, feature = "writable"))]
    mod update_wiki_tests {
        use super::*;
        use wiremock::matchers::{body_string_contains, header, method, path};

        #[tokio::test]
        async fn test_update_wiki_success_with_all_params() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(123, 456, "Updated Wiki Title");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/123"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .and(body_string_contains("name=Updated+Wiki+Title"))
                .and(body_string_contains("content=Updated+wiki+content"))
                .and(body_string_contains("mailNotify=true"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new()
                .name("Updated Wiki Title")
                .content("Updated wiki content")
                .mail_notify(true);

            let result = wiki_api.update_wiki(WikiId::new(123), &params).await;
            assert!(result.is_ok());
            let detail = result.unwrap();
            assert_eq!(detail.name, "Updated Wiki Title");
            assert_eq!(detail.id.value(), 123);
        }

        #[tokio::test]
        async fn test_update_wiki_success_with_name_only() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(456, 789, "New Title Only");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/456"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .and(body_string_contains("name=New+Title+Only"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().name("New Title Only");

            let result = wiki_api.update_wiki(WikiId::new(456), &params).await;
            assert!(result.is_ok());
            let detail = result.unwrap();
            assert_eq!(detail.name, "New Title Only");
        }

        #[tokio::test]
        async fn test_update_wiki_success_with_content_only() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(789, 123, "Original Title");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/789"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .and(body_string_contains("content=New+content+here"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().content("New content here");

            let result = wiki_api.update_wiki(789u32, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_update_wiki_success_with_mail_notify_false() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(111, 222, "Test Wiki");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/111"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .and(body_string_contains("mailNotify=false"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().mail_notify(false);

            let result = wiki_api.update_wiki(111u32, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_update_wiki_success_with_special_characters() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(333, 444, "Title & <Special>");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/333"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new()
                .name("Title & <Special>")
                .content("Content with \"quotes\" and symbols: @#$%");

            let result = wiki_api.update_wiki(333u32, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_update_wiki_not_found() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/404"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().name("Does not exist");

            let result = wiki_api.update_wiki(404u32, &params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_update_wiki_unauthorized() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/403"))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().name("Unauthorized");

            let result = wiki_api.update_wiki(403u32, &params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_update_wiki_server_error() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/500"))
                .respond_with(ResponseTemplate::new(500))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new().content("Server error content");

            let result = wiki_api.update_wiki(500u32, &params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_update_wiki_empty_params() {
            let mock_server = MockServer::start().await;
            let client = setup_client(&mock_server).await;
            let wiki_api = WikiApi::new(client);

            let expected_detail = create_mock_wiki_detail(555, 666, "Unchanged");

            Mock::given(method("PATCH"))
                .and(path("/api/v2/wikis/555"))
                .and(header("Content-Type", "application/x-www-form-urlencoded"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
                .mount(&mock_server)
                .await;

            let params = UpdateWikiParams::new(); // No parameters set

            let result = wiki_api.update_wiki(555u32, &params).await;
            assert!(result.is_ok());
        }
    }
}
