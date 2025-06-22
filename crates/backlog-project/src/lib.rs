pub mod api;
pub mod models;
pub mod requests;

pub use api::ProjectApi;
pub use backlog_domain_models::{Category, IssueType, Milestone, Priority, Resolution, Status};
pub use models::Project;
pub use requests::{
    GetCategoryListParams, GetIssueTypeListParams, GetPriorityListParams, GetProjectDetailParams,
    GetProjectListParams, GetResolutionListParams, GetStatusListParams,
    GetVersionMilestoneListParams,
};

// Re-export request structs for writable features
#[cfg(feature = "writable")]
pub use requests::{
    AddCategoryParams, AddIssueTypeParams, DeleteIssueTypeParams, UpdateCategoryParams,
    UpdateIssueTypeParams,
};
