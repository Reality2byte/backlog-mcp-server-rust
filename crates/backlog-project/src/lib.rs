pub mod api;
pub mod models;
//pub mod tests;

// re-export domain models
pub use backlog_domain_models::{
    Category, IssueType, Milestone, Priority, Project, Resolution, Status,
};

pub use api::{
    GetCategoryListParams, GetCategoryListResponse, GetIssueTypeListParams,
    GetIssueTypeListResponse, GetMilestoneListParams, GetMilestoneListResponse,
    GetPriorityListParams, GetPriorityListResponse, GetProjectDetailParams,
    GetProjectDetailResponse, GetProjectIconParams, GetProjectListParams, GetProjectListResponse,
    GetResolutionListParams, GetResolutionListResponse, GetStatusListParams, GetStatusListResponse,
    ProjectApi,
};

#[cfg(feature = "writable")]
pub use api::{
    AddCategoryParams, AddCategoryResponse, AddIssueTypeParams, AddIssueTypeResponse,
    AddMilestoneParams, AddMilestoneResponse, AddStatusParams, AddStatusResponse,
    DeleteCategoryParams, DeleteCategoryResponse, DeleteIssueTypeParams, DeleteIssueTypeResponse,
    DeleteStatusParams, DeleteStatusResponse, DeleteVersionParams, DeleteVersionResponse,
    UpdateCategoryParams, UpdateCategoryResponse, UpdateIssueTypeParams, UpdateIssueTypeResponse,
    UpdateStatusOrderParams, UpdateStatusOrderResponse, UpdateStatusParams, UpdateStatusResponse,
    UpdateVersionParams, UpdateVersionResponse,
};
