use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{AttachmentId, Identifier, IssueId, UserId},
};
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for creating a new pull request.
///
/// This struct now includes all path information needed to construct the complete request.
#[cfg(feature = "writable")]
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddPullRequestParams {
    /// The project ID or key where the repository is located.
    pub project_id_or_key: ProjectIdOrKey,

    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
    /// Pull request title (required)
    pub summary: String,

    /// Pull request description (required)
    pub description: String,

    /// Target merge branch name (required)
    pub base: String,

    /// Source branch name to be merged (required)
    pub branch: String,

    /// Related issue ID (optional)
    #[builder(default, setter(strip_option))]
    pub issue_id: Option<IssueId>,

    /// Pull request assignee user ID (optional)
    #[builder(default, setter(strip_option))]
    pub assignee_id: Option<UserId>,

    /// User IDs to be notified about the pull request (optional)
    #[builder(default, setter(strip_option))]
    pub notified_user_ids: Option<Vec<UserId>>,

    /// Attachment file IDs (optional)
    #[builder(default, setter(strip_option))]
    pub attachment_ids: Option<Vec<AttachmentId>>,
}

#[cfg(feature = "writable")]
impl AddPullRequestParams {
    /// Creates a new AddPullRequestParams with all required fields.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        summary: &str,
        description: &str,
        base: &str,
        branch: &str,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            summary: summary.to_string(),
            description: description.to_string(),
            base: base.to_string(),
            branch: branch.to_string(),
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            attachment_ids: None,
        }
    }
}

/// Convert AddPullRequestParams to form-encoded parameters for HTTP requests.
/// Handles array parameters with [] notation as required by Backlog API.
#[cfg(feature = "writable")]
impl From<&AddPullRequestParams> for Vec<(String, String)> {
    fn from(params: &AddPullRequestParams) -> Self {
        let mut seq = vec![
            ("summary".to_string(), params.summary.clone()),
            ("description".to_string(), params.description.clone()),
            ("base".to_string(), params.base.clone()),
            ("branch".to_string(), params.branch.clone()),
        ];

        // Optional issue ID
        if let Some(issue_id) = &params.issue_id {
            seq.push(("issueId".to_string(), issue_id.value().to_string()));
        }

        // Optional assignee ID
        if let Some(assignee_id) = &params.assignee_id {
            seq.push(("assigneeId".to_string(), assignee_id.value().to_string()));
        }

        // Handle notified user ID array parameters
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.value().to_string()));
            });
        }

        // Handle attachment ID array parameters
        if let Some(attachment_ids) = &params.attachment_ids {
            attachment_ids.iter().for_each(|id| {
                seq.push(("attachmentId[]".to_string(), id.value().to_string()));
            });
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddPullRequestParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;

    #[test]
    fn test_add_pull_request_params_new() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key.clone(),
            repo_id_or_name.clone(),
            "Fix bug in user authentication",
            "This PR fixes the authentication issue found in issue #123",
            "main",
            "feature/fix-auth",
        );

        assert_eq!(params.project_id_or_key, project_id_or_key);
        assert_eq!(params.repo_id_or_name, repo_id_or_name);
        assert_eq!(params.summary, "Fix bug in user authentication");
        assert_eq!(
            params.description,
            "This PR fixes the authentication issue found in issue #123"
        );
        assert_eq!(params.base, "main");
        assert_eq!(params.branch, "feature/fix-auth");
        assert!(params.issue_id.is_none());
        assert!(params.assignee_id.is_none());
        assert!(params.notified_user_ids.is_none());
        assert!(params.attachment_ids.is_none());
    }

    #[test]
    fn test_add_pull_request_params_builder_minimal() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key.clone())
            .repo_id_or_name(repo_id_or_name.clone())
            .summary("Test PR".to_string())
            .description("Test description".to_string())
            .base("main".to_string())
            .branch("feature/test".to_string())
            .build()
            .unwrap();

        assert_eq!(params.project_id_or_key, project_id_or_key);
        assert_eq!(params.repo_id_or_name, repo_id_or_name);
        assert_eq!(params.summary, "Test PR");
        assert_eq!(params.description, "Test description");
        assert_eq!(params.base, "main");
        assert_eq!(params.branch, "feature/test");
    }

    #[test]
    fn test_add_pull_request_params_builder_full() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key.clone())
            .repo_id_or_name(repo_id_or_name.clone())
            .summary("Feature PR".to_string())
            .description("Implementing new feature".to_string())
            .base("develop".to_string())
            .branch("feature/new-feature".to_string())
            .issue_id(IssueId::new(123))
            .assignee_id(UserId::new(456))
            .notified_user_ids(vec![UserId::new(789), UserId::new(101112)])
            .attachment_ids(vec![AttachmentId::new(111), AttachmentId::new(222)])
            .build()
            .unwrap();

        assert_eq!(params.project_id_or_key, project_id_or_key);
        assert_eq!(params.repo_id_or_name, repo_id_or_name);
        assert_eq!(params.summary, "Feature PR");
        assert_eq!(params.issue_id, Some(IssueId::new(123)));
        assert_eq!(params.assignee_id, Some(UserId::new(456)));
        assert_eq!(
            params.notified_user_ids,
            Some(vec![UserId::new(789), UserId::new(101112)])
        );
        assert_eq!(
            params.attachment_ids,
            Some(vec![AttachmentId::new(111), AttachmentId::new(222)])
        );
    }

    #[test]
    fn test_add_pull_request_params_builder_missing_required() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let result = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .summary("Test".to_string())
            // Missing description, base, branch
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_add_pull_request_params_to_form_minimal() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Test PR",
            "Test description",
            "main",
            "feature/test",
        );

        let form_params: Vec<(String, String)> = (&params).into();

        assert_eq!(form_params.len(), 4);
        assert!(form_params.contains(&("summary".to_string(), "Test PR".to_string())));
        assert!(form_params.contains(&("description".to_string(), "Test description".to_string())));
        assert!(form_params.contains(&("base".to_string(), "main".to_string())));
        assert!(form_params.contains(&("branch".to_string(), "feature/test".to_string())));
    }

    #[test]
    fn test_add_pull_request_params_to_form_with_optionals() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .summary("Full PR".to_string())
            .description("Complete description".to_string())
            .base("main".to_string())
            .branch("feature/full".to_string())
            .issue_id(IssueId::new(123))
            .assignee_id(UserId::new(456))
            .notified_user_ids(vec![UserId::new(789), UserId::new(101112)])
            .attachment_ids(vec![AttachmentId::new(111)])
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // 4 required + 2 single optionals + 2 notified users + 1 attachment = 9 total
        assert_eq!(form_params.len(), 9);

        // Check required params
        assert!(form_params.contains(&("summary".to_string(), "Full PR".to_string())));
        assert!(form_params.contains(&(
            "description".to_string(),
            "Complete description".to_string()
        )));
        assert!(form_params.contains(&("base".to_string(), "main".to_string())));
        assert!(form_params.contains(&("branch".to_string(), "feature/full".to_string())));

        // Check optional single params
        assert!(form_params.contains(&("issueId".to_string(), "123".to_string())));
        assert!(form_params.contains(&("assigneeId".to_string(), "456".to_string())));

        // Check array params
        assert!(form_params.contains(&("notifiedUserId[]".to_string(), "789".to_string())));
        assert!(form_params.contains(&("notifiedUserId[]".to_string(), "101112".to_string())));
        assert!(form_params.contains(&("attachmentId[]".to_string(), "111".to_string())));
    }

    #[test]
    fn test_add_pull_request_params_to_form_empty_arrays() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .summary("Test PR".to_string())
            .description("Test description".to_string())
            .base("main".to_string())
            .branch("feature/test".to_string())
            .notified_user_ids(vec![])
            .attachment_ids(vec![])
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // Should only have the 4 required parameters (empty arrays should not add anything)
        assert_eq!(form_params.len(), 4);
        assert!(form_params.contains(&("summary".to_string(), "Test PR".to_string())));
        assert!(form_params.contains(&("description".to_string(), "Test description".to_string())));
        assert!(form_params.contains(&("base".to_string(), "main".to_string())));
        assert!(form_params.contains(&("branch".to_string(), "feature/test".to_string())));
    }
}
