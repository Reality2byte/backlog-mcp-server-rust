pub mod api;
pub mod models;
//pub mod tests;

// re-export domain models
pub use backlog_domain_models::{
    Category, IssueType, Milestone, Priority, Project, Resolution, Status,
};

// re-export activity models
pub use models::activity::{Activity, ActivityContent, Content, ReasonId, TypeId};

pub use api::{
    GetCategoryListParams, GetCategoryListResponse, GetCustomFieldListParams,
    GetCustomFieldListResponse, GetIssueTypeListParams, GetIssueTypeListResponse,
    GetMilestoneListParams, GetMilestoneListResponse, GetPriorityListParams,
    GetPriorityListResponse, GetProjectDetailParams, GetProjectDetailResponse,
    GetProjectIconParams, GetProjectListParams, GetProjectListResponse,
    GetProjectRecentUpdatesParams, GetProjectRecentUpdatesResponse, GetProjectUserListParams,
    GetProjectUserListResponse, GetResolutionListParams, GetResolutionListResponse,
    GetStatusListParams, GetStatusListResponse, ProjectApi,
};

#[cfg(feature = "writable")]
pub use api::{
    AddCategoryParams, AddCategoryResponse, AddCustomFieldParams, AddCustomFieldResponse,
    AddIssueTypeParams, AddIssueTypeResponse, AddMilestoneParams, AddMilestoneResponse,
    AddStatusParams, AddStatusResponse, DeleteCategoryParams, DeleteCategoryResponse,
    DeleteCustomFieldParams, DeleteCustomFieldResponse, DeleteIssueTypeParams,
    DeleteIssueTypeResponse, DeleteStatusParams, DeleteStatusResponse, DeleteVersionParams,
    DeleteVersionResponse, UpdateCategoryParams, UpdateCategoryResponse, UpdateCustomFieldParams,
    UpdateCustomFieldResponse, UpdateIssueTypeParams, UpdateIssueTypeResponse,
    UpdateStatusOrderParams, UpdateStatusOrderResponse, UpdateStatusParams, UpdateStatusResponse,
    UpdateVersionParams, UpdateVersionResponse,
};
