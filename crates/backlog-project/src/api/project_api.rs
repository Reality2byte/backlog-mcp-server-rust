use backlog_api_core::Result;
#[cfg(feature = "writable")]
use backlog_domain_models::Milestone;
use client::Client;

use crate::api::{
    GetCategoryListParams, GetCategoryListResponse, GetCustomFieldListParams,
    GetCustomFieldListResponse, GetIssueTypeListParams, GetIssueTypeListResponse,
    GetMilestoneListParams, GetMilestoneListResponse, GetPriorityListParams,
    GetPriorityListResponse, GetProjectDetailParams, GetProjectDetailResponse,
    GetProjectIconParams, GetProjectRecentUpdatesParams, GetProjectRecentUpdatesResponse,
    GetProjectUserListParams, GetProjectUserListResponse, GetResolutionListParams,
    GetResolutionListResponse, GetStatusListParams, GetStatusListResponse,
    get_project_list::{GetProjectListParams, GetProjectListResponse},
};
#[cfg(feature = "writable")]
use crate::{
    AddCategoryParams, AddCustomFieldParams, AddIssueTypeParams, AddMilestoneParams,
    AddStatusParams, DeleteCategoryParams, DeleteIssueTypeParams, DeleteVersionParams,
    UpdateCategoryParams, UpdateIssueTypeParams, UpdateVersionParams,
    api::{
        AddCategoryResponse, AddCustomFieldResponse, AddIssueTypeResponse, AddStatusResponse,
        DeleteCategoryResponse, DeleteIssueTypeResponse, DeleteStatusParams, DeleteStatusResponse,
        DeleteVersionResponse, UpdateCategoryResponse, UpdateCustomFieldParams,
        UpdateCustomFieldResponse, UpdateIssueTypeResponse, UpdateStatusOrderParams,
        UpdateStatusOrderResponse, UpdateStatusParams, UpdateStatusResponse, UpdateVersionResponse,
    },
};

pub struct ProjectApi(Client);

// You should use `XxxxResponse` structs for the response types instead of models directly.
impl ProjectApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the list of projects.
    /// Corresponds to `GET /api/v2/projects`.
    pub async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<GetProjectListResponse> {
        self.0.execute(params).await
    }

    /// Gets a project by its ID or key.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey`.
    pub async fn get_project(
        &self,
        params: GetProjectDetailParams,
    ) -> Result<GetProjectDetailResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of statuses for a project.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/statuses`.
    pub async fn get_status_list(
        &self,
        params: GetStatusListParams,
    ) -> Result<GetStatusListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of issue types for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/issueTypes`.
    pub async fn get_issue_type_list(
        &self,
        params: GetIssueTypeListParams,
    ) -> Result<GetIssueTypeListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of version milestones for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/versions`.
    pub async fn get_version_milestone_list(
        &self,
        params: GetMilestoneListParams,
    ) -> Result<GetMilestoneListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of categories for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/categories`.
    pub async fn get_category_list(
        &self,
        params: GetCategoryListParams,
    ) -> Result<GetCategoryListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of priorities.
    ///
    /// Corresponds to `GET /api/v2/priorities`.
    pub async fn get_priority_list(&self) -> Result<GetPriorityListResponse> {
        self.0.execute(GetPriorityListParams).await
    }

    /// Gets the list of resolutions.
    ///
    /// Corresponds to `GET /api/v2/resolutions`.
    pub async fn get_resolution_list(&self) -> Result<GetResolutionListResponse> {
        self.0.execute(GetResolutionListParams).await
    }

    /// Gets the project icon image data.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/image`.
    pub async fn get_project_icon(&self, params: GetProjectIconParams) -> Result<Vec<u8>> {
        let downloaded_file = self.0.download_file(params).await?;
        Ok(downloaded_file.bytes.to_vec())
    }

    /// Gets the list of project members.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/users`.
    pub async fn get_project_user_list(
        &self,
        params: GetProjectUserListParams,
    ) -> Result<GetProjectUserListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of custom fields for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/customFields`.
    pub async fn get_custom_field_list(
        &self,
        params: GetCustomFieldListParams,
    ) -> Result<GetCustomFieldListResponse> {
        self.0.execute(params).await
    }

    /// Gets recent updates in the project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/activities`.
    pub async fn get_project_recent_updates(
        &self,
        params: GetProjectRecentUpdatesParams,
    ) -> Result<GetProjectRecentUpdatesResponse> {
        self.0.execute(params).await
    }

    /// Adds a category to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/categories`.
    #[cfg(feature = "writable")]
    pub async fn add_category(&self, params: AddCategoryParams) -> Result<AddCategoryResponse> {
        self.0.execute(params).await
    }

    /// Updates a category in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_category(
        &self,
        params: UpdateCategoryParams,
    ) -> Result<UpdateCategoryResponse> {
        self.0.execute(params).await
    }

    /// Deletes a category from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_category(
        &self,
        params: DeleteCategoryParams,
    ) -> Result<DeleteCategoryResponse> {
        self.0.execute(params).await
    }

    /// Adds an issue type to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/issueTypes`.
    #[cfg(feature = "writable")]
    pub async fn add_issue_type(&self, params: AddIssueTypeParams) -> Result<AddIssueTypeResponse> {
        self.0.execute(params).await
    }

    /// Deletes an issue type from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/issueTypes/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_issue_type(
        &self,
        params: DeleteIssueTypeParams,
    ) -> Result<DeleteIssueTypeResponse> {
        self.0.execute(params).await
    }

    /// Updates an issue type in a project.
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/issueTypes/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_issue_type(
        &self,
        params: UpdateIssueTypeParams,
    ) -> Result<UpdateIssueTypeResponse> {
        self.0.execute(params).await
    }

    /// Adds a version/milestone to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/versions`.
    #[cfg(feature = "writable")]
    pub async fn add_version(&self, params: AddMilestoneParams) -> Result<Milestone> {
        self.0.execute(params).await
    }

    /// Updates a version/milestone in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/versions/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_version(
        &self,
        params: UpdateVersionParams,
    ) -> Result<UpdateVersionResponse> {
        self.0.execute(params).await
    }

    /// Deletes a version/milestone from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/versions/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_version(
        &self,
        params: DeleteVersionParams,
    ) -> Result<DeleteVersionResponse> {
        self.0.execute(params).await
    }

    /// Adds a status to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/statuses`.
    #[cfg(feature = "writable")]
    pub async fn add_status(&self, params: AddStatusParams) -> Result<AddStatusResponse> {
        self.0.execute(params).await
    }

    /// Updates a status in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/statuses/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_status(&self, params: UpdateStatusParams) -> Result<UpdateStatusResponse> {
        self.0.execute(params).await
    }

    /// Deletes a status from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/statuses/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_status(&self, params: DeleteStatusParams) -> Result<DeleteStatusResponse> {
        self.0.execute(params).await
    }

    /// Updates the display order of statuses in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/statuses/updateDisplayOrder`.
    #[cfg(feature = "writable")]
    pub async fn update_status_order(
        &self,
        params: UpdateStatusOrderParams,
    ) -> Result<UpdateStatusOrderResponse> {
        self.0.execute(params).await
    }

    /// Updates a custom field in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/customFields/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_custom_field(
        &self,
        params: UpdateCustomFieldParams,
    ) -> Result<UpdateCustomFieldResponse> {
        self.0.execute(params).await
    }

    /// Adds a custom field to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/customFields`.
    #[cfg(feature = "writable")]
    pub async fn add_custom_field(
        &self,
        params: AddCustomFieldParams,
    ) -> Result<AddCustomFieldResponse> {
        self.0.execute(params).await
    }
}
