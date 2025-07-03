use crate::models::Activity;
use backlog_api_core::IntoRequest;
use backlog_core::identifier::ActivityId;
use serde::Serialize;

pub type GetActivityResponse = Activity;

#[derive(Debug, Clone)]
pub struct GetActivityParams {
    pub activity_id: ActivityId,
}

impl IntoRequest for GetActivityParams {
    fn path(&self) -> String {
        format!("/api/v2/activities/{}", self.activity_id)
    }

    fn to_query(&self) -> impl Serialize {
        // This API doesn't use query parameters
        std::collections::HashMap::<String, String>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_activity_params_path() {
        let params = GetActivityParams {
            activity_id: ActivityId::from(12345),
        };

        assert_eq!(params.path(), "/api/v2/activities/12345");
    }
}
