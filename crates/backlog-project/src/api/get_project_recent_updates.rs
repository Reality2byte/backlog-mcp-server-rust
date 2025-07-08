use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::{
    ProjectIdOrKey,
    identifier::{ActivityId, ActivityTypeId},
};
use serde::Serialize;

use backlog_core::activity::Activity;

pub type GetProjectRecentUpdatesResponse = Vec<Activity>;

#[derive(Debug, Clone, ToFormParams)]
pub struct GetProjectRecentUpdatesParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    #[form(array, name = "activityTypeId")]
    pub activity_type_ids: Option<Vec<ActivityTypeId>>,

    #[form(name = "minId")]
    pub min_id: Option<ActivityId>,

    #[form(name = "maxId")]
    pub max_id: Option<ActivityId>,

    pub count: Option<u32>,
    pub order: Option<String>,
}

impl GetProjectRecentUpdatesParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            activity_type_ids: None,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        }
    }
}

impl IntoRequest for GetProjectRecentUpdatesParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/activities", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        self.to_form()
    }
}
