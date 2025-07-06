use backlog_api_core::IntoRequest;
use backlog_core::identifier::UserId;
use serde::Serialize;

/// Parameters for the Get Watching Count API.
///
/// This API counts the number of watchings for a specific user,
/// with optional filtering by read status.
#[derive(Debug, Clone)]
pub struct GetWatchingCountParams {
    /// The ID of the user whose watching count to retrieve.
    pub user_id: UserId,
    /// Filter by resource already read status.
    pub resource_already_read: Option<bool>,
    /// Filter by already read status (watchings updated after menu view).
    pub already_read: Option<bool>,
}

impl GetWatchingCountParams {
    /// Creates a new instance with the specified user ID.
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
            resource_already_read: None,
            already_read: None,
        }
    }

    /// Sets the resource already read filter.
    pub fn with_resource_already_read(mut self, resource_already_read: bool) -> Self {
        self.resource_already_read = Some(resource_already_read);
        self
    }

    /// Sets the already read filter.
    pub fn with_already_read(mut self, already_read: bool) -> Self {
        self.already_read = Some(already_read);
        self
    }
}

/// Query parameters for serialization
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_already_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    already_read: Option<bool>,
}

impl IntoRequest for GetWatchingCountParams {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/watchings/count", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        QueryParams {
            resource_already_read: self.resource_already_read,
            already_read: self.already_read,
        }
    }
}

#[cfg(test)]
#[path = "get_watching_count_test.rs"]
mod tests;
