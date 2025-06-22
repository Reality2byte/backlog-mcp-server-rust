pub mod api;
pub mod models;
//pub mod tests;

// re-export domain models
pub use backlog_domain_models::{Category, IssueType, Milestone, Priority, Resolution, Status, Project};

pub use api::{
    ProjectApi,
    GetProjectDetailParams, GetProjectDetailResponse,
    GetProjectListParams, GetProjectListResponse,
    GetCategoryListParams, GetCategoryListResponse,
    GetIssueTypeListParams, GetIssueTypeListResponse,
    GetStatusListParams, GetStatusListResponse,
    GetVersionMilestoneListParams, GetVersionMilestoneListResponse,
    GetPriorityListParams, GetPriorityListResponse,
    GetResolutionListParams, GetResolutionListResponse,
    GetProjectIconParams,
};



#[cfg(feature = "writable")]
pub use api::{
    AddCategoryParams, AddCategoryResponse,
    UpdateCategoryParams, UpdateCategoryResponse,
    DeleteCategoryParams, DeleteCategoryResponse,
    AddIssueTypeParams, AddIssueTypeResponse,
    UpdateIssueTypeParams, UpdateIssueTypeResponse,
    DeleteIssueTypeParams, DeleteIssueTypeResponse,
    AddVersionParams, AddVersionResponse,
    UpdateVersionParams, UpdateVersionResponse,
    DeleteVersionParams, DeleteVersionResponse,
    AddStatusParams, AddStatusResponse,
    UpdateStatusParams, UpdateStatusResponse,
    DeleteStatusParams, DeleteStatusResponse,
    UpdateStatusOrderParams, UpdateStatusOrderResponse,
};
