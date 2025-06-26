#[cfg(feature = "writable")]
mod writable_tests {
    use super::common::*;
    use backlog_wiki::api::{AddWikiParams, UpdateWikiParams};
    use wiremock::MockServer;
    use wiremock::matchers::{body_string_contains, header, method, path};

    #[tokio::test]
    async fn test_update_wiki_success_with_all_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

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

        let params = UpdateWikiParams::new(WikiId::new(123))
            .name("Updated Wiki Title")
            .content("Updated wiki content")
            .mail_notify(true);

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Updated Wiki Title");
        assert_eq!(detail.id.value(), 123);
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_name_only() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(456, 789, "New Title Only");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/456"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("name=New+Title+Only"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(456)).name("New Title Only");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "New Title Only");
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_content_only() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(789, 123, "Original Title");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/789"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("content=New+content+here"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(789)).content("New content here");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_mail_notify_false() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(111, 222, "Test Wiki");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/111"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("mailNotify=false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(111)).mail_notify(false);

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/404"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(404)).name("Does not exist");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/403"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(403)).name("Unauthorized");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/500"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(500)).content("Server error content");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_empty_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(555, 666, "Unchanged");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/555"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(555));

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    // Tests for add_wiki functionality
    #[tokio::test]
    async fn test_add_wiki_success_with_all_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(789, 123, "New Wiki Page");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=123"))
            .and(body_string_contains("name=New+Wiki+Page"))
            .and(body_string_contains("content=This+is+new+wiki+content"))
            .and(body_string_contains("mailNotify=true"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(
            ProjectId::new(123),
            "New Wiki Page",
            "This is new wiki content",
        )
        .mail_notify(true);

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "New Wiki Page");
        assert_eq!(detail.project_id.value(), 123);
    }

    #[tokio::test]
    async fn test_add_wiki_success_minimal_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(456, 789, "Minimal Wiki");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=789"))
            .and(body_string_contains("name=Minimal+Wiki"))
            .and(body_string_contains("content=Basic+content"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(789), "Minimal Wiki", "Basic content");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Minimal Wiki");
    }

    #[tokio::test]
    async fn test_add_wiki_success_with_mail_notify_false() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(111, 222, "No Notify Wiki");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=222"))
            .and(body_string_contains("name=No+Notify+Wiki"))
            .and(body_string_contains("content=Content+without+notification"))
            .and(body_string_contains("mailNotify=false"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(
            ProjectId::new(222),
            "No Notify Wiki",
            "Content without notification",
        )
        .mail_notify(false);

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_wiki_bad_request() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(400), "Bad Request", "Invalid content");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(403), "Unauthorized", "No permission");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_project_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(404), "Not Found", "Project does not exist");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params =
            AddWikiParams::new(ProjectId::new(500), "Server Error", "Internal server error");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }
}

mod common;
