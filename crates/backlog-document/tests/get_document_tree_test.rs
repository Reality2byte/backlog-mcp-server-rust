mod common;
use common::*;

use backlog_core::identifier::{DocumentId, ProjectId};
use backlog_document::GetDocumentTreeParams;
use wiremock::matchers::query_param;

#[tokio::test]
async fn test_get_document_tree_success() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"{
        "projectId": 1,
        "activeTree": {
            "id": "Active",
            "children": [
                {
                    "id": "doc1",
                    "name": "設計書",
                    "updated": "2023-12-01T10:00:00Z",
                    "emoji": "📝",
                    "emojiType": "document",
                    "statusId": 1,
                    "children": [
                        {
                            "id": "doc2",
                            "name": "API設計書",
                            "updated": "2023-12-01T09:30:00Z",
                            "emoji": "🔧",
                            "emojiType": "tool",
                            "statusId": 1,
                            "children": []
                        }
                    ]
                },
                {
                    "id": "doc3",
                    "name": "プロジェクト概要",
                    "updated": "2023-11-30T15:00:00Z",
                    "emoji": "📋",
                    "emojiType": "clipboard",
                    "statusId": 1,
                    "children": []
                }
            ]
        },
        "trashTree": {
            "id": "Trash",
            "children": [
                {
                    "id": "doc4",
                    "name": "削除予定ドキュメント",
                    "updated": "2023-11-29T12:00:00Z",
                    "emoji": "🗑️",
                    "emojiType": "trash",
                    "statusId": 2,
                    "children": []
                }
            ]
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "TEST_PROJECT"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: "TEST_PROJECT".parse().unwrap(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_ok());
    let tree = result.unwrap();

    // Verify project ID
    assert_eq!(tree.project_id, ProjectId::new(1));

    // Verify active tree structure
    assert_eq!(tree.active_tree.id, "Active");
    assert_eq!(tree.active_tree.children.len(), 2);

    // Verify first document in active tree
    let first_doc = &tree.active_tree.children[0];
    assert_eq!(first_doc.id, DocumentId::unsafe_new("doc1".to_string()));
    assert_eq!(first_doc.name, "設計書");
    assert_eq!(first_doc.emoji, Some("📝".to_string()));
    assert_eq!(first_doc.emoji_type, Some("document".to_string()));
    assert_eq!(first_doc.children.len(), 1);

    // Verify nested document
    let nested_doc = &first_doc.children[0];
    assert_eq!(nested_doc.id, DocumentId::unsafe_new("doc2".to_string()));
    assert_eq!(nested_doc.name, "API設計書");
    assert_eq!(nested_doc.emoji, Some("🔧".to_string()));
    assert_eq!(nested_doc.children.len(), 0);

    // Verify second document in active tree
    let second_doc = &tree.active_tree.children[1];
    assert_eq!(second_doc.id, DocumentId::unsafe_new("doc3".to_string()));
    assert_eq!(second_doc.name, "プロジェクト概要");
    assert_eq!(second_doc.children.len(), 0);

    // Verify trash tree
    assert_eq!(tree.trash_tree.id, "Trash");
    assert_eq!(tree.trash_tree.children.len(), 1);

    let trash_doc = &tree.trash_tree.children[0];
    assert_eq!(trash_doc.id, DocumentId::unsafe_new("doc4".to_string()));
    assert_eq!(trash_doc.name, "削除予定ドキュメント");
    assert_eq!(trash_doc.emoji, Some("🗑️".to_string()));
}

#[tokio::test]
async fn test_get_document_tree_with_project_id() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"{
        "projectId": 123,
        "activeTree": {
            "id": "Active",
            "children": []
        },
        "trashTree": {
            "id": "Trash", 
            "children": []
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "123"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: ProjectId::new(123).into(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_ok());
    let tree = result.unwrap();
    assert_eq!(tree.project_id, ProjectId::new(123));
    assert_eq!(tree.active_tree.children.len(), 0);
    assert_eq!(tree.trash_tree.children.len(), 0);
}

#[tokio::test]
async fn test_get_document_tree_empty_response() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"{
        "projectId": 1,
        "activeTree": {
            "id": "Active",
            "children": []
        },
        "trashTree": {
            "id": "Trash",
            "children": []
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "EMPTY_PROJECT"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: "EMPTY_PROJECT".parse().unwrap(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_ok());
    let tree = result.unwrap();
    assert_eq!(tree.active_tree.children.len(), 0);
    assert_eq!(tree.trash_tree.children.len(), 0);
}

#[tokio::test]
async fn test_get_document_tree_not_found() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "NONEXISTENT"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_string(r#"{"errors":[{"message":"プロジェクトが見つかりません","code":6,"moreInfo":""}]}"#)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: "NONEXISTENT".parse().unwrap(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_document_tree_unauthorized() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "PRIVATE_PROJECT"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_string(r#"{"errors":[{"message":"このリソースにアクセスする権限がありません","code":11,"moreInfo":""}]}"#)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: "PRIVATE_PROJECT".parse().unwrap(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_document_tree_server_error() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents/tree"))
        .and(query_param("projectIdOrKey", "ERROR_PROJECT"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let params = GetDocumentTreeParams {
        project_id_or_key: "ERROR_PROJECT".parse().unwrap(),
    };

    let result = doc_api.get_document_tree(params).await;

    assert!(result.is_err());
}
