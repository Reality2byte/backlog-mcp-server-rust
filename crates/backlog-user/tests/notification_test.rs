use backlog_user::models::Notification;

#[test]
fn test_deserialize_minimal_notification() {
    let json = r#"{
        "id": 22,
        "alreadyRead": false,
        "reason": 2,
        "resourceAlreadyRead": false,
        "project": {
            "id": 92,
            "projectKey": "SUB",
            "name": "Subtasking",
            "chartEnabled": false,
            "subtaskingEnabled": true,
            "projectLeaderCanEditProjectLeader": false,
            "useWiki": true,
            "useFileSharing": true,
            "useWikiTreeView": true,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "markdown",
            "archived": false,
            "displayOrder": 0,
            "useDevAttributes": true
        },
        "issue": null,
        "comment": null,
        "pullRequest": null,
        "pullRequestComment": null,
        "sender": {
            "id": 2,
            "userId": "user1",
            "name": "Test User",
            "roleType": 2,
            "lang": "en",
            "mailAddress": "user@example.com"
        },
        "created": "2024-12-01T10:00:00Z"
    }"#;

    let result: Result<Notification, _> = serde_json::from_str(json);
    if let Err(e) = &result {
        eprintln!("Deserialization error: {e}");
    }
    assert!(result.is_ok());
}
