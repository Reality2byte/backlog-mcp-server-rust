mod common;

use backlog_core::{ProjectIdOrKey, ProjectKey, TextFormattingRule};
use backlog_project::api::{
    GetCategoryListParams, GetIssueTypeListParams, GetMilestoneListParams, GetProjectDetailParams,
    GetProjectIconParams, GetProjectListParams, GetStatusListParams,
};
use backlog_project::{Category, IssueType, Priority, Project, Resolution, Status};
use common::*;
use std::str::FromStr;
use wiremock::MockServer;

#[tokio::test]
async fn test_get_version_milestone_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id_or_key_str = "TEST_PROJECT";
    let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();
    let project_id_numeric = ProjectId::new(1);

    let expected_versions: Vec<Milestone> = vec![
        Milestone {
            id: MilestoneId::new(1),
            project_id: project_id_numeric,
            name: "Version 1.0".to_string(),
            description: Some("Initial release".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
            archived: false,
            display_order: Some(1),
        },
        Milestone {
            id: MilestoneId::new(2),
            project_id: project_id_numeric,
            name: "Version 1.1".to_string(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: true,
            display_order: Some(2),
        },
    ];

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/versions",
            project_id_or_key.clone()
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_versions))
        .mount(&mock_server)
        .await;
    let params = GetMilestoneListParams::new(project_id_or_key.clone());
    let result = project_api.get_version_milestone_list(params).await;
    assert!(result.is_ok());
    let versions = result.unwrap();
    assert_eq!(versions.len(), 2);
}

#[tokio::test]
async fn test_get_version_milestone_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id_or_key_str = "TEST_PROJECT_ERROR";
    let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/versions",
            project_id_or_key.clone()
        )))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    let params = GetMilestoneListParams::new(project_id_or_key.clone());
    let result = project_api.get_version_milestone_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_status_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_statuses = vec![
        Status {
            id: StatusId::new(1),
            project_id,
            name: "Open".to_string(),
            color: "#ff0000".to_string(),
            display_order: 1,
        },
        Status {
            id: StatusId::new(2),
            project_id,
            name: "In Progress".to_string(),
            color: "#00ff00".to_string(),
            display_order: 2,
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
        .mount(&mock_server)
        .await;

    let params = GetStatusListParams::new(project_id);
    let result = project_api.get_status_list(params).await;
    assert!(result.is_ok());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0].name, "Open");
    assert_eq!(statuses[1].name, "In Progress");
}

#[tokio::test]
async fn test_get_status_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/statuses"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetStatusListParams::new(project_id);
    let result = project_api.get_status_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_issue_type_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_issue_types = vec![
        IssueType {
            id: IssueTypeId::new(1),
            project_id,
            name: "Bug".to_string(),
            color: "#e30613".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        },
        IssueType {
            id: IssueTypeId::new(2),
            project_id,
            name: "Task".to_string(),
            color: "#7ea800".to_string(),
            display_order: 2,
            template_summary: Some("Task template".to_string()),
            template_description: Some("Task description template".to_string()),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/issueTypes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_types))
        .mount(&mock_server)
        .await;

    let params = GetIssueTypeListParams::new(project_id);
    let result = project_api.get_issue_type_list(params).await;
    assert!(result.is_ok());
    let issue_types = result.unwrap();
    assert_eq!(issue_types.len(), 2);
    assert_eq!(issue_types[0].name, "Bug");
    assert_eq!(issue_types[1].name, "Task");
}

#[tokio::test]
async fn test_get_issue_type_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/issueTypes"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetIssueTypeListParams::new(project_id);
    let result = project_api.get_issue_type_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_category_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_categories = vec![
        Category {
            id: CategoryId::new(1),
            project_id,
            name: "Backend".to_string(),
            display_order: 1,
        },
        Category {
            id: CategoryId::new(2),
            project_id,
            name: "Frontend".to_string(),
            display_order: 2,
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/categories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_categories))
        .mount(&mock_server)
        .await;

    let params = GetCategoryListParams::new(project_id);
    let result = project_api.get_category_list(params).await;
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].name, "Backend");
    assert_eq!(categories[1].name, "Frontend");
}

#[tokio::test]
async fn test_get_category_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/categories"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetCategoryListParams::new(project_id);
    let result = project_api.get_category_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_priority_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_priorities = vec![
        Priority {
            id: PriorityId::new(1),
            name: "High".to_string(),
        },
        Priority {
            id: PriorityId::new(2),
            name: "Medium".to_string(),
        },
        Priority {
            id: PriorityId::new(3),
            name: "Low".to_string(),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/priorities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_priorities))
        .mount(&mock_server)
        .await;

    let result = project_api.get_priority_list().await;
    assert!(result.is_ok());
    let priorities = result.unwrap();
    assert_eq!(priorities.len(), 3);
    assert_eq!(priorities[0].name, "High");
    assert_eq!(priorities[1].name, "Medium");
    assert_eq!(priorities[2].name, "Low");
}

#[tokio::test]
async fn test_get_priority_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/priorities"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = project_api.get_priority_list().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_resolution_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_resolutions = vec![
        Resolution {
            id: ResolutionId::new(1),
            name: "Fixed".to_string(),
        },
        Resolution {
            id: ResolutionId::new(2),
            name: "Won't Fix".to_string(),
        },
        Resolution {
            id: ResolutionId::new(3),
            name: "Duplicate".to_string(),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/resolutions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resolutions))
        .mount(&mock_server)
        .await;

    let result = project_api.get_resolution_list().await;
    assert!(result.is_ok());
    let resolutions = result.unwrap();
    assert_eq!(resolutions.len(), 3);
    assert_eq!(resolutions[0].name, "Fixed");
    assert_eq!(resolutions[1].name, "Won't Fix");
    assert_eq!(resolutions[2].name, "Duplicate");
}

#[tokio::test]
async fn test_get_resolution_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/resolutions"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = project_api.get_resolution_list().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_projects = vec![Project {
        id: ProjectId::new(1),
        project_key: ProjectKey::from_str("TEST1").unwrap(),
        name: "Test Project 1".to_string(),
        chart_enabled: true,
        subtasking_enabled: false,
        project_leader_can_edit_project_leader: true,
        use_wiki: true,
        use_file_sharing: true,
        use_wiki_tree_view: false,
        use_original_image_size_at_wiki: false,
        text_formatting_rule: TextFormattingRule::Markdown,
        archived: false,
        display_order: 0,
        use_dev_attributes: true,
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_projects))
        .mount(&mock_server)
        .await;

    let params = GetProjectListParams {
        archived: Some(false),
        all: false,
    };
    let result = project_api.get_project_list(params).await;
    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Test Project 1");
}

#[tokio::test]
async fn test_get_project_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let params = GetProjectListParams {
        archived: None,
        all: true,
    };
    let result = project_api.get_project_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_project = Project {
        id: ProjectId::new(123),
        project_key: ProjectKey::from_str("TESTPROJ").unwrap(),
        name: "Test Project".to_string(),
        chart_enabled: true,
        subtasking_enabled: true,
        project_leader_can_edit_project_leader: false,
        use_wiki: true,
        use_file_sharing: false,
        use_wiki_tree_view: false,
        use_original_image_size_at_wiki: false,
        text_formatting_rule: TextFormattingRule::Backlog,
        archived: false,
        display_order: 0,
        use_dev_attributes: false,
    };

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_project))
        .mount(&mock_server)
        .await;

    let params = GetProjectDetailParams::new(ProjectKey::from_str("TESTPROJ").unwrap());
    let result = project_api.get_project(params).await;
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.name, "Test Project");
    assert_eq!(
        project.project_key,
        ProjectKey::from_str("TESTPROJ").unwrap()
    );
}

#[tokio::test]
async fn test_get_project_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NONEXISTENT"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectDetailParams::new(ProjectKey::from_str("NONEXISTENT").unwrap());
    let result = project_api.get_project(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_icon_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_image_data = b"fake_image_data";

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ/image"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_image_data))
        .mount(&mock_server)
        .await;

    let params = GetProjectIconParams::new(ProjectKey::from_str("TESTPROJ").unwrap());
    let result = project_api.get_project_icon(params).await;
    assert!(result.is_ok());
    let image_data = result.unwrap();
    assert_eq!(image_data, expected_image_data);
}

#[tokio::test]
async fn test_get_project_icon_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NONEXISTENT/image"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectIconParams::new(ProjectKey::from_str("NONEXISTENT").unwrap());
    let result = project_api.get_project_icon(params).await;
    assert!(result.is_err());
}
