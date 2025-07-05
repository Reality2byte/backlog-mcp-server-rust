#[cfg(test)]
mod tests {
    #[cfg(feature = "writable")]
    use backlog_core::id::TeamId;
    use backlog_core::{ProjectIdOrKey, identifier::Identifier};
    use backlog_project::GetProjectTeamListParams;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_project_api(mock_server: &MockServer) -> backlog_project::ProjectApi {
        let client = client::test_utils::setup_client(mock_server).await;
        backlog_project::ProjectApi::new(client)
    }

    #[tokio::test]
    async fn test_get_project_team_list_with_project_id() {
        let mock_server = MockServer::start().await;
        let project_api = setup_project_api(&mock_server).await;

        let project_id = ProjectIdOrKey::Id(1.into());
        let json_response = r#"[
            {
                "id": 3,
                "name": "Development Team",
                "members": [
                    {
                        "id": 1,
                        "userId": "admin",
                        "name": "Admin User",
                        "roleType": 1,
                        "lang": "ja",
                        "mailAddress": "admin@example.com"
                    },
                    {
                        "id": 2,
                        "userId": "developer",
                        "name": "Developer",
                        "roleType": 2,
                        "lang": "en",
                        "mailAddress": "dev@example.com"
                    }
                ],
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "created": "2023-01-01T00:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "updated": "2023-01-01T00:00:00Z"
            }
        ]"#;

        Mock::given(method("GET"))
            .and(path("/api/v2/projects/1/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(json_response).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = GetProjectTeamListParams {
            project_id_or_key: project_id,
        };
        let result = project_api.get_project_team_list(params).await;

        assert!(result.is_ok());
        let teams = result.unwrap();
        assert_eq!(teams.len(), 1);
        let team = &teams[0];
        assert_eq!(team.id.value(), 3);
        assert_eq!(team.name, "Development Team");
        assert_eq!(team.members.len(), 2);
    }

    #[tokio::test]
    async fn test_get_project_team_list_with_project_key() {
        let mock_server = MockServer::start().await;
        let project_api = setup_project_api(&mock_server).await;

        let project_key = ProjectIdOrKey::Key("TESTPROJECT".parse().unwrap());
        let json_response = r#"[]"#;

        Mock::given(method("GET"))
            .and(path("/api/v2/projects/TESTPROJECT/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(json_response).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = GetProjectTeamListParams {
            project_id_or_key: project_key,
        };
        let result = project_api.get_project_team_list(params).await;

        assert!(result.is_ok());
        let teams = result.unwrap();
        assert_eq!(teams.len(), 0);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_project_team() {
        use backlog_project::AddProjectTeamParams;

        let mock_server = MockServer::start().await;
        let project_api = setup_project_api(&mock_server).await;

        let project_id = ProjectIdOrKey::Id(1.into());
        let team_id = TeamId::new(3);

        let json_response = r#"{
            "id": 3,
            "name": "Development Team",
            "members": [
                {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                }
            ],
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2023-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2023-01-01T00:00:00Z"
        }"#;

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/1/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(json_response).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = AddProjectTeamParams {
            project_id_or_key: project_id,
            team_id,
        };
        let result = project_api.add_project_team(params).await;

        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.id.value(), 3);
        assert_eq!(team.name, "Development Team");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_project_team() {
        use backlog_project::DeleteProjectTeamParams;

        let mock_server = MockServer::start().await;
        let project_api = setup_project_api(&mock_server).await;

        let project_id = ProjectIdOrKey::Id(1.into());
        let team_id = TeamId::new(3);

        let json_response = r#"{
            "id": 3,
            "name": "Development Team",
            "members": [],
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2023-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2023-01-01T00:00:00Z"
        }"#;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/1/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(json_response).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = DeleteProjectTeamParams {
            project_id_or_key: project_id,
            team_id,
        };
        let result = project_api.delete_project_team(params).await;

        assert!(result.is_ok());
        let team = result.unwrap();
        assert_eq!(team.id.value(), 3);
        assert_eq!(team.name, "Development Team");
    }
}
