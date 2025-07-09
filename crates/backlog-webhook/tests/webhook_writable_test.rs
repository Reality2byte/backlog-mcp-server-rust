#[cfg(feature = "writable")]
mod common;

#[cfg(feature = "writable")]
mod tests {
    use super::common::*;
    use backlog_api_core::IntoRequest;
    use backlog_core::{
        ProjectIdOrKey, ProjectKey,
        id::{ActivityTypeId, ProjectId, WebhookId},
    };
    use backlog_webhook::{UpdateWebhookParams, UpdateWebhookParamsBuilder, WebhookApi};
    use wiremock::{Mock, ResponseTemplate, matchers};

    #[tokio::test]
    async fn test_update_webhook_params_path() {
        let params = UpdateWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            webhook_id: WebhookId::new(123),
            name: None,
            description: None,
            hook_url: None,
            all_event: None,
            activity_type_ids: None,
        };
        assert_eq!(params.path(), "/api/v2/projects/TEST/webhooks/123");

        let params_with_id = UpdateWebhookParams {
            project_id_or_key: ProjectIdOrKey::from(ProjectId::new(456)),
            webhook_id: WebhookId::new(789),
            name: None,
            description: None,
            hook_url: None,
            all_event: None,
            activity_type_ids: None,
        };
        assert_eq!(params_with_id.path(), "/api/v2/projects/456/webhooks/789");
    }

    #[tokio::test]
    async fn test_update_webhook_params_form() {
        let params = UpdateWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            webhook_id: WebhookId::new(123),
            name: Some("Updated Webhook".to_string()),
            description: Some("Updated description".to_string()),
            hook_url: Some("https://example.com/new-hook".to_string()),
            all_event: Some(true),
            activity_type_ids: Some(vec![
                ActivityTypeId::new(1),
                ActivityTypeId::new(2),
                ActivityTypeId::new(3),
            ]),
        };

        let form: Vec<(String, String)> = (&params).into();

        // Check all form parameters
        assert!(form.contains(&("name".to_string(), "Updated Webhook".to_string())));
        assert!(form.contains(&("description".to_string(), "Updated description".to_string())));
        assert!(form.contains(&(
            "hookUrl".to_string(),
            "https://example.com/new-hook".to_string()
        )));
        assert!(form.contains(&("allEvent".to_string(), "true".to_string())));

        // Check array parameters
        assert!(form.contains(&("activityTypeId[]".to_string(), "1".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "2".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "3".to_string())));
    }

    #[tokio::test]
    async fn test_update_webhook_minimal_params() {
        let params = UpdateWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            webhook_id: WebhookId::new(123),
            name: Some("New Name".to_string()),
            description: None,
            hook_url: None,
            all_event: None,
            activity_type_ids: None,
        };

        let form: Vec<(String, String)> = (&params).into();

        // Only name should be included
        assert_eq!(form.len(), 1);
        assert!(form.contains(&("name".to_string(), "New Name".to_string())));
    }

    #[tokio::test]
    async fn test_update_webhook_success() {
        let mock_server = setup_mock_server().await;
        let response_body = mock_single_webhook_response();

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/projects/TEST/webhooks/1"))
            .and(matchers::body_string_contains("name=Updated+Webhook"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = client::Client::new(&mock_server.uri())
            .unwrap()
            .with_api_key("test-api-key");
        let api = WebhookApi::new(client);

        let params = UpdateWebhookParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()))
            .webhook_id(WebhookId::new(1))
            .name("Updated Webhook")
            .build()
            .unwrap();

        let result = api.execute_update_webhook(params).await;
        assert!(result.is_ok(), "Error: {:?}", result.err());

        let webhook = result.unwrap();
        assert_eq!(webhook.id, 1);
    }

    #[tokio::test]
    async fn test_update_webhook_builder_pattern() {
        let mut builder = UpdateWebhookParamsBuilder::default();
        let params = builder
            .project_id_or_key(ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()))
            .webhook_id(WebhookId::new(123))
            .name("Test Webhook")
            .description("Test Description")
            .hook_url("https://example.com/hook")
            .all_event(false)
            .activity_type_ids(vec![ActivityTypeId::new(1), ActivityTypeId::new(2)])
            .build()
            .unwrap();

        assert_eq!(params.name, Some("Test Webhook".to_string()));
        assert_eq!(params.description, Some("Test Description".to_string()));
        assert_eq!(
            params.hook_url,
            Some("https://example.com/hook".to_string())
        );
        assert_eq!(params.all_event, Some(false));
        assert_eq!(params.activity_type_ids.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_update_webhook_all_event_false_with_activity_types() {
        let params = UpdateWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            webhook_id: WebhookId::new(123),
            name: None,
            description: None,
            hook_url: None,
            all_event: Some(false),
            activity_type_ids: Some(vec![
                ActivityTypeId::new(1),
                ActivityTypeId::new(4),
                ActivityTypeId::new(5),
            ]),
        };

        let form: Vec<(String, String)> = (&params).into();

        // Check both parameters are included
        assert!(form.contains(&("allEvent".to_string(), "false".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "1".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "4".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "5".to_string())));
    }

    #[tokio::test]
    async fn test_update_webhook_not_found() {
        let mock_server = setup_mock_server().await;
        let response_body = mock_error_response();

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/projects/TEST/webhooks/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = client::Client::new(&mock_server.uri())
            .unwrap()
            .with_api_key("test-api-key");
        let api = WebhookApi::new(client);

        let params = UpdateWebhookParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()))
            .webhook_id(WebhookId::new(999))
            .name("Updated Webhook")
            .build()
            .unwrap();

        let result = api.execute_update_webhook(params).await;
        assert!(result.is_err());
    }

    // Add webhook tests
    #[tokio::test]
    async fn test_add_webhook_params_path() {
        use backlog_webhook::AddWebhookParams;

        let params = AddWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            name: "Test Webhook".to_string(),
            hook_url: "https://example.com/webhook".to_string(),
            description: None,
            all_event: None,
            activity_type_ids: None,
        };
        assert_eq!(params.path(), "/api/v2/projects/TEST/webhooks");

        let params_with_id = AddWebhookParams {
            project_id_or_key: ProjectIdOrKey::from(ProjectId::new(123)),
            name: "Test Webhook".to_string(),
            hook_url: "https://example.com/webhook".to_string(),
            description: None,
            all_event: None,
            activity_type_ids: None,
        };
        assert_eq!(params_with_id.path(), "/api/v2/projects/123/webhooks");
    }

    #[tokio::test]
    async fn test_add_webhook_params_form() {
        use backlog_webhook::AddWebhookParams;

        let params = AddWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            name: "New Webhook".to_string(),
            hook_url: "https://example.com/new-webhook".to_string(),
            description: Some("Test webhook description".to_string()),
            all_event: Some(false),
            activity_type_ids: Some(vec![
                ActivityTypeId::new(1),
                ActivityTypeId::new(2),
                ActivityTypeId::new(3),
            ]),
        };

        let form: Vec<(String, String)> = (&params).into();

        // Check all form parameters
        assert!(form.contains(&("name".to_string(), "New Webhook".to_string())));
        assert!(form.contains(&(
            "hookUrl".to_string(),
            "https://example.com/new-webhook".to_string()
        )));
        assert!(form.contains(&(
            "description".to_string(),
            "Test webhook description".to_string()
        )));
        assert!(form.contains(&("allEvent".to_string(), "false".to_string())));

        // Check array parameters
        assert!(form.contains(&("activityTypeId[]".to_string(), "1".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "2".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "3".to_string())));
    }

    #[tokio::test]
    async fn test_add_webhook_minimal_params() {
        use backlog_webhook::AddWebhookParams;

        let params = AddWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            name: "Minimal Webhook".to_string(),
            hook_url: "https://example.com/minimal".to_string(),
            description: None,
            all_event: None,
            activity_type_ids: None,
        };

        let form: Vec<(String, String)> = (&params).into();

        // Should only contain required parameters
        assert!(form.contains(&("name".to_string(), "Minimal Webhook".to_string())));
        assert!(form.contains(&(
            "hookUrl".to_string(),
            "https://example.com/minimal".to_string()
        )));

        // Count should be exactly 2 (only required params)
        let required_params_count = form
            .iter()
            .filter(|(key, _)| key == "name" || key == "hookUrl")
            .count();
        assert_eq!(required_params_count, 2);
    }

    #[tokio::test]
    async fn test_add_webhook_success() {
        let mock_server = setup_mock_server().await;
        let response_body = mock_single_webhook_response();

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/projects/TEST/webhooks"))
            .and(matchers::body_string_contains("name=New+Webhook"))
            .and(matchers::body_string_contains("hookUrl=https"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = client::Client::new(&mock_server.uri())
            .unwrap()
            .with_api_key("test-api-key");
        let api = WebhookApi::new(client);

        use backlog_webhook::AddWebhookParamsBuilder;

        let params = AddWebhookParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()))
            .name("New Webhook")
            .hook_url("https://example.com/webhook")
            .build()
            .unwrap();

        let result = api.execute_add_webhook(params).await;
        assert!(result.is_ok(), "Error: {:?}", result.err());

        let webhook = result.unwrap();
        assert_eq!(webhook.id, 1);
        assert_eq!(webhook.name, "webhook1");
    }

    #[tokio::test]
    async fn test_add_webhook_builder_pattern() {
        use backlog_webhook::AddWebhookParamsBuilder;

        let mut builder = AddWebhookParamsBuilder::default();
        let params = builder
            .project_id_or_key(ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()))
            .name("Builder Test Webhook")
            .hook_url("https://example.com/builder")
            .description("Built with builder pattern")
            .all_event(true)
            .activity_type_ids(vec![ActivityTypeId::new(1), ActivityTypeId::new(2)])
            .build()
            .unwrap();

        assert_eq!(params.name, "Builder Test Webhook".to_string());
        assert_eq!(params.hook_url, "https://example.com/builder".to_string());
        assert_eq!(
            params.description,
            Some("Built with builder pattern".to_string())
        );
        assert_eq!(params.all_event, Some(true));
        assert_eq!(params.activity_type_ids.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_add_webhook_all_event_true_with_activity_types() {
        use backlog_webhook::AddWebhookParams;

        // Test when all_event is true but activity_type_ids are also specified
        let params = AddWebhookParams {
            project_id_or_key: ProjectIdOrKey::from("TEST".parse::<ProjectKey>().unwrap()),
            name: "All Event Webhook".to_string(),
            hook_url: "https://example.com/all-events".to_string(),
            description: None,
            all_event: Some(true),
            activity_type_ids: Some(vec![ActivityTypeId::new(1), ActivityTypeId::new(2)]),
        };

        let form: Vec<(String, String)> = (&params).into();

        // Both parameters should be included
        assert!(form.contains(&("allEvent".to_string(), "true".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "1".to_string())));
        assert!(form.contains(&("activityTypeId[]".to_string(), "2".to_string())));
    }

    #[tokio::test]
    async fn test_add_webhook_project_not_found() {
        let mock_server = setup_mock_server().await;
        let response_body = mock_error_response();

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/projects/INVALID/webhooks"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = client::Client::new(&mock_server.uri())
            .unwrap()
            .with_api_key("test-api-key");
        let api = WebhookApi::new(client);

        use backlog_webhook::AddWebhookParamsBuilder;

        let params = AddWebhookParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::from(
                "INVALID".parse::<ProjectKey>().unwrap(),
            ))
            .name("Test Webhook")
            .hook_url("https://example.com/webhook")
            .build()
            .unwrap();

        let result = api.execute_add_webhook(params).await;
        assert!(result.is_err());
    }
}
