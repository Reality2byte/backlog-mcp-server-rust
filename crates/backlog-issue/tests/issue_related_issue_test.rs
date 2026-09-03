mod common;
use common::*;

use backlog_core::IssueKey;
use backlog_issue::GetRelatedIssuesParams;
use std::str::FromStr;

fn related_issue_json(id: u32, key: &str) -> serde_json::Value {
    json!({
        "id": id, "projectId": 1, "issueKey": key, "keyId": id, "summary": format!("Issue {id}"),
        "description": "",
        "issueType": {"id": 1, "projectId": 1, "name": "Task", "color": "#7ea800", "displayOrder": 0},
        "priority": {"id": 3, "name": "Normal"},
        "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
        "category": [], "versions": [], "milestone": [],
        "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "mailAddress": "admin@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
        "created": "2024-03-14T06:35:39Z",
        "updated": "2024-04-13T06:35:39Z",
        "type": "RELATES"
    })
}

#[tokio::test]
async fn test_get_related_issues_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/issues/BLG-1/relatedIssues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            related_issue_json(2, "BLG-2"),
            related_issue_json(3, "BLG-3"),
        ])))
        .mount(&mock_server)
        .await;

    let params = GetRelatedIssuesParams::new(IssueKey::from_str("BLG-1").unwrap());
    let related = issue_api.get_related_issues(params).await.unwrap();

    assert_eq!(related.len(), 2);
    assert_eq!(related[0].issue.id, IssueId::new(2));
    assert_eq!(related[0].issue.issue_key.to_string(), "BLG-2");
    assert_eq!(related[0].relation_type, "RELATES");
    assert_eq!(related[1].issue.id, IssueId::new(3));
}

#[tokio::test]
async fn test_get_related_issues_empty() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/issues/12345/relatedIssues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let params = GetRelatedIssuesParams::new(IssueId::new(12345));
    let related = issue_api.get_related_issues(params).await.unwrap();

    assert!(related.is_empty());
}

#[tokio::test]
async fn test_get_related_issues_not_found() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/issues/BLG-999/relatedIssues"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "errors": [{"message": "No issue for the issueIdOrKey."}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetRelatedIssuesParams::new(IssueKey::from_str("BLG-999").unwrap());
    let result = issue_api.get_related_issues(params).await;

    assert!(result.is_err());
}

#[cfg(feature = "writable")]
mod writable_tests {
    use super::*;
    use backlog_issue::{AddRelatedIssueParams, DeleteRelatedIssueParams};
    use wiremock::matchers::body_string;

    #[tokio::test]
    async fn test_add_related_issue_success() {
        let mock_server = wiremock::MockServer::start().await;
        let issue_api = setup_issue_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/BLG-1/relatedIssues"))
            .and(body_string("targetIssueId=2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(related_issue_json(2, "BLG-2")))
            .mount(&mock_server)
            .await;

        let params =
            AddRelatedIssueParams::new(IssueKey::from_str("BLG-1").unwrap(), IssueId::new(2));
        let related = issue_api.add_related_issue(params).await.unwrap();

        assert_eq!(related.issue.id, IssueId::new(2));
        assert_eq!(related.relation_type, "RELATES");
    }

    #[tokio::test]
    async fn test_add_related_issue_bad_request() {
        let mock_server = wiremock::MockServer::start().await;
        let issue_api = setup_issue_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/BLG-1/relatedIssues"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "errors": [{"message": "Too many related issues."}]
            })))
            .mount(&mock_server)
            .await;

        let params =
            AddRelatedIssueParams::new(IssueKey::from_str("BLG-1").unwrap(), IssueId::new(2));
        let result = issue_api.add_related_issue(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_related_issue_success() {
        let mock_server = wiremock::MockServer::start().await;
        let issue_api = setup_issue_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/issues/BLG-1/relatedIssues/2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(related_issue_json(2, "BLG-2")))
            .mount(&mock_server)
            .await;

        let params =
            DeleteRelatedIssueParams::new(IssueKey::from_str("BLG-1").unwrap(), IssueId::new(2));
        let related = issue_api.delete_related_issue(params).await.unwrap();

        assert_eq!(related.issue.id, IssueId::new(2));
        assert_eq!(related.relation_type, "RELATES");
    }

    #[tokio::test]
    async fn test_delete_related_issue_not_found() {
        let mock_server = wiremock::MockServer::start().await;
        let issue_api = setup_issue_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/issues/BLG-1/relatedIssues/2"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "errors": [{"message": "No related issue."}]
            })))
            .mount(&mock_server)
            .await;

        let params =
            DeleteRelatedIssueParams::new(IssueKey::from_str("BLG-1").unwrap(), IssueId::new(2));
        let result = issue_api.delete_related_issue(params).await;

        assert!(result.is_err());
    }
}
