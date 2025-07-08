use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::activity::Activity;
use backlog_core::identifier::{ActivityTypeId, UserId};
use serde::Serialize;

/// Response type for getting user recent updates
pub type GetUserRecentUpdatesResponse = Vec<Activity>;

/// Parameters for getting user recent updates.
///
/// # Example
/// ```no_run
/// # use backlog_user::GetUserRecentUpdatesParams;
/// # use backlog_core::identifier::UserId;
/// let params = GetUserRecentUpdatesParams {
///     user_id: UserId::from(12345),
///     activity_type_ids: None,
///     min_id: None,
///     max_id: None,
///     count: None,
///     order: None,
/// };
/// ```
#[derive(Debug, Clone, ToFormParams)]
pub struct GetUserRecentUpdatesParams {
    /// User ID
    #[form(skip)]
    pub user_id: UserId,

    /// Filter by activity type IDs
    #[form(array, name = "activityTypeId")]
    pub activity_type_ids: Option<Vec<ActivityTypeId>>,

    /// Get activities with ID greater than this value (for pagination)
    #[form(name = "minId")]
    pub min_id: Option<i64>,

    /// Get activities with ID less than this value (for pagination)
    #[form(name = "maxId")]
    pub max_id: Option<i64>,

    /// Maximum number of results to return (default: 20, max: 100)
    pub count: Option<u32>,

    /// Sort order (asc or desc)
    pub order: Option<String>,
}

impl IntoRequest for GetUserRecentUpdatesParams {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/activities", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        self.to_form()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_recent_updates_params_path() {
        let params = GetUserRecentUpdatesParams {
            user_id: UserId::from(12345),
            activity_type_ids: None,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        };

        assert_eq!(params.path(), "/api/v2/users/12345/activities");
    }
}
