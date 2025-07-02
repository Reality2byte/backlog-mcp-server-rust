use backlog_api_core::IntoRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Response type for getting licence information
pub type GetLicenceResponse = Licence;

/// Licence information model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Licence {
    pub active: bool,
    pub attachment_limit: i64,
    pub attachment_limit_per_file: i64,
    pub attachment_num_limit: i64,
    pub attribute: bool,
    pub attribute_limit: i64,
    pub burndown: bool,
    pub comment_limit: i64,
    pub component_limit: i64,
    pub file_sharing: bool,
    pub gantt: bool,
    pub git: bool,
    pub issue_limit: i64,
    pub licence_type_id: i64,
    pub limit_date: Option<DateTime<Utc>>,
    pub nulab_account: bool,
    pub parent_child_issue: bool,
    pub post_issue_by_mail: bool,
    pub project_limit: i64,
    pub pull_request_attachment_limit_per_file: i64,
    pub pull_request_attachment_num_limit: i64,
    pub remote_address: bool,
    pub remote_address_limit: i64,
    pub started_on: Option<DateTime<Utc>>,
    pub storage_limit: i64,
    pub subversion: bool,
    pub subversion_external: bool,
    pub user_limit: i64,
    pub version_limit: i64,
    pub wiki_attachment: bool,
    pub wiki_attachment_limit_per_file: i64,
    pub wiki_attachment_num_limit: i64,
}

/// Parameters for getting licence information.
///
/// Corresponds to `GET /api/v2/space/licence`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLicenceParams;

impl GetLicenceParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetLicenceParams {
    fn path(&self) -> String {
        "/api/v2/space/licence".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
