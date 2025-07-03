use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::{ActivityId, ActivityTypeId};
use backlog_project::Activity;
use serde::Serialize;

pub type GetSpaceRecentUpdatesResponse = Vec<Activity>;

#[derive(Debug, Clone, Default, ToFormParams)]
pub struct GetSpaceRecentUpdatesParams {
    #[form(array, name = "activityTypeId")]
    pub activity_type_ids: Option<Vec<ActivityTypeId>>,

    #[form(name = "minId")]
    pub min_id: Option<ActivityId>,

    #[form(name = "maxId")]
    pub max_id: Option<ActivityId>,

    pub count: Option<u32>,
    pub order: Option<String>,
}

impl IntoRequest for GetSpaceRecentUpdatesParams {
    fn path(&self) -> String {
        "/api/v2/space/activities".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self.to_form()
    }
}
