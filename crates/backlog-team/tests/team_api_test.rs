use backlog_core::{id::TeamId, identifier::Identifier};
use backlog_team::api::{GetTeamParams, ListTeamsOrder, ListTeamsParams};
use pretty_assertions::assert_eq;
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

mod common;
use common::setup_team_api;

#[tokio::test]
async fn test_get_team_success() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let team_id = 123;
    let expected_response = json!({
        "id": team_id,
        "name": "Marketing Team",
        "members": [
            {
                "id": 1,
                "userId": "5",
                "roleType": 1,
                "lang": "ja",
                "name": "admin",
                "mailAddress": "admin@example.com",
                "nulabAccount": {
                    "nulabId": "abc123",
                    "name": "admin",
                    "uniqueId": "admin-unique"
                },
                "keyword": "admin"
            },
            {
                "id": 2,
                "userId": "10",
                "roleType": 2,
                "lang": "ja",
                "name": "user1",
                "mailAddress": "user1@example.com",
                "nulabAccount": {
                    "nulabId": "def456",
                    "name": "user1",
                    "uniqueId": "user1-unique"
                },
                "keyword": "user1"
            }
        ],
        "createdUser": {
            "id": 1,
            "userId": "1",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com",
            "nulabAccount": {
                "nulabId": "abc123",
                "name": "admin",
                "uniqueId": "admin-unique"
            },
            "keyword": "admin"
        },
        "created": "2024-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "1",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com",
            "nulabAccount": {
                "nulabId": "abc123",
                "name": "admin",
                "uniqueId": "admin-unique"
            },
            "keyword": "admin"
        },
        "updated": "2024-01-02T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/teams/{team_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetTeamParams {
        team_id: TeamId::new(team_id),
    };

    let result = api.get_team(params).await;
    if let Err(ref e) = result {
        eprintln!("Error calling get_team: {e:?}");
    }
    assert!(result.is_ok());

    let team = result.unwrap();
    assert_eq!(team.id.value(), team_id);
    assert_eq!(team.name, "Marketing Team");
    assert_eq!(team.members.len(), 2);
    assert_eq!(team.members[0].name, "admin");
    assert_eq!(team.members[1].name, "user1");
    assert_eq!(team.created_user.name, "admin");
    assert_eq!(team.updated_user.name, "admin");
}

#[tokio::test]
async fn test_get_team_not_found() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let team_id = 999;
    let error_response = json!({
        "errors": [
            {
                "message": "No team found.",
                "code": 3,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/teams/{team_id}")))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let params = GetTeamParams {
        team_id: TeamId::new(team_id),
    };

    let result = api.get_team(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_team_forbidden() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let team_id = 123;
    let error_response = json!({
        "errors": [
            {
                "message": "You do not have permission to view this team.",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/teams/{team_id}")))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let params = GetTeamParams {
        team_id: TeamId::new(team_id),
    };

    let result = api.get_team(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_teams_success() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let expected_response = json!([
        {
            "id": 1,
            "name": "Engineering Team",
            "displayOrder": 1,
            "members": [
                {
                    "id": 1,
                    "userId": "5",
                    "roleType": 1,
                    "lang": "ja",
                    "name": "admin",
                    "mailAddress": "admin@example.com",
                    "nulabAccount": {
                        "nulabId": "abc123",
                        "name": "admin",
                        "uniqueId": "admin-unique"
                    },
                    "keyword": "admin"
                }
            ],
            "createdUser": {
                "id": 1,
                "userId": "1",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": {
                    "nulabId": "abc123",
                    "name": "admin",
                    "uniqueId": "admin-unique"
                },
                "keyword": "admin"
            },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "1",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": {
                    "nulabId": "abc123",
                    "name": "admin",
                    "uniqueId": "admin-unique"
                },
                "keyword": "admin"
            },
            "updated": "2024-01-02T00:00:00Z"
        },
        {
            "id": 2,
            "name": "Marketing Team",
            "displayOrder": 2,
            "members": [
                {
                    "id": 2,
                    "userId": "10",
                    "roleType": 2,
                    "lang": "ja",
                    "name": "user1",
                    "mailAddress": "user1@example.com",
                    "nulabAccount": {
                        "nulabId": "def456",
                        "name": "user1",
                        "uniqueId": "user1-unique"
                    },
                    "keyword": "user1"
                }
            ],
            "createdUser": {
                "id": 1,
                "userId": "1",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": {
                    "nulabId": "abc123",
                    "name": "admin",
                    "uniqueId": "admin-unique"
                },
                "keyword": "admin"
            },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "1",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": {
                    "nulabId": "abc123",
                    "name": "admin",
                    "uniqueId": "admin-unique"
                },
                "keyword": "admin"
            },
            "updated": "2024-01-02T00:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/teams"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = ListTeamsParams::default();

    let result = api.list_teams(params).await;
    if let Err(ref e) = result {
        eprintln!("Error calling list_teams: {e:?}");
    }
    assert!(result.is_ok());

    let teams = result.unwrap();
    assert_eq!(teams.len(), 2);
    assert_eq!(teams[0].team.id.value(), 1);
    assert_eq!(teams[0].team.name, "Engineering Team");
    assert_eq!(teams[0].display_order, Some(1));
    assert_eq!(teams[1].team.id.value(), 2);
    assert_eq!(teams[1].team.name, "Marketing Team");
    assert_eq!(teams[1].display_order, Some(2));
}

#[tokio::test]
async fn test_list_teams_with_params() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let expected_response = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/teams"))
        .and(query_param("order", "asc"))
        .and(query_param("offset", "10"))
        .and(query_param("count", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = ListTeamsParams {
        order: Some(ListTeamsOrder::Asc),
        offset: Some(10),
        count: Some(50),
    };

    let result = api.list_teams(params).await;
    assert!(result.is_ok());

    let teams = result.unwrap();
    assert!(teams.is_empty());
}

#[tokio::test]
async fn test_list_teams_forbidden() {
    let mock_server = MockServer::start().await;
    let api = setup_team_api(&mock_server).await;

    let error_response = json!({
        "errors": [
            {
                "message": "You do not have permission to view teams.",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/teams"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let params = ListTeamsParams::default();

    let result = api.list_teams(params).await;
    assert!(result.is_err());
}
